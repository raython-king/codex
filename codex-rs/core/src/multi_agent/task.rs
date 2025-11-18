//! Multi-agent task implementation

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

use codex_protocol::user_input::UserInput;

use crate::codex::{Session, TurnContext};
use crate::state::TaskKind;
use crate::tasks::{SessionTask, SessionTaskContext};

use super::config::MultiAgentConfig;
use super::coordinator::MultiAgentCoordinator;
use super::task_decomposer::DecompositionContext;

/// Multi-agent collaborative development task
pub struct MultiAgentTask {
    config: MultiAgentConfig,
}

impl MultiAgentTask {
    pub fn new(config: MultiAgentConfig) -> Self {
        Self { config }
    }

    /// Create a multi-agent task with default configuration
    pub fn with_defaults() -> Self {
        Self {
            config: MultiAgentConfig::default(),
        }
    }

    /// Build decomposition context from turn context and session
    async fn build_decomposition_context(
        &self,
        _session: &Session,
        ctx: &TurnContext,
    ) -> DecompositionContext {
        // Get current working directory
        let cwd = ctx.cwd.clone();

        // TODO: Analyze the codebase to identify relevant files
        // For now, use empty list
        let relevant_files = Vec::new();

        // TODO: Extract project metadata
        let project_metadata = HashMap::new();

        DecompositionContext {
            relevant_files,
            cwd,
            include_tests: true, // Default to including tests
            project_metadata,
        }
    }

    /// Format the final message from aggregated results
    fn format_result_message(
        &self,
        result: &super::result_aggregator::AggregatedResult,
    ) -> String {
        let mut message = String::new();

        message.push_str("## Multi-Agent Task Completed\n\n");

        if result.success {
            message.push_str("✅ All agents completed successfully\n\n");
        } else {
            message.push_str("⚠️ Some agents encountered issues\n\n");
        }

        message.push_str(&format!("- **Agents**: {}\n", result.agent_count));
        message.push_str(&format!(
            "- **Execution Time**: {:.2}s\n",
            result.total_execution_time_ms as f64 / 1000.0
        ));
        message.push_str(&format!(
            "- **Files Changed**: {}\n",
            result.final_changes.len()
        ));

        if result.conflict_count > 0 {
            message.push_str(&format!(
                "- **Conflicts Resolved**: {}\n",
                result.conflict_count
            ));
        }

        if !result.messages.is_empty() {
            message.push_str("\n### Agent Messages\n\n");
            for (idx, msg) in result.messages.iter().enumerate() {
                message.push_str(&format!("{}. {}\n", idx + 1, msg));
            }
        }

        if !result.errors.is_empty() {
            message.push_str("\n### Errors\n\n");
            for error in &result.errors {
                message.push_str(&format!("- {}\n", error));
            }
        }

        if !result.final_changes.is_empty() {
            message.push_str("\n### File Changes\n\n");
            for change in &result.final_changes {
                message.push_str(&format!(
                    "- {:?}: {}\n",
                    change.change_type,
                    change.path.display()
                ));
            }
        }

        message
    }
}

#[async_trait]
impl SessionTask for MultiAgentTask {
    fn kind(&self) -> TaskKind {
        TaskKind::MultiAgent
    }

    async fn run(
        self: Arc<Self>,
        session: Arc<SessionTaskContext>,
        ctx: Arc<TurnContext>,
        input: Vec<UserInput>,
        cancellation_token: CancellationToken,
    ) -> Option<String> {
        info!("Starting multi-agent task");

        // Build decomposition context
        let decomp_context = self
            .build_decomposition_context(session.clone_session().as_ref(), ctx.as_ref())
            .await;

        // Get base configuration from turn context
        let base_config = ctx.client.config().as_ref().clone();

        // Create coordinator
        let coordinator = MultiAgentCoordinator::new(
            self.config.clone(),
            base_config,
            session.auth_manager(),
            session.clone_session(),
            Arc::clone(&ctx),
        );

        // Execute the multi-agent task
        match coordinator
            .execute(input, decomp_context, cancellation_token)
            .await
        {
            Ok(result) => {
                let message = self.format_result_message(&result);
                info!("Multi-agent task completed successfully");
                Some(message)
            }
            Err(e) => {
                warn!("Multi-agent task failed: {}", e);
                Some(format!("Multi-agent task failed: {}", e))
            }
        }
    }

    async fn abort(&self, _session: Arc<SessionTaskContext>, _ctx: Arc<TurnContext>) {
        info!("Multi-agent task aborted");
        // Cleanup will be handled by cancellation tokens
    }
}
