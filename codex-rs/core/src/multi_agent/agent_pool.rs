//! Agent pool management for multi-agent collaboration
//!
//! This is a foundational implementation that provides the structure for
//! multi-agent coordination. Full integration with Session management
//! will be completed in future iterations.

use super::types::{AgentCapability, AgentId, AgentSpec};
use crate::error::CodexErr;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Configuration for the agent pool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPoolConfig {
    /// Maximum number of agents in the pool
    pub max_agents: usize,
    /// Maximum number of concurrent tasks across all agents
    pub max_concurrent_tasks: usize,
    /// Enable event aggregation from child agents
    pub enable_event_aggregation: bool,
}

impl Default for AgentPoolConfig {
    fn default() -> Self {
        Self {
            max_agents: 10,
            max_concurrent_tasks: 20,
            enable_event_aggregation: true,
        }
    }
}

/// Information about a managed agent
#[derive(Debug, Clone)]
struct ManagedAgent {
    /// Agent specification
    spec: AgentSpec,
    /// Number of currently active tasks
    active_tasks: usize,
}

/// Manages a pool of agents for multi-agent collaboration
///
/// Note: This is a placeholder implementation. Full integration with
/// the Session system will be added in a future update.
pub struct AgentPool {
    /// Pool configuration
    config: AgentPoolConfig,
    /// Managed agents by ID
    agents: Arc<RwLock<HashMap<AgentId, ManagedAgent>>>,
}

impl AgentPool {
    /// Create a new agent pool
    pub fn new(config: AgentPoolConfig) -> Self {
        Self {
            config,
            agents: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add an agent to the pool
    ///
    /// Note: Currently registers the agent specification. Session creation
    /// will be integrated in a future update.
    pub async fn add_agent(&self, spec: AgentSpec) -> Result<AgentId, CodexErr> {
        let mut agents = self.agents.write().await;

        if agents.len() >= self.config.max_agents {
            return Err(CodexErr::Fatal(format!(
                "Agent pool is full (max: {})",
                self.config.max_agents
            )));
        }

        if agents.contains_key(&spec.id) {
            return Err(CodexErr::Fatal(format!(
                "Agent with ID '{}' already exists",
                spec.id
            )));
        }

        let agent_id = spec.id.clone();

        let managed_agent = ManagedAgent {
            spec,
            active_tasks: 0,
        };

        agents.insert(agent_id.clone(), managed_agent);

        Ok(agent_id)
    }

    /// Remove an agent from the pool
    pub async fn remove_agent(&self, agent_id: &AgentId) -> Result<(), CodexErr> {
        let mut agents = self.agents.write().await;

        agents
            .remove(agent_id)
            .ok_or_else(|| CodexErr::Fatal(format!("Agent with ID '{}' not found", agent_id)))?;

        Ok(())
    }

    /// Find agents with specific capabilities
    pub async fn find_agents_with_capabilities(
        &self,
        required_capabilities: &[AgentCapability],
    ) -> Vec<AgentId> {
        let agents = self.agents.read().await;

        agents
            .iter()
            .filter(|(_, agent)| {
                required_capabilities
                    .iter()
                    .all(|cap| agent.spec.has_capability(cap))
            })
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Get the least loaded agent with required capabilities
    pub async fn get_least_loaded_agent(
        &self,
        required_capabilities: &[AgentCapability],
    ) -> Option<AgentId> {
        let agents = self.agents.read().await;

        agents
            .iter()
            .filter(|(_, agent)| {
                required_capabilities
                    .iter()
                    .all(|cap| agent.spec.has_capability(cap))
                    && agent.active_tasks < agent.spec.max_concurrent_tasks
            })
            .min_by_key(|(_, agent)| agent.active_tasks)
            .map(|(id, _)| id.clone())
    }

    /// Increment task count for an agent
    pub async fn increment_task_count(&self, agent_id: &AgentId) -> Result<(), CodexErr> {
        let mut agents = self.agents.write().await;

        let agent = agents
            .get_mut(agent_id)
            .ok_or_else(|| CodexErr::Fatal(format!("Agent with ID '{}' not found", agent_id)))?;

        agent.active_tasks += 1;

        Ok(())
    }

    /// Decrement task count for an agent
    pub async fn decrement_task_count(&self, agent_id: &AgentId) -> Result<(), CodexErr> {
        let mut agents = self.agents.write().await;

        let agent = agents
            .get_mut(agent_id)
            .ok_or_else(|| CodexErr::Fatal(format!("Agent with ID '{}' not found", agent_id)))?;

        if agent.active_tasks > 0 {
            agent.active_tasks -= 1;
        }

        Ok(())
    }

    /// Get statistics about the agent pool
    pub async fn get_stats(&self) -> AgentPoolStats {
        let agents = self.agents.read().await;

        let total_agents = agents.len();
        let total_active_tasks = agents.values().map(|a| a.active_tasks).sum();
        let available_capacity = agents
            .values()
            .map(|a| a.spec.max_concurrent_tasks.saturating_sub(a.active_tasks))
            .sum();

        AgentPoolStats {
            total_agents,
            total_active_tasks,
            available_capacity,
            max_agents: self.config.max_agents,
            max_concurrent_tasks: self.config.max_concurrent_tasks,
        }
    }

    /// List all agents in the pool
    pub async fn list_agents(&self) -> Vec<(AgentId, AgentSpec)> {
        let agents = self.agents.read().await;
        agents
            .iter()
            .map(|(id, agent)| (id.clone(), agent.spec.clone()))
            .collect()
    }

    /// Get agent specification
    pub async fn get_agent_spec(&self, agent_id: &AgentId) -> Option<AgentSpec> {
        let agents = self.agents.read().await;
        agents.get(agent_id).map(|a| a.spec.clone())
    }
}

/// Statistics about the agent pool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPoolStats {
    /// Total number of agents
    pub total_agents: usize,
    /// Total number of active tasks across all agents
    pub total_active_tasks: usize,
    /// Available task capacity
    pub available_capacity: usize,
    /// Maximum number of agents allowed
    pub max_agents: usize,
    /// Maximum concurrent tasks allowed
    pub max_concurrent_tasks: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::multi_agent::types::AgentRole;

    #[tokio::test]
    async fn test_agent_pool_creation() {
        let config = AgentPoolConfig::default();
        let pool = AgentPool::new(config);
        let stats = pool.get_stats().await;
        assert_eq!(stats.total_agents, 0);
    }

    #[tokio::test]
    async fn test_add_agent() {
        let pool = AgentPool::new(AgentPoolConfig::default());

        let spec = AgentSpec::new("test-agent", AgentRole::GeneralPurpose)
            .with_capability(AgentCapability::CodeGeneration);

        let result = pool.add_agent(spec).await;
        assert!(result.is_ok());

        let stats = pool.get_stats().await;
        assert_eq!(stats.total_agents, 1);
    }

    #[tokio::test]
    async fn test_find_agents_with_capabilities() {
        let pool = AgentPool::new(AgentPoolConfig::default());

        let spec1 = AgentSpec::new("agent-1", AgentRole::GeneralPurpose)
            .with_capability(AgentCapability::CodeGeneration);

        let spec2 = AgentSpec::new("agent-2", AgentRole::GeneralPurpose)
            .with_capability(AgentCapability::CodeReview);

        pool.add_agent(spec1).await.unwrap();
        pool.add_agent(spec2).await.unwrap();

        let agents = pool
            .find_agents_with_capabilities(&[AgentCapability::CodeGeneration])
            .await;

        assert_eq!(agents.len(), 1);
        assert_eq!(agents[0].0, "agent-1");
    }
}
