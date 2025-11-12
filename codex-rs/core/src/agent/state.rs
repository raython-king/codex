//! Per-agent state management.

use super::{AgentConfig, AgentId};
use std::sync::Mutex;
use std::time::SystemTime;

/// Record of a completed task for an agent.
#[derive(Debug, Clone)]
pub struct TaskRecord {
    /// Submission ID of the task.
    pub sub_id: String,

    /// When the task started.
    pub started_at: SystemTime,

    /// When the task completed (if completed).
    pub completed_at: Option<SystemTime>,

    /// Whether the task succeeded.
    pub success: bool,
}

/// Per-agent state within a session.
///
/// Each agent maintains its own task queue, approval decisions, and execution history.
pub struct AgentState {
    /// Configuration for this agent.
    pub config: AgentConfig,

    /// Number of currently active tasks.
    /// Protected by mutex to allow concurrent access.
    active_task_count: Mutex<usize>,

    /// Task execution history.
    /// Protected by mutex for thread-safe updates.
    task_history: Mutex<Vec<TaskRecord>>,
}

impl AgentState {
    /// Creates a new agent state from the given configuration.
    pub fn new(config: AgentConfig) -> Self {
        Self {
            config,
            active_task_count: Mutex::new(0),
            task_history: Mutex::new(Vec::new()),
        }
    }

    /// Returns the agent's ID.
    pub fn id(&self) -> &AgentId {
        &self.config.id
    }

    /// Returns the agent's name.
    pub fn name(&self) -> &str {
        &self.config.name
    }

    /// Returns the number of currently active tasks.
    pub fn active_task_count(&self) -> usize {
        *self.active_task_count.lock().unwrap()
    }

    /// Checks if this agent can accept a new task.
    pub fn can_accept_task(&self) -> bool {
        self.active_task_count() < self.config.max_concurrent_tasks
    }

    /// Increments the active task count.
    ///
    /// Returns an error if the agent is already at maximum capacity.
    pub fn start_task(&self, sub_id: String) -> Result<(), String> {
        let mut count = self.active_task_count.lock().unwrap();
        if *count >= self.config.max_concurrent_tasks {
            return Err(format!(
                "Agent '{}' is at maximum capacity ({} tasks)",
                self.config.name, self.config.max_concurrent_tasks
            ));
        }

        *count += 1;

        // Record task start
        let mut history = self.task_history.lock().unwrap();
        history.push(TaskRecord {
            sub_id,
            started_at: SystemTime::now(),
            completed_at: None,
            success: false,
        });

        Ok(())
    }

    /// Decrements the active task count and records completion.
    pub fn complete_task(&self, sub_id: &str, success: bool) {
        let mut count = self.active_task_count.lock().unwrap();
        if *count > 0 {
            *count -= 1;
        }

        // Update task record
        let mut history = self.task_history.lock().unwrap();
        if let Some(record) = history.iter_mut().rev().find(|r| r.sub_id == sub_id) {
            record.completed_at = Some(SystemTime::now());
            record.success = success;
        }
    }

    /// Returns the task history for this agent.
    pub fn task_history(&self) -> Vec<TaskRecord> {
        self.task_history.lock().unwrap().clone()
    }

    /// Returns statistics about this agent's execution.
    pub fn stats(&self) -> AgentStats {
        let history = self.task_history.lock().unwrap();
        let total_tasks = history.len();
        let completed_tasks = history.iter().filter(|r| r.completed_at.is_some()).count();
        let successful_tasks = history.iter().filter(|r| r.success).count();
        let active_tasks = *self.active_task_count.lock().unwrap();

        AgentStats {
            total_tasks,
            completed_tasks,
            successful_tasks,
            active_tasks,
        }
    }
}

/// Statistics about an agent's execution.
#[derive(Debug, Clone)]
pub struct AgentStats {
    /// Total number of tasks started.
    pub total_tasks: usize,

    /// Number of tasks that completed.
    pub completed_tasks: usize,

    /// Number of tasks that completed successfully.
    pub successful_tasks: usize,

    /// Number of currently active tasks.
    pub active_tasks: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::AgentConfig;

    #[test]
    fn test_agent_state_creation() {
        let config = AgentConfig::new("test", "Test Agent");
        let state = AgentState::new(config);

        assert_eq!(state.id().as_str(), "test");
        assert_eq!(state.name(), "Test Agent");
        assert_eq!(state.active_task_count(), 0);
        assert!(state.can_accept_task());
    }

    #[test]
    fn test_task_lifecycle() {
        let config = AgentConfig::new("test", "Test Agent")
            .with_max_concurrent_tasks(2);
        let state = AgentState::new(config);

        // Start first task
        assert!(state.start_task("sub1".to_string()).is_ok());
        assert_eq!(state.active_task_count(), 1);
        assert!(state.can_accept_task());

        // Start second task
        assert!(state.start_task("sub2".to_string()).is_ok());
        assert_eq!(state.active_task_count(), 2);
        assert!(!state.can_accept_task()); // At capacity

        // Try to start third task (should fail)
        assert!(state.start_task("sub3".to_string()).is_err());

        // Complete first task
        state.complete_task("sub1", true);
        assert_eq!(state.active_task_count(), 1);
        assert!(state.can_accept_task());

        // Complete second task
        state.complete_task("sub2", false);
        assert_eq!(state.active_task_count(), 0);
    }

    #[test]
    fn test_agent_stats() {
        let config = AgentConfig::new("test", "Test Agent");
        let state = AgentState::new(config);

        state.start_task("sub1".to_string()).unwrap();
        state.complete_task("sub1", true);

        state.start_task("sub2".to_string()).unwrap();
        state.complete_task("sub2", false);

        state.start_task("sub3".to_string()).unwrap();

        let stats = state.stats();
        assert_eq!(stats.total_tasks, 3);
        assert_eq!(stats.completed_tasks, 2);
        assert_eq!(stats.successful_tasks, 1);
        assert_eq!(stats.active_tasks, 1);
    }
}
