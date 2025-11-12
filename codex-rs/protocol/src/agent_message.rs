/// Agent-to-agent messaging protocol.
///
/// This module defines the data structures for communication between agents
/// in a multi-agent session.

use crate::AgentId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use ts_rs::TS;

/// Type of message being sent between agents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum MessageType {
    /// Request another agent to perform a task.
    Request,
    /// Response to a previous request.
    Response,
    /// Notify another agent of an event or state change.
    Notification,
    /// Query another agent's status or capabilities.
    Query,
}

/// A message sent from one agent to another.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
#[ts(export)]
pub struct AgentMessage {
    /// Unique identifier for this message.
    pub id: String,

    /// Agent sending the message.
    pub from: AgentId,

    /// Agent receiving the message.
    pub to: AgentId,

    /// Type of message.
    pub message_type: MessageType,

    /// Message payload (flexible JSON structure).
    pub payload: serde_json::Value,

    /// Optional reference to a previous message (for responses).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<String>,

    /// Timestamp when the message was created (milliseconds since UNIX epoch).
    pub timestamp: u64,
}

impl AgentMessage {
    /// Creates a new message.
    pub fn new(
        from: AgentId,
        to: AgentId,
        message_type: MessageType,
        payload: serde_json::Value,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        Self {
            id: format!("msg-{}-{}", from.as_str(), timestamp),
            from,
            to,
            message_type,
            payload,
            in_reply_to: None,
            timestamp,
        }
    }

    /// Creates a response message to this message.
    pub fn create_response(&self, payload: serde_json::Value) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        Self {
            id: format!("msg-{}-{}", self.to.as_str(), timestamp),
            from: self.to.clone(),
            to: self.from.clone(),
            message_type: MessageType::Response,
            payload,
            in_reply_to: Some(self.id.clone()),
            timestamp,
        }
    }
}

/// Priority level for message processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum MessagePriority {
    /// Low priority (background tasks).
    Low,
    /// Normal priority (default).
    Normal,
    /// High priority (urgent requests).
    High,
    /// Critical priority (system messages).
    Critical,
}

impl Default for MessagePriority {
    fn default() -> Self {
        Self::Normal
    }
}

/// A message with priority and routing metadata.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
#[ts(export)]
pub struct QueuedMessage {
    /// The actual message.
    pub message: AgentMessage,

    /// Priority for processing.
    #[serde(default)]
    pub priority: MessagePriority,

    /// Whether this message requires a response.
    #[serde(default)]
    pub requires_response: bool,

    /// Timeout in milliseconds (None = no timeout).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
}

impl QueuedMessage {
    /// Creates a new queued message with default priority.
    pub fn new(message: AgentMessage) -> Self {
        Self {
            message,
            priority: MessagePriority::Normal,
            requires_response: false,
            timeout_ms: None,
        }
    }

    /// Sets the priority.
    pub fn with_priority(mut self, priority: MessagePriority) -> Self {
        self.priority = priority;
        self
    }

    /// Marks that this message requires a response.
    pub fn requires_response(mut self) -> Self {
        self.requires_response = true;
        self
    }

    /// Sets a timeout.
    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = Some(timeout_ms);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_agent_message_creation() {
        let from = AgentId::new("agent-a");
        let to = AgentId::new("agent-b");
        let payload = json!({"task": "process_data"});

        let msg = AgentMessage::new(
            from.clone(),
            to.clone(),
            MessageType::Request,
            payload.clone(),
        );

        assert_eq!(msg.from, from);
        assert_eq!(msg.to, to);
        assert_eq!(msg.message_type, MessageType::Request);
        assert_eq!(msg.payload, payload);
        assert!(msg.in_reply_to.is_none());
        assert!(msg.timestamp > 0);
    }

    #[test]
    fn test_create_response() {
        let original = AgentMessage::new(
            AgentId::new("agent-a"),
            AgentId::new("agent-b"),
            MessageType::Request,
            json!({"task": "process"}),
        );

        let response = original.create_response(json!({"result": "done"}));

        assert_eq!(response.from, original.to);
        assert_eq!(response.to, original.from);
        assert_eq!(response.message_type, MessageType::Response);
        assert_eq!(response.in_reply_to, Some(original.id.clone()));
    }

    #[test]
    fn test_message_serialization() {
        let msg = AgentMessage::new(
            AgentId::new("agent-a"),
            AgentId::new("agent-b"),
            MessageType::Query,
            json!({"query": "status"}),
        );

        let serialized = serde_json::to_string(&msg).unwrap();
        let deserialized: AgentMessage = serde_json::from_str(&serialized).unwrap();

        assert_eq!(msg.from, deserialized.from);
        assert_eq!(msg.to, deserialized.to);
        assert_eq!(msg.message_type, deserialized.message_type);
    }

    #[test]
    fn test_queued_message_builder() {
        let msg = AgentMessage::new(
            AgentId::new("agent-a"),
            AgentId::new("agent-b"),
            MessageType::Request,
            json!({}),
        );

        let queued = QueuedMessage::new(msg)
            .with_priority(MessagePriority::High)
            .requires_response()
            .with_timeout(5000);

        assert_eq!(queued.priority, MessagePriority::High);
        assert!(queued.requires_response);
        assert_eq!(queued.timeout_ms, Some(5000));
    }

    #[test]
    fn test_message_priority_ordering() {
        assert!(MessagePriority::Critical > MessagePriority::High);
        assert!(MessagePriority::High > MessagePriority::Normal);
        assert!(MessagePriority::Normal > MessagePriority::Low);
    }
}
