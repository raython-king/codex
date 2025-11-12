/// Task management for agent coordinator.
///
/// This module defines the data structures for coordinated tasks,
/// including task types, priorities, and status tracking.

use crate::agent::AgentId;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Coordinated task - a task managed by the coordinator
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoordinatedTask {
    /// Unique task identifier
    pub id: String,

    /// Task type
    pub task_type: TaskType,

    /// Task payload (flexible JSON)
    pub payload: serde_json::Value,

    /// Target agent (None = coordinator selects)
    pub target_agent: Option<AgentId>,

    /// Priority level
    pub priority: TaskPriority,

    /// Optional timeout in milliseconds
    pub timeout_ms: Option<u64>,

    /// Creation timestamp
    pub created_at: SystemTime,

    /// Current status
    pub status: TaskStatus,

    /// Retry count
    pub retry_count: u32,
}

impl CoordinatedTask {
    /// Creates a new task
    pub fn new(id: String, task_type: TaskType, payload: serde_json::Value) -> Self {
        Self {
            id,
            task_type,
            payload,
            target_agent: None,
            priority: TaskPriority::Normal,
            timeout_ms: None,
            created_at: SystemTime::now(),
            status: TaskStatus::Pending,
            retry_count: 0,
        }
    }

    /// Sets the target agent
    pub fn with_target_agent(mut self, agent_id: AgentId) -> Self {
        self.target_agent = Some(agent_id);
        self
    }

    /// Sets the priority
    pub fn with_priority(mut self, priority: TaskPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Sets the timeout
    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = Some(timeout_ms);
        self
    }

    /// Checks if the task has timed out
    pub fn is_timed_out(&self) -> bool {
        if let Some(timeout) = self.timeout_ms {
            if let Ok(elapsed) = self.created_at.elapsed() {
                return elapsed.as_millis() as u64 > timeout;
            }
        }
        false
    }
}

/// Task type - defines how the task should be executed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskType {
    /// Single task - assign to one agent
    Single,

    /// Parallel task - assign to multiple agents concurrently
    Parallel {
        /// Number of agents to use
        agent_count: usize,
    },

    /// Sequential task - execute subtasks in order
    Sequential {
        /// Sub-tasks to execute
        subtasks: Vec<CoordinatedTask>,
    },

    /// Broadcast task - send to all agents
    Broadcast,
}

/// Task priority - determines execution order
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TaskPriority {
    /// Low priority (background tasks)
    Low,

    /// Normal priority (default)
    Normal,

    /// High priority (urgent tasks)
    High,

    /// Critical priority (system tasks)
    Critical,
}

impl Default for TaskPriority {
    fn default() -> Self {
        Self::Normal
    }
}

/// Task status - tracks task lifecycle
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    /// Task is waiting to be assigned
    Pending,

    /// Task has been assigned to an agent
    Assigned {
        /// Agent ID
        agent_id: AgentId,
    },

    /// Task is currently running
    Running {
        /// Agent executing the task
        agent_id: AgentId,
    },

    /// Task completed successfully
    Completed {
        /// Task result
        result: TaskResult,
    },

    /// Task failed
    Failed {
        /// Error message
        error: String,
    },

    /// Task timed out
    TimedOut,

    /// Task was cancelled
    Cancelled,
}

/// Task result - the outcome of a completed task
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskResult {
    /// Task ID
    pub task_id: String,

    /// Agent that executed the task
    pub agent_id: AgentId,

    /// Result data (flexible JSON)
    pub result: serde_json::Value,

    /// Execution duration in milliseconds
    pub duration_ms: u64,

    /// Completion timestamp
    pub completed_at: SystemTime,
}

impl TaskResult {
    /// Creates a new task result
    pub fn new(
        task_id: String,
        agent_id: AgentId,
        result: serde_json::Value,
        duration_ms: u64,
    ) -> Self {
        Self {
            task_id,
            agent_id,
            result,
            duration_ms,
            completed_at: SystemTime::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_task_creation() {
        let task = CoordinatedTask::new(
            "task-1".to_string(),
            TaskType::Single,
            json!({"action": "test"}),
        );

        assert_eq!(task.id, "task-1");
        assert_eq!(task.task_type, TaskType::Single);
        assert_eq!(task.priority, TaskPriority::Normal);
        assert!(matches!(task.status, TaskStatus::Pending));
    }

    #[test]
    fn test_task_builder() {
        let task = CoordinatedTask::new(
            "task-2".to_string(),
            TaskType::Single,
            json!({}),
        )
        .with_target_agent(AgentId::new("agent-a"))
        .with_priority(TaskPriority::High)
        .with_timeout(5000);

        assert_eq!(task.target_agent, Some(AgentId::new("agent-a")));
        assert_eq!(task.priority, TaskPriority::High);
        assert_eq!(task.timeout_ms, Some(5000));
    }

    #[test]
    fn test_priority_ordering() {
        assert!(TaskPriority::Critical > TaskPriority::High);
        assert!(TaskPriority::High > TaskPriority::Normal);
        assert!(TaskPriority::Normal > TaskPriority::Low);
    }

    #[test]
    fn test_task_result() {
        let result = TaskResult::new(
            "task-1".to_string(),
            AgentId::new("agent-a"),
            json!({"status": "success"}),
            150,
        );

        assert_eq!(result.task_id, "task-1");
        assert_eq!(result.agent_id, AgentId::new("agent-a"));
        assert_eq!(result.duration_ms, 150);
    }

    #[test]
    fn test_task_timeout() {
        let task = CoordinatedTask::new(
            "task-1".to_string(),
            TaskType::Single,
            json!({}),
        )
        .with_timeout(1); // 1ms timeout

        // Task should timeout
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(task.is_timed_out());
    }
}
