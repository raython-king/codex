//! Inter-agent communication protocol

use super::types::{AgentId, DelegatedTask, TaskResult, TaskStatus};
use crate::protocol::Event;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Messages exchanged between agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterAgentMessage {
    /// Request to delegate a task to another agent
    DelegateTask {
        /// Task to be delegated
        task: DelegatedTask,
        /// Context from the parent agent
        context: HashMap<String, String>,
    },

    /// Acknowledge task delegation
    TaskAccepted {
        /// Task identifier
        task_id: String,
        /// Agent that accepted the task
        agent_id: AgentId,
    },

    /// Reject task delegation
    TaskRejected {
        /// Task identifier
        task_id: String,
        /// Agent that rejected the task
        agent_id: AgentId,
        /// Reason for rejection
        reason: String,
    },

    /// Update on task progress
    TaskProgress {
        /// Task identifier
        task_id: String,
        /// Agent executing the task
        agent_id: AgentId,
        /// Current status
        status: TaskStatus,
        /// Progress message
        message: String,
        /// Progress percentage (0-100)
        progress_pct: Option<u8>,
    },

    /// Task completion notification
    TaskCompleted {
        /// Task result
        result: TaskResult,
        /// Events generated during task execution
        events: Vec<Event>,
    },

    /// Request for information from another agent
    InfoRequest {
        /// Request identifier
        request_id: String,
        /// Source agent
        from_agent: AgentId,
        /// Target agent
        to_agent: AgentId,
        /// Query or question
        query: String,
    },

    /// Response to information request
    InfoResponse {
        /// Request identifier this responds to
        request_id: String,
        /// Agent providing the response
        from_agent: AgentId,
        /// Response data
        data: String,
    },

    /// Request approval from parent/coordinator agent
    ApprovalRequest {
        /// Request identifier
        request_id: String,
        /// Agent requesting approval
        from_agent: AgentId,
        /// What needs approval
        action: String,
        /// Additional context
        context: HashMap<String, String>,
    },

    /// Approval decision
    ApprovalDecision {
        /// Request identifier
        request_id: String,
        /// Whether approved
        approved: bool,
        /// Reason if denied
        reason: Option<String>,
    },

    /// Broadcast a message to all agents
    Broadcast {
        /// Source agent
        from_agent: AgentId,
        /// Message content
        message: String,
        /// Message metadata
        metadata: HashMap<String, String>,
    },

    /// Agent status update
    StatusUpdate {
        /// Agent reporting status
        agent_id: AgentId,
        /// Current status
        status: AgentStatus,
        /// Status message
        message: Option<String>,
    },
}

/// Status of an agent
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentStatus {
    /// Agent is idle and ready for work
    Idle,
    /// Agent is busy processing tasks
    Busy,
    /// Agent is waiting for input or approval
    Waiting,
    /// Agent encountered an error
    Error,
    /// Agent is shutting down
    ShuttingDown,
}

/// Protocol handler for inter-agent communication
pub struct InterAgentProtocol {
    /// Agent ID of this protocol instance
    agent_id: AgentId,
    /// Message handlers
    handlers: HashMap<String, Box<dyn MessageHandler>>,
}

/// Trait for handling inter-agent messages
pub trait MessageHandler: Send + Sync {
    /// Handle an incoming message
    fn handle(&self, message: InterAgentMessage) -> Result<Option<InterAgentMessage>, String>;
}

impl InterAgentProtocol {
    /// Create a new protocol instance
    pub fn new(agent_id: AgentId) -> Self {
        Self {
            agent_id,
            handlers: HashMap::new(),
        }
    }

    /// Register a message handler
    pub fn register_handler(&mut self, message_type: String, handler: Box<dyn MessageHandler>) {
        self.handlers.insert(message_type, handler);
    }

    /// Process an incoming message
    pub fn process_message(
        &self,
        message: InterAgentMessage,
    ) -> Result<Option<InterAgentMessage>, String> {
        let message_type = Self::message_type(&message);

        if let Some(handler) = self.handlers.get(&message_type) {
            handler.handle(message)
        } else {
            // Default handling - just acknowledge
            Ok(None)
        }
    }

    /// Get the message type as a string
    fn message_type(message: &InterAgentMessage) -> String {
        match message {
            InterAgentMessage::DelegateTask { .. } => "delegate_task".to_string(),
            InterAgentMessage::TaskAccepted { .. } => "task_accepted".to_string(),
            InterAgentMessage::TaskRejected { .. } => "task_rejected".to_string(),
            InterAgentMessage::TaskProgress { .. } => "task_progress".to_string(),
            InterAgentMessage::TaskCompleted { .. } => "task_completed".to_string(),
            InterAgentMessage::InfoRequest { .. } => "info_request".to_string(),
            InterAgentMessage::InfoResponse { .. } => "info_response".to_string(),
            InterAgentMessage::ApprovalRequest { .. } => "approval_request".to_string(),
            InterAgentMessage::ApprovalDecision { .. } => "approval_decision".to_string(),
            InterAgentMessage::Broadcast { .. } => "broadcast".to_string(),
            InterAgentMessage::StatusUpdate { .. } => "status_update".to_string(),
        }
    }

    /// Create a task delegation message
    pub fn create_delegate_task_message(
        task: DelegatedTask,
        context: HashMap<String, String>,
    ) -> InterAgentMessage {
        InterAgentMessage::DelegateTask { task, context }
    }

    /// Create a task acceptance message
    pub fn create_task_accepted_message(task_id: String, agent_id: AgentId) -> InterAgentMessage {
        InterAgentMessage::TaskAccepted { task_id, agent_id }
    }

    /// Create a task rejection message
    pub fn create_task_rejected_message(
        task_id: String,
        agent_id: AgentId,
        reason: String,
    ) -> InterAgentMessage {
        InterAgentMessage::TaskRejected {
            task_id,
            agent_id,
            reason,
        }
    }

    /// Create a task progress message
    pub fn create_task_progress_message(
        task_id: String,
        agent_id: AgentId,
        status: TaskStatus,
        message: String,
        progress_pct: Option<u8>,
    ) -> InterAgentMessage {
        InterAgentMessage::TaskProgress {
            task_id,
            agent_id,
            status,
            message,
            progress_pct,
        }
    }

    /// Create a task completion message
    pub fn create_task_completed_message(
        result: TaskResult,
        events: Vec<Event>,
    ) -> InterAgentMessage {
        InterAgentMessage::TaskCompleted { result, events }
    }

    /// Create an info request message
    pub fn create_info_request_message(
        request_id: String,
        from_agent: AgentId,
        to_agent: AgentId,
        query: String,
    ) -> InterAgentMessage {
        InterAgentMessage::InfoRequest {
            request_id,
            from_agent,
            to_agent,
            query,
        }
    }

    /// Create an approval request message
    pub fn create_approval_request_message(
        request_id: String,
        from_agent: AgentId,
        action: String,
        context: HashMap<String, String>,
    ) -> InterAgentMessage {
        InterAgentMessage::ApprovalRequest {
            request_id,
            from_agent,
            action,
            context,
        }
    }

    /// Create a status update message
    pub fn create_status_update_message(
        agent_id: AgentId,
        status: AgentStatus,
        message: Option<String>,
    ) -> InterAgentMessage {
        InterAgentMessage::StatusUpdate {
            agent_id,
            status,
            message,
        }
    }

    /// Get the agent ID
    pub fn agent_id(&self) -> &AgentId {
        &self.agent_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_type_identification() {
        let task = DelegatedTask::new("task-1", "agent-1", "Test task");
        let message = InterAgentMessage::DelegateTask {
            task,
            context: HashMap::new(),
        };

        assert_eq!(InterAgentProtocol::message_type(&message), "delegate_task");
    }

    #[test]
    fn test_protocol_creation() {
        let protocol = InterAgentProtocol::new(AgentId::from("test-agent"));
        assert_eq!(protocol.agent_id().0, "test-agent");
    }
}
