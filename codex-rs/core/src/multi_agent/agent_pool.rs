//! Agent pool management for multi-agent execution

use std::sync::Arc;
use std::time::Instant;

use async_channel::Receiver;
use codex_protocol::user_input::UserInput;
use tokio::sync::Semaphore;
use tokio_util::sync::CancellationToken;

use crate::codex::{Session, TurnContext};
use crate::codex_delegate::run_codex_conversation_one_shot;
use crate::config::Config;
use crate::error::CodexErr;
use crate::AuthManager;

use super::result_aggregator::AgentResult;
use super::task_decomposer::SubTask;

/// Handle to a running agent
pub struct AgentHandle {
    /// Unique ID for this agent
    pub agent_id: String,

    /// The sub-task this agent is working on
    pub task_id: String,

    /// Channel to receive the final result
    result_rx: Receiver<AgentResult>,

    /// Cancellation token for this agent
    cancel_token: CancellationToken,

    /// Start time
    start_time: Instant,
}

impl AgentHandle {
    /// Wait for the agent to complete and return its result
    pub async fn await_result(self) -> Result<AgentResult, CodexErr> {
        match self.result_rx.recv().await {
            Ok(result) => Ok(result),
            Err(_) => Err(CodexErr::Fatal(format!(
                "Agent {} failed to send result",
                self.agent_id
            ))),
        }
    }

    /// Cancel this agent's execution
    pub fn cancel(&self) {
        self.cancel_token.cancel();
    }

    /// Get elapsed time since agent started
    pub fn elapsed(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }
}

/// Pool of agents for concurrent task execution
pub struct AgentPool {
    /// Maximum concurrent agents
    semaphore: Arc<Semaphore>,

    /// Base configuration for spawning agents
    base_config: Config,

    /// Authentication manager
    auth_manager: Arc<AuthManager>,

    /// Parent session for approval delegation
    parent_session: Arc<Session>,

    /// Parent turn context
    parent_ctx: Arc<TurnContext>,

    /// Next agent ID
    next_agent_id: std::sync::atomic::AtomicU64,
}

impl AgentPool {
    pub fn new(
        max_concurrent: usize,
        base_config: Config,
        auth_manager: Arc<AuthManager>,
        parent_session: Arc<Session>,
        parent_ctx: Arc<TurnContext>,
    ) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            base_config,
            auth_manager,
            parent_session,
            parent_ctx,
            next_agent_id: std::sync::atomic::AtomicU64::new(0),
        }
    }

    /// Spawn an agent to work on a sub-task
    pub async fn spawn_agent(
        &self,
        sub_task: SubTask,
        cancel_token: CancellationToken,
    ) -> Result<AgentHandle, CodexErr> {
        // Acquire a permit from the semaphore (blocks if at max capacity)
        let permit = self
            .semaphore
            .clone()
            .acquire_owned()
            .await
            .map_err(|e| CodexErr::Fatal(format!("Failed to acquire agent permit: {}", e)))?;

        let agent_id = self
            .next_agent_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let agent_id_str = format!("agent_{}", agent_id);

        let (result_tx, result_rx) = async_channel::bounded(1);

        // Clone necessary context
        let config = self.base_config.clone();
        let auth_manager = Arc::clone(&self.auth_manager);
        let parent_session = Arc::clone(&self.parent_session);
        let parent_ctx = Arc::clone(&self.parent_ctx);
        let agent_id_clone = agent_id_str.clone();
        let task_id = sub_task.id.clone();
        let cancel_token_for_spawn = cancel_token.clone();

        // Spawn the agent in a background task
        tokio::spawn(async move {
            let start_time = Instant::now();

            // Create custom instructions for this agent
            let mut agent_config = config;
            agent_config.user_instructions = Some(format!(
                "You are working on a specific sub-task as part of a multi-agent collaboration.\n\
                 Your task: {}\n\
                 Focus only on this task. Other agents are handling other parts of the project.\n\
                 Target files: {:?}",
                sub_task.description, sub_task.target_files
            ));

            // Prepare input
            let input = vec![UserInput::Text {
                text: sub_task.prompt,
            }];

            // Run the agent
            let result = match run_codex_conversation_one_shot(
                agent_config,
                auth_manager,
                input,
                parent_session,
                parent_ctx,
                cancel_token_for_spawn.clone(),
                None,
            )
            .await
            {
                Ok(codex_io) => {
                    // Collect all events from the agent
                    let mut errors = Vec::new();
                    let mut final_message = None;

                    while let Ok(event) = codex_io.next_event().await {
                        match event.msg {
                            codex_protocol::protocol::EventMsg::TaskComplete(complete) => {
                                final_message = complete.last_agent_message;
                                break;
                            }
                            codex_protocol::protocol::EventMsg::TurnAborted(_) => {
                                errors.push("Agent execution was aborted".to_string());
                                break;
                            }
                            _ => {
                                // Process other events if needed
                            }
                        }
                    }

                    AgentResult {
                        agent_id: agent_id_clone,
                        task_id: task_id.clone(),
                        success: errors.is_empty(),
                        message: final_message,
                        modified_files: vec![], // TODO: Track file changes
                        errors,
                        execution_time_ms: start_time.elapsed().as_millis() as u64,
                    }
                }
                Err(e) => AgentResult {
                    agent_id: agent_id_clone,
                    task_id: task_id.clone(),
                    success: false,
                    message: None,
                    modified_files: vec![],
                    errors: vec![format!("Failed to start agent: {}", e)],
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                },
            };

            // Send result
            let _ = result_tx.send(result).await;

            // Release the permit
            drop(permit);
        });

        Ok(AgentHandle {
            agent_id: agent_id_str,
            task_id: sub_task.id,
            result_rx,
            cancel_token: cancel_token.child_token(),
            start_time: Instant::now(),
        })
    }

    /// Get the number of available agent slots
    pub fn available_slots(&self) -> usize {
        self.semaphore.available_permits()
    }
}
