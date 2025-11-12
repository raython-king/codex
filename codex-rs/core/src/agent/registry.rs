//! Agent registry for managing multiple agents in a session.
//!
//! The [`AgentRegistry`] is the central component for managing agent lifecycles
//! within a Codex session. It handles registration, lookup, and removal of agents,
//! while ensuring the default agent is always available for backward compatibility.
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```
//! use codex_core::agent::{AgentRegistry, AgentConfig, AgentRole};
//!
//! // Create a new registry (includes default agent)
//! let mut registry = AgentRegistry::new();
//!
//! // Register a specialized agent
//! let planner = AgentConfig::new("planner", "Task Planner")
//!     .with_role(AgentRole::Planner);
//!
//! let agent_id = registry.register(planner).unwrap();
//!
//! // Look up the agent
//! let agent_state = registry.get(&agent_id).unwrap();
//! println!("Agent: {}", agent_state.name());
//! ```
//!
//! ## Managing Multiple Agents
//!
//! ```
//! use codex_core::agent::{AgentRegistry, AgentConfig, AgentRole};
//!
//! let mut registry = AgentRegistry::new();
//!
//! // Register multiple specialized agents
//! let agents = vec![
//!     AgentConfig::new("planner", "Planner").with_role(AgentRole::Planner),
//!     AgentConfig::new("coder", "Coder").with_role(AgentRole::Coder),
//!     AgentConfig::new("reviewer", "Reviewer").with_role(AgentRole::Reviewer),
//! ];
//!
//! for config in agents {
//!     registry.register(config).expect("Failed to register agent");
//! }
//!
//! // List all agents
//! for agent_id in registry.agent_ids() {
//!     println!("Registered agent: {}", agent_id.as_str());
//! }
//! ```
//!
//! ## Using Default Agent
//!
//! ```
//! use codex_core::agent::{AgentRegistry, AgentId};
//!
//! let registry = AgentRegistry::new();
//!
//! // Get the default agent (always available)
//! let default_agent = registry.default();
//!
//! // Or use get_or_default for None values
//! let agent = registry.get_or_default(None);
//! assert_eq!(agent.id(), registry.default_id());
//! ```

use super::{AgentConfig, AgentId, AgentState};
use std::collections::HashMap;
use std::sync::Arc;

/// Error type for agent registry operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentRegistryError {
    /// Agent with this ID already exists.
    AgentAlreadyExists(AgentId),

    /// Agent with this ID was not found.
    AgentNotFound(AgentId),

    /// Cannot unregister the default agent.
    CannotUnregisterDefaultAgent,
}

impl std::fmt::Display for AgentRegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AgentAlreadyExists(id) => {
                write!(f, "Agent with ID '{}' already exists", id)
            }
            Self::AgentNotFound(id) => {
                write!(f, "Agent with ID '{}' not found", id)
            }
            Self::CannotUnregisterDefaultAgent => {
                write!(f, "Cannot unregister the default agent")
            }
        }
    }
}

impl std::error::Error for AgentRegistryError {}

/// Registry of all agents in a session.
///
/// The `AgentRegistry` maintains a collection of agents and provides methods
/// for managing their lifecycle. A default agent is always present and cannot
/// be removed, ensuring backward compatibility.
///
/// # Thread Safety
///
/// Agent states are wrapped in `Arc` for efficient sharing across threads.
/// The registry itself should be protected by a lock when shared across threads.
///
/// # Examples
///
/// ```
/// use codex_core::agent::{AgentRegistry, AgentConfig, AgentRole};
///
/// let mut registry = AgentRegistry::new();
///
/// // Register a new agent
/// let config = AgentConfig::new("planner", "Task Planner")
///     .with_role(AgentRole::Planner)
///     .with_allowed_tools(vec!["Read".to_string(), "Grep".to_string()]);
///
/// match registry.register(config) {
///     Ok(agent_id) => println!("Registered agent: {}", agent_id.as_str()),
///     Err(e) => eprintln!("Registration failed: {}", e),
/// }
/// ```
pub struct AgentRegistry {
    /// Map of agent ID to agent state.
    agents: HashMap<AgentId, Arc<AgentState>>,

    /// ID of the default agent (used for backward compatibility).
    default_agent: AgentId,
}

impl AgentRegistry {
    /// Creates a new agent registry with a default agent.
    ///
    /// The default agent is automatically created and will be used for
    /// all operations that don't specify an agent ID, ensuring backward
    /// compatibility with single-agent behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// use codex_core::agent::AgentRegistry;
    ///
    /// let registry = AgentRegistry::new();
    /// assert_eq!(registry.count(), 1); // Contains default agent
    /// ```
    pub fn new() -> Self {
        let default_id = AgentId::default_agent();
        let default_config = AgentConfig::new(
            default_id.clone(),
            "Default Agent",
        );
        let default_state = Arc::new(AgentState::new(default_config));

        let mut agents = HashMap::new();
        agents.insert(default_id.clone(), default_state);

        Self {
            agents,
            default_agent: default_id,
        }
    }

    /// Registers a new agent with the given configuration.
    ///
    /// # Errors
    ///
    /// Returns [`AgentRegistryError::AgentAlreadyExists`] if an agent with
    /// the same ID is already registered.
    ///
    /// # Examples
    ///
    /// ```
    /// use codex_core::agent::{AgentRegistry, AgentConfig, AgentRole};
    ///
    /// let mut registry = AgentRegistry::new();
    ///
    /// let config = AgentConfig::new("coder", "Code Writer")
    ///     .with_role(AgentRole::Coder);
    ///
    /// let agent_id = registry.register(config).unwrap();
    /// assert_eq!(agent_id.as_str(), "coder");
    /// ```
    pub fn register(&mut self, config: AgentConfig) -> Result<AgentId, AgentRegistryError> {
        let id = config.id.clone();

        if self.agents.contains_key(&id) {
            return Err(AgentRegistryError::AgentAlreadyExists(id));
        }

        let state = Arc::new(AgentState::new(config));
        self.agents.insert(id.clone(), state);

        Ok(id)
    }

    /// Unregisters an agent by ID.
    ///
    /// # Errors
    ///
    /// - Returns [`AgentRegistryError::CannotUnregisterDefaultAgent`] if attempting
    ///   to unregister the default agent.
    /// - Returns [`AgentRegistryError::AgentNotFound`] if the agent doesn't exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use codex_core::agent::{AgentRegistry, AgentConfig};
    ///
    /// let mut registry = AgentRegistry::new();
    ///
    /// // Register an agent
    /// let config = AgentConfig::new("temp", "Temporary Agent");
    /// let agent_id = registry.register(config).unwrap();
    ///
    /// // Unregister it when done
    /// registry.unregister(&agent_id).unwrap();
    /// assert!(!registry.contains(&agent_id));
    /// ```
    pub fn unregister(&mut self, id: &AgentId) -> Result<(), AgentRegistryError> {
        if id == &self.default_agent {
            return Err(AgentRegistryError::CannotUnregisterDefaultAgent);
        }

        if self.agents.remove(id).is_none() {
            return Err(AgentRegistryError::AgentNotFound(id.clone()));
        }

        Ok(())
    }

    /// Gets an agent by ID.
    ///
    /// Returns None if the agent doesn't exist.
    pub fn get(&self, id: &AgentId) -> Option<Arc<AgentState>> {
        self.agents.get(id).cloned()
    }

    /// Gets the default agent.
    pub fn default(&self) -> Arc<AgentState> {
        self.agents
            .get(&self.default_agent)
            .expect("Default agent should always exist")
            .clone()
    }

    /// Gets the default agent ID.
    pub fn default_id(&self) -> &AgentId {
        &self.default_agent
    }

    /// Returns all registered agent IDs.
    pub fn agent_ids(&self) -> Vec<AgentId> {
        self.agents.keys().cloned().collect()
    }

    /// Returns the number of registered agents.
    pub fn count(&self) -> usize {
        self.agents.len()
    }

    /// Checks if an agent with the given ID exists.
    pub fn contains(&self, id: &AgentId) -> bool {
        self.agents.contains_key(id)
    }

    /// Gets an agent or the default agent if the ID is None.
    ///
    /// This is the primary method for resolving agent IDs in a backward-compatible way:
    /// - If `id` is `None`, returns the default agent
    /// - If `id` is `Some` but the agent doesn't exist, returns the default agent
    /// - If `id` is `Some` and exists, returns that agent
    ///
    /// # Examples
    ///
    /// ```
    /// use codex_core::agent::{AgentRegistry, AgentConfig, AgentId};
    ///
    /// let mut registry = AgentRegistry::new();
    /// let config = AgentConfig::new("custom", "Custom Agent");
    /// registry.register(config).unwrap();
    ///
    /// // Get specific agent
    /// let custom = registry.get_or_default(Some(&AgentId::new("custom")));
    /// assert_eq!(custom.id().as_str(), "custom");
    ///
    /// // Get default when None
    /// let default = registry.get_or_default(None);
    /// assert_eq!(default.id(), registry.default_id());
    /// ```
    pub fn get_or_default(&self, id: Option<&AgentId>) -> Arc<AgentState> {
        match id {
            Some(id) => self.get(id).unwrap_or_else(|| self.default()),
            None => self.default(),
        }
    }
}

impl Default for AgentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::AgentRole;

    #[test]
    fn test_registry_creation() {
        let registry = AgentRegistry::new();
        assert_eq!(registry.count(), 1);
        assert!(registry.contains(&AgentId::default_agent()));
    }

    #[test]
    fn test_register_agent() {
        let mut registry = AgentRegistry::new();

        let config = AgentConfig::new("planner", "Task Planner")
            .with_role(AgentRole::Planner);

        let id = registry.register(config).unwrap();
        assert_eq!(id.as_str(), "planner");
        assert_eq!(registry.count(), 2);
        assert!(registry.contains(&id));
    }

    #[test]
    fn test_register_duplicate() {
        let mut registry = AgentRegistry::new();

        let config1 = AgentConfig::new("test", "Test 1");
        let config2 = AgentConfig::new("test", "Test 2");

        assert!(registry.register(config1).is_ok());
        assert!(matches!(
            registry.register(config2),
            Err(AgentRegistryError::AgentAlreadyExists(_))
        ));
    }

    #[test]
    fn test_unregister_agent() {
        let mut registry = AgentRegistry::new();

        let config = AgentConfig::new("test", "Test Agent");
        let id = registry.register(config).unwrap();

        assert_eq!(registry.count(), 2);
        assert!(registry.unregister(&id).is_ok());
        assert_eq!(registry.count(), 1);
        assert!(!registry.contains(&id));
    }

    #[test]
    fn test_cannot_unregister_default() {
        let mut registry = AgentRegistry::new();
        let default_id = registry.default_id().clone();

        assert!(matches!(
            registry.unregister(&default_id),
            Err(AgentRegistryError::CannotUnregisterDefaultAgent)
        ));
    }

    #[test]
    fn test_get_agent() {
        let mut registry = AgentRegistry::new();

        let config = AgentConfig::new("test", "Test Agent");
        let id = registry.register(config).unwrap();

        let state = registry.get(&id);
        assert!(state.is_some());
        assert_eq!(state.unwrap().id(), &id);
    }

    #[test]
    fn test_get_or_default() {
        let mut registry = AgentRegistry::new();

        let config = AgentConfig::new("test", "Test Agent");
        let id = registry.register(config).unwrap();

        // Get specific agent
        let state1 = registry.get_or_default(Some(&id));
        assert_eq!(state1.id(), &id);

        // Get default when None
        let state2 = registry.get_or_default(None);
        assert_eq!(state2.id(), &AgentId::default_agent());

        // Get default when non-existent ID
        let non_existent = AgentId::new("non-existent");
        let state3 = registry.get_or_default(Some(&non_existent));
        assert_eq!(state3.id(), &AgentId::default_agent());
    }

    #[test]
    fn test_agent_ids() {
        let mut registry = AgentRegistry::new();

        registry.register(AgentConfig::new("agent1", "Agent 1")).unwrap();
        registry.register(AgentConfig::new("agent2", "Agent 2")).unwrap();

        let ids = registry.agent_ids();
        assert_eq!(ids.len(), 3); // default + 2 registered
        assert!(ids.contains(&AgentId::default_agent()));
        assert!(ids.contains(&AgentId::new("agent1")));
        assert!(ids.contains(&AgentId::new("agent2")));
    }
}
