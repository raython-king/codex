//! Agent identifier types.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Unique identifier for an agent within a session.
///
/// Each agent in a multi-agent session has a unique ID that is used to route
/// operations and track events. The special ID "default" is used for backward
/// compatibility with single-agent sessions.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AgentId(String);

impl AgentId {
    /// Creates a new AgentId from a string.
    ///
    /// # Example
    /// ```
    /// use codex_core::agent::AgentId;
    /// let id = AgentId::new("planner");
    /// ```
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Returns the default agent ID used for backward compatibility.
    ///
    /// When no agent_id is specified in operations, this ID is used
    /// to maintain compatibility with single-agent sessions.
    pub fn default_agent() -> Self {
        Self("default".to_string())
    }

    /// Returns the inner string value.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Checks if this is the default agent.
    pub fn is_default(&self) -> bool {
        self.0 == "default"
    }
}

impl fmt::Display for AgentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for AgentId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for AgentId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl AsRef<str> for AgentId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_id_creation() {
        let id = AgentId::new("test-agent");
        assert_eq!(id.as_str(), "test-agent");
    }

    #[test]
    fn test_default_agent() {
        let default_id = AgentId::default_agent();
        assert_eq!(default_id.as_str(), "default");
        assert!(default_id.is_default());
    }

    #[test]
    fn test_agent_id_equality() {
        let id1 = AgentId::new("agent1");
        let id2 = AgentId::new("agent1");
        let id3 = AgentId::new("agent2");

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_agent_id_serialization() {
        let id = AgentId::new("test-agent");
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, r#""test-agent""#);

        let deserialized: AgentId = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, id);
    }
}
