//! Agent configuration types.

use super::AgentId;
use serde::{Deserialize, Serialize};

/// Role of an agent in a multi-agent system.
///
/// Defines the specialization and responsibilities of an agent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentRole {
    /// Plans and breaks down complex tasks into subtasks.
    Planner,

    /// Writes and modifies code.
    Coder,

    /// Reviews code for quality, correctness, and security.
    Reviewer,

    /// Runs tests and validates functionality.
    Tester,

    /// Writes and updates documentation.
    Documenter,

    /// Debugs issues and investigates problems.
    Debugger,

    /// Generic agent with no specific specialization.
    Generic,
}

impl Default for AgentRole {
    fn default() -> Self {
        Self::Generic
    }
}

impl AgentRole {
    /// Returns a human-readable description of this role.
    pub fn description(&self) -> &'static str {
        match self {
            Self::Planner => "Plans and decomposes complex tasks",
            Self::Coder => "Writes and modifies code",
            Self::Reviewer => "Reviews code for quality and correctness",
            Self::Tester => "Runs tests and validates functionality",
            Self::Documenter => "Writes and maintains documentation",
            Self::Debugger => "Investigates and fixes issues",
            Self::Generic => "General-purpose agent",
        }
    }
}

/// Configuration for a specific agent.
///
/// Defines the identity, role, and behavior of an agent within a session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Unique identifier for this agent.
    pub id: AgentId,

    /// Human-readable name for this agent.
    pub name: String,

    /// Role/specialization of this agent.
    #[serde(default)]
    pub role: AgentRole,

    /// Optional custom system prompt for this agent.
    ///
    /// If provided, this will be prepended to all conversations with this agent.
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

fn default_max_concurrent_tasks() -> usize {
    1
}

impl AgentConfig {
    /// Creates a new agent configuration with the given ID and name.
    pub fn new(id: impl Into<AgentId>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            role: AgentRole::Generic,
            system_prompt: None,
            allowed_tools: None,
            max_concurrent_tasks: 1,
        }
    }

    /// Sets the role for this agent.
    pub fn with_role(mut self, role: AgentRole) -> Self {
        self.role = role;
        self
    }

    /// Sets a custom system prompt for this agent.
    pub fn with_system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(prompt.into());
        self
    }

    /// Sets the allowed tools for this agent.
    pub fn with_allowed_tools(mut self, tools: Vec<String>) -> Self {
        self.allowed_tools = Some(tools);
        self
    }

    /// Sets the maximum number of concurrent tasks.
    pub fn with_max_concurrent_tasks(mut self, max: usize) -> Self {
        self.max_concurrent_tasks = max;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_role_default() {
        let role = AgentRole::default();
        assert_eq!(role, AgentRole::Generic);
    }

    #[test]
    fn test_agent_role_description() {
        assert_eq!(AgentRole::Planner.description(), "Plans and decomposes complex tasks");
        assert_eq!(AgentRole::Coder.description(), "Writes and modifies code");
    }

    #[test]
    fn test_agent_config_builder() {
        let config = AgentConfig::new("planner", "Task Planner")
            .with_role(AgentRole::Planner)
            .with_system_prompt("You are a task planning agent.")
            .with_max_concurrent_tasks(2);

        assert_eq!(config.id.as_str(), "planner");
        assert_eq!(config.name, "Task Planner");
        assert_eq!(config.role, AgentRole::Planner);
        assert_eq!(config.system_prompt, Some("You are a task planning agent.".to_string()));
        assert_eq!(config.max_concurrent_tasks, 2);
    }

    #[test]
    fn test_agent_config_serialization() {
        let config = AgentConfig::new("test", "Test Agent");
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: AgentConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, config.id);
        assert_eq!(deserialized.name, config.name);
    }
}
