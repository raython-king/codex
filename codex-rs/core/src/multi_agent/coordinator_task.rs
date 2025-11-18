//! Coordinator task for orchestrating multiple agents
//!
//! This provides a foundational implementation of multi-agent coordination.
//! Full session integration will be completed in future iterations.

use super::agent_pool::{AgentPool, AgentPoolConfig};
use super::task_distributor::{TaskDistributionStrategy, TaskDistributor};
use super::types::{AgentCapability, AgentSpec, DelegatedTask};
use crate::codex::TurnContext;
use crate::error::CodexErr;
use crate::state::TaskKind;
use crate::tasks::{SessionTask, SessionTaskContext};
use async_trait::async_trait;
use codex_protocol::user_input::UserInput;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

/// Configuration for the coordinator task
#[derive(Debug, Clone)]
pub struct CoordinatorConfig {
    /// Agent pool configuration
    pub pool_config: AgentPoolConfig,
    /// Task distribution strategy
    pub distribution_strategy: TaskDistributionStrategy,
    /// Agent specifications to create
    pub agent_specs: Vec<AgentSpec>,
    /// Enable detailed logging
    pub verbose: bool,
}

impl Default for CoordinatorConfig {
    fn default() -> Self {
        Self {
            pool_config: AgentPoolConfig::default(),
            distribution_strategy: TaskDistributionStrategy::LeastLoaded,
            agent_specs: Vec::new(),
            verbose: false,
        }
    }
}

/// A task that coordinates multiple agents to work together
pub struct CoordinatorTask {
    /// Configuration
    config: CoordinatorConfig,
    /// Agent pool
    agent_pool: Arc<RwLock<Option<Arc<AgentPool>>>>,
    /// Task distributor
    task_distributor: Arc<RwLock<Option<Arc<TaskDistributor>>>>,
    /// Next task ID
    next_task_id: Arc<RwLock<usize>>,
}

impl CoordinatorTask {
    /// Create a new coordinator task
    pub fn new(config: CoordinatorConfig) -> Self {
        Self {
            config,
            agent_pool: Arc::new(RwLock::new(None)),
            task_distributor: Arc::new(RwLock::new(None)),
            next_task_id: Arc::new(RwLock::new(0)),
        }
    }

    /// Initialize the agent pool and agents
    async fn initialize_agents(&self) -> Result<Arc<AgentPool>, CodexErr> {
        // Create agent pool
        let pool = Arc::new(AgentPool::new(self.config.pool_config.clone()));

        // Add configured agents
        for spec in &self.config.agent_specs {
            pool.add_agent(spec.clone()).await?;
        }

        Ok(pool)
    }

    /// Infer required capabilities from task description
    fn infer_capabilities(&self, description: &str) -> std::collections::HashSet<AgentCapability> {
        let mut capabilities = std::collections::HashSet::new();
        let desc_lower = description.to_lowercase();

        // Simple keyword matching to infer capabilities
        if desc_lower.contains("code")
            || desc_lower.contains("implement")
            || desc_lower.contains("write")
        {
            capabilities.insert(AgentCapability::CodeGeneration);
        }

        if desc_lower.contains("review") || desc_lower.contains("check") {
            capabilities.insert(AgentCapability::CodeReview);
        }

        if desc_lower.contains("test") || desc_lower.contains("verify") {
            capabilities.insert(AgentCapability::Testing);
        }

        if desc_lower.contains("document") || desc_lower.contains("explain") {
            capabilities.insert(AgentCapability::Documentation);
        }

        if desc_lower.contains("debug") || desc_lower.contains("fix") {
            capabilities.insert(AgentCapability::Debugging);
        }

        if desc_lower.contains("design") || desc_lower.contains("architecture") {
            capabilities.insert(AgentCapability::Architecture);
        }

        if desc_lower.contains("research") || desc_lower.contains("find") {
            capabilities.insert(AgentCapability::Research);
        }

        if desc_lower.contains("shell") || desc_lower.contains("command") {
            capabilities.insert(AgentCapability::ShellExecution);
        }

        // If no specific capabilities detected, mark as general purpose
        if capabilities.is_empty() {
            capabilities.insert(AgentCapability::CodeGeneration);
        }

        capabilities
    }

    /// Plan task delegation based on user input
    async fn plan_delegation(&self, input: &[UserInput]) -> Result<Vec<DelegatedTask>, CodexErr> {
        let message = input
            .iter()
            .filter_map(|i| match i {
                UserInput::Text { text } => Some(text.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n");

        let task_id = {
            let mut next_id = self.next_task_id.write().await;
            let id = format!("task-{next_id}");
            *next_id += 1;
            id
        };

        // Infer capabilities needed
        let required_capabilities = self.infer_capabilities(&message);

        // Get distributor
        let distributor = {
            let dist_opt = self.task_distributor.read().await;
            dist_opt
                .as_ref()
                .ok_or_else(|| CodexErr::Fatal("Task distributor not initialized".to_string()))?
                .clone()
        };

        // Create task
        let mut task = DelegatedTask::new(task_id, "", message);
        task.required_capabilities = required_capabilities;

        // Find appropriate agent
        let agent_id = distributor.distribute_task(&task).await?;
        task.agent_id = agent_id;

        Ok(vec![task])
    }
}

#[async_trait]
impl SessionTask for CoordinatorTask {
    fn kind(&self) -> TaskKind {
        TaskKind::Regular
    }

    async fn run(
        self: Arc<Self>,
        _session: Arc<SessionTaskContext>,
        _ctx: Arc<TurnContext>,
        input: Vec<UserInput>,
        _cancellation_token: CancellationToken,
    ) -> Option<String> {
        let start_time = Instant::now();

        // Initialize agents
        let pool = match self.initialize_agents().await {
            Ok(p) => p,
            Err(e) => {
                return Some(format!("Error: Failed to initialize agents: {e}"));
            }
        };

        {
            let mut pool_opt = self.agent_pool.write().await;
            *pool_opt = Some(pool.clone());
        }

        // Initialize task distributor
        let distributor = Arc::new(TaskDistributor::new(
            pool.clone(),
            self.config.distribution_strategy.clone(),
        ));

        {
            let mut dist_opt = self.task_distributor.write().await;
            *dist_opt = Some(distributor.clone());
        }

        // Plan task delegation
        let tasks = match self.plan_delegation(&input).await {
            Ok(t) => t,
            Err(e) => {
                return Some(format!("Error: Failed to plan delegation: {e}"));
            }
        };

        let elapsed = start_time.elapsed();

        // Report coordination plan
        let mut report = format!(
            "Multi-agent coordination plan created in {:.2}s\n\n",
            elapsed.as_secs_f64()
        );

        report.push_str("Planned task delegation:\n");
        for task in &tasks {
            report.push_str(&format!(
                "- Task: {}\n  Agent: {}\n  Capabilities: {:?}\n\n",
                task.description, task.agent_id, task.required_capabilities
            ));
        }

        let stats = pool.get_stats().await;
        report.push_str(&format!(
            "Agent pool status:\n- Total agents: {}\n- Active tasks: {}\n- Available capacity: {}\n",
            stats.total_agents, stats.total_active_tasks, stats.available_capacity
        ));

        Some(report)
    }

    async fn abort(&self, _session: Arc<SessionTaskContext>, _ctx: Arc<TurnContext>) {
        // Cleanup handled automatically
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_inference() {
        let config = CoordinatorConfig::default();
        let task = CoordinatorTask::new(config);

        let caps = task.infer_capabilities("implement a new feature");
        assert!(caps.contains(&AgentCapability::CodeGeneration));

        let caps = task.infer_capabilities("review this code");
        assert!(caps.contains(&AgentCapability::CodeReview));

        let caps = task.infer_capabilities("write tests for the module");
        assert!(caps.contains(&AgentCapability::Testing));
    }

    #[tokio::test]
    async fn test_coordinator_creation() {
        let config = CoordinatorConfig {
            agent_specs: vec![
                AgentSpec::new("agent-1", AgentRole::GeneralPurpose)
                    .with_capability(AgentCapability::CodeGeneration),
            ],
            ..Default::default()
        };

        let coordinator = CoordinatorTask::new(config);
        let pool = coordinator.initialize_agents().await.unwrap();
        let stats = pool.get_stats().await;

        assert_eq!(stats.total_agents, 1);
    }
}
