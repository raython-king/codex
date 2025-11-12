/// Task allocation strategies for the agent coordinator.
///
/// This module implements different strategies for assigning tasks to agents,
/// including round-robin, load-based, capability-based, and priority-aware approaches.

use crate::agent::{AgentId, AgentRegistry};
use crate::coordinator::task::{CoordinatedTask, TaskPriority};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Trait for task allocation strategies
pub trait AllocationStrategy: Send + Sync {
    /// Select an agent for the given task
    fn select_agent(
        &self,
        task: &CoordinatedTask,
        registry: &AgentRegistry,
    ) -> Result<AgentId, String>;

    /// Strategy name for debugging
    fn name(&self) -> &str;
}

/// Round-robin strategy - evenly distributes tasks across agents
pub struct RoundRobinStrategy {
    current_index: AtomicUsize,
}

impl RoundRobinStrategy {
    pub fn new() -> Self {
        Self {
            current_index: AtomicUsize::new(0),
        }
    }
}

impl Default for RoundRobinStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl AllocationStrategy for RoundRobinStrategy {
    fn select_agent(
        &self,
        _task: &CoordinatedTask,
        registry: &AgentRegistry,
    ) -> Result<AgentId, String> {
        let agents = registry.list_agents();
        if agents.is_empty() {
            return Err("No agents available".to_string());
        }

        let index = self.current_index.fetch_add(1, Ordering::SeqCst) % agents.len();
        Ok(agents[index].clone())
    }

    fn name(&self) -> &str {
        "RoundRobin"
    }
}

/// Load-based strategy - selects the least busy agent
pub struct LoadBasedStrategy;

impl LoadBasedStrategy {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LoadBasedStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl AllocationStrategy for LoadBasedStrategy {
    fn select_agent(
        &self,
        _task: &CoordinatedTask,
        registry: &AgentRegistry,
    ) -> Result<AgentId, String> {
        let agents = registry.list_agents();
        if agents.is_empty() {
            return Err("No agents available".to_string());
        }

        // Find agent with lowest active task count
        let mut min_load_agent = None;
        let mut min_load = usize::MAX;

        for agent_id in agents {
            if let Some(agent) = registry.get(&agent_id) {
                let active_tasks = agent.active_task_count();
                if active_tasks < min_load {
                    min_load = active_tasks;
                    min_load_agent = Some(agent_id.clone());
                }
            }
        }

        min_load_agent.ok_or_else(|| "No suitable agent found".to_string())
    }

    fn name(&self) -> &str {
        "LoadBased"
    }
}

/// Capability-based strategy - matches tasks to agents by capabilities
pub struct CapabilityBasedStrategy {
    /// Fallback to another strategy if no capability match
    fallback: Box<dyn AllocationStrategy + Send + Sync>,
}

impl CapabilityBasedStrategy {
    pub fn new() -> Self {
        Self {
            fallback: Box::new(LoadBasedStrategy::new()),
        }
    }

    pub fn with_fallback(fallback: Box<dyn AllocationStrategy + Send + Sync>) -> Self {
        Self { fallback }
    }

    /// Check if agent has the required capabilities for a task
    fn has_capabilities(
        agent_id: &AgentId,
        registry: &AgentRegistry,
        _task: &CoordinatedTask,
    ) -> bool {
        if let Some(agent) = registry.get(agent_id) {
            // If agent has no tool restrictions, it can do anything
            if agent.config.allowed_tools.is_none() {
                return true;
            }

            // For now, we consider all agents capable
            // In the future, this can check task.payload for required tools
            true
        } else {
            false
        }
    }
}

impl Default for CapabilityBasedStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl AllocationStrategy for CapabilityBasedStrategy {
    fn select_agent(
        &self,
        task: &CoordinatedTask,
        registry: &AgentRegistry,
    ) -> Result<AgentId, String> {
        let agents = registry.list_agents();
        if agents.is_empty() {
            return Err("No agents available".to_string());
        }

        // Find first agent with matching capabilities
        for agent_id in &agents {
            if Self::has_capabilities(agent_id, registry, task) {
                return Ok(agent_id.clone());
            }
        }

        // Fallback to other strategy if no capability match
        self.fallback.select_agent(task, registry)
    }

    fn name(&self) -> &str {
        "CapabilityBased"
    }
}

/// Priority-aware strategy - considers task priority when selecting agents
pub struct PriorityAwareStrategy {
    base_strategy: Box<dyn AllocationStrategy + Send + Sync>,
}

impl PriorityAwareStrategy {
    pub fn new() -> Self {
        Self {
            base_strategy: Box::new(LoadBasedStrategy::new()),
        }
    }

    pub fn with_base_strategy(base_strategy: Box<dyn AllocationStrategy + Send + Sync>) -> Self {
        Self { base_strategy }
    }
}

impl Default for PriorityAwareStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl AllocationStrategy for PriorityAwareStrategy {
    fn select_agent(
        &self,
        task: &CoordinatedTask,
        registry: &AgentRegistry,
    ) -> Result<AgentId, String> {
        match task.priority {
            TaskPriority::Critical | TaskPriority::High => {
                // High priority: always use load-based to find least busy agent
                LoadBasedStrategy::new().select_agent(task, registry)
            }
            _ => {
                // Normal/Low priority: use base strategy
                self.base_strategy.select_agent(task, registry)
            }
        }
    }

    fn name(&self) -> &str {
        "PriorityAware"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{AgentConfig, AgentRole};
    use crate::coordinator::task::TaskType;
    use serde_json::json;

    fn create_test_registry_with_agents(count: usize) -> AgentRegistry {
        let mut registry = AgentRegistry::new();

        for i in 0..count {
            let config = AgentConfig::new(
                AgentId::new(format!("agent-{}", i)),
                format!("Agent {}", i),
            );
            registry.register(config).unwrap();
        }

        registry
    }

    #[test]
    fn test_round_robin_strategy() {
        let registry = create_test_registry_with_agents(3);
        let strategy = RoundRobinStrategy::new();

        let task = CoordinatedTask::new("task-1".to_string(), TaskType::Single, json!({}));

        // Registry has 4 agents: default + agent-0, agent-1, agent-2
        // Should cycle through all 4 agents
        let agent1 = strategy.select_agent(&task, &registry).unwrap();
        let agent2 = strategy.select_agent(&task, &registry).unwrap();
        let agent3 = strategy.select_agent(&task, &registry).unwrap();
        let agent4 = strategy.select_agent(&task, &registry).unwrap();
        let agent5 = strategy.select_agent(&task, &registry).unwrap();

        // agent5 should be same as agent1 (cycled back)
        assert_eq!(agent1, agent5);

        // All four should be different
        assert_ne!(agent1, agent2);
        assert_ne!(agent2, agent3);
        assert_ne!(agent3, agent4);
        assert_ne!(agent4, agent1);
    }

    #[test]
    fn test_load_based_strategy() {
        let mut registry = AgentRegistry::new();

        // Create agents with different loads - need higher max_concurrent_tasks
        let config1 = AgentConfig::new(
            AgentId::new("agent-1"),
            "Agent 1".to_string(),
        )
        .with_max_concurrent_tasks(10);
        let config2 = AgentConfig::new(
            AgentId::new("agent-2"),
            "Agent 2".to_string(),
        )
        .with_max_concurrent_tasks(10);

        registry.register(config1).unwrap();
        registry.register(config2).unwrap();

        // Simulate agent-1 having more active tasks
        if let Some(agent1) = registry.get(&AgentId::new("agent-1")) {
            agent1.start_task("task-1".to_string()).unwrap();
            agent1.start_task("task-2".to_string()).unwrap();
        }

        // Also add a task to the default agent so it's not the least busy
        if let Some(default_agent) = registry.get(&AgentId::new("default")) {
            default_agent.start_task("task-default".to_string()).unwrap();
        }

        let strategy = LoadBasedStrategy::new();
        let task = CoordinatedTask::new("task-1".to_string(), TaskType::Single, json!({}));

        // Should select agent-2 (lower load: 0 tasks vs agent-1: 2 tasks, default: 1 task)
        let selected = strategy.select_agent(&task, &registry).unwrap();
        assert_eq!(selected, AgentId::new("agent-2"));
    }

    #[test]
    fn test_capability_based_strategy() {
        let registry = create_test_registry_with_agents(2);
        let strategy = CapabilityBasedStrategy::new();

        let task = CoordinatedTask::new("task-1".to_string(), TaskType::Single, json!({}));

        // Should succeed with any agent (all agents are capable for now)
        let selected = strategy.select_agent(&task, &registry);
        assert!(selected.is_ok());
    }

    #[test]
    fn test_priority_aware_strategy() {
        let mut registry = AgentRegistry::new();

        // Need higher max_concurrent_tasks to allow multiple tasks
        let config1 = AgentConfig::new(
            AgentId::new("agent-1"),
            "Agent 1".to_string(),
        )
        .with_max_concurrent_tasks(10);
        let config2 = AgentConfig::new(
            AgentId::new("agent-2"),
            "Agent 2".to_string(),
        )
        .with_max_concurrent_tasks(10);

        registry.register(config1).unwrap();
        registry.register(config2).unwrap();

        // Simulate agent-1 being busy
        if let Some(agent1) = registry.get(&AgentId::new("agent-1")) {
            agent1.start_task("task-1".to_string()).unwrap();
            agent1.start_task("task-2".to_string()).unwrap();
        }

        // Also add a task to the default agent so it's not the least busy
        if let Some(default_agent) = registry.get(&AgentId::new("default")) {
            default_agent.start_task("task-default".to_string()).unwrap();
        }

        let strategy = PriorityAwareStrategy::new();

        // High priority task should go to least busy agent (agent-2: 0 tasks)
        let high_priority_task = CoordinatedTask::new(
            "task-high".to_string(),
            TaskType::Single,
            json!({}),
        )
        .with_priority(TaskPriority::High);

        let selected = strategy.select_agent(&high_priority_task, &registry).unwrap();
        assert_eq!(selected, AgentId::new("agent-2"));
    }

    #[test]
    fn test_strategy_names() {
        assert_eq!(RoundRobinStrategy::new().name(), "RoundRobin");
        assert_eq!(LoadBasedStrategy::new().name(), "LoadBased");
        assert_eq!(CapabilityBasedStrategy::new().name(), "CapabilityBased");
        assert_eq!(PriorityAwareStrategy::new().name(), "PriorityAware");
    }

    #[test]
    fn test_empty_registry() {
        let registry = AgentRegistry::new();
        let strategy = RoundRobinStrategy::new();
        let task = CoordinatedTask::new("task-1".to_string(), TaskType::Single, json!({}));

        // Registry always has a default agent, so this should succeed
        let result = strategy.select_agent(&task, &registry);
        assert!(result.is_ok());
        // Should return the default agent
        let agent_id = result.unwrap();
        assert_eq!(agent_id, AgentId::new("default"));
    }
}
