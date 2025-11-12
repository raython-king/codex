//! Protocol-level agent configuration types.
//!
//! This module defines a simplified AgentConfig suitable for protocol
//! serialization. The core crate has a richer AgentConfig with additional
//! runtime state.

use crate::AgentId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Protocol-level agent configuration.
///
/// This is a simplified version used for RegisterAgent operations.
/// The core crate has a more detailed AgentConfig for runtime use.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[ts(export)]
pub struct AgentConfig {
    /// Unique identifier for this agent.
    pub id: AgentId,

    /// Human-readable name for this agent.
    pub name: String,

    /// Role/specialization of this agent (as a string for flexibility).
    #[serde(default = "default_role")]
    pub role: String,

    /// Optional custom system prompt for this agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,

    /// Optional list of allowed tool names.
    ///
    /// If provided, this agent will only be able to use tools in this list.
    /// If None, the agent can use all available tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_tools: Option<Vec<String>>,

    /// Maximum number of concurrent tasks this agent can handle.
    ///
    /// Defaults to 1 for sequential task execution.
    #[serde(default = "default_max_concurrent_tasks")]
    pub max_concurrent_tasks: usize,
}

fn default_role() -> String {
    "generic".to_string()
}

fn default_max_concurrent_tasks() -> usize {
    1
}

impl AgentConfig {
    /// Creates a new agent configuration with the given ID and name.
    pub fn new(id: impl Into<AgentId>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            role: default_role(),
            system_prompt: None,
            allowed_tools: None,
            max_concurrent_tasks: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_config_creation() {
        let config = AgentConfig::new("test", "Test Agent");
        assert_eq!(config.id.as_str(), "test");
        assert_eq!(config.name, "Test Agent");
        assert_eq!(config.role, "generic");
        assert_eq!(config.max_concurrent_tasks, 1);
    }

    #[test]
    fn test_agent_config_serialization() {
        let config = AgentConfig {
            id: AgentId::new("planner"),
            name: "Task Planner".to_string(),
            role: "planner".to_string(),
            system_prompt: Some("You are a task planner.".to_string()),
            allowed_tools: Some(vec!["plan".to_string(), "task".to_string()]),
            max_concurrent_tasks: 2,
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: AgentConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, config.id);
        assert_eq!(deserialized.name, config.name);
        assert_eq!(deserialized.role, config.role);
    }

    #[test]
    fn test_agent_config_defaults() {
        let json = r#"{"id":"test","name":"Test"}"#;
        let config: AgentConfig = serde_json::from_str(json).unwrap();

        assert_eq!(config.role, "generic");
        assert_eq!(config.max_concurrent_tasks, 1);
        assert!(config.system_prompt.is_none());
        assert!(config.allowed_tools.is_none());
    }
}
