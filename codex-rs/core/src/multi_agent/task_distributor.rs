//! Task distribution and routing for multi-agent systems

use super::agent_pool::AgentPool;
use super::types::{AgentId, DelegatedTask};
use crate::error::CodexErr;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Strategy for distributing tasks to agents
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskDistributionStrategy {
    /// Round-robin distribution
    RoundRobin,
    /// Least loaded agent first
    LeastLoaded,
    /// Best match based on capabilities
    BestMatch,
    /// Priority-based distribution
    Priority,
    /// Random distribution
    Random,
}

/// Manages task distribution across multiple agents
pub struct TaskDistributor {
    /// Reference to the agent pool
    agent_pool: Arc<AgentPool>,
    /// Distribution strategy
    strategy: TaskDistributionStrategy,
    /// Round-robin counter
    round_robin_counter: Arc<RwLock<usize>>,
    /// Task assignments (task_id -> agent_id)
    task_assignments: Arc<RwLock<HashMap<String, AgentId>>>,
}

impl TaskDistributor {
    /// Create a new task distributor
    pub fn new(agent_pool: Arc<AgentPool>, strategy: TaskDistributionStrategy) -> Self {
        Self {
            agent_pool,
            strategy,
            round_robin_counter: Arc::new(RwLock::new(0)),
            task_assignments: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Distribute a task to an appropriate agent
    pub async fn distribute_task(&self, task: &DelegatedTask) -> Result<AgentId, CodexErr> {
        let agent_id = match self.strategy {
            TaskDistributionStrategy::RoundRobin => self.round_robin_distribute(task).await?,
            TaskDistributionStrategy::LeastLoaded => self.least_loaded_distribute(task).await?,
            TaskDistributionStrategy::BestMatch => self.best_match_distribute(task).await?,
            TaskDistributionStrategy::Priority => self.priority_distribute(task).await?,
            TaskDistributionStrategy::Random => self.random_distribute(task).await?,
        };

        // Record the assignment
        let mut assignments = self.task_assignments.write().await;
        assignments.insert(task.task_id.clone(), agent_id.clone());

        Ok(agent_id)
    }

    /// Get the agent assigned to a task
    pub async fn get_task_assignment(&self, task_id: &str) -> Option<AgentId> {
        let assignments = self.task_assignments.read().await;
        assignments.get(task_id).cloned()
    }

    /// Remove a task assignment
    pub async fn remove_task_assignment(&self, task_id: &str) -> Option<AgentId> {
        let mut assignments = self.task_assignments.write().await;
        assignments.remove(task_id)
    }

    /// Round-robin distribution
    async fn round_robin_distribute(&self, task: &DelegatedTask) -> Result<AgentId, CodexErr> {
        // Get agents with required capabilities
        let required_caps: Vec<_> = task.required_capabilities.iter().cloned().collect();
        let mut agents = self
            .agent_pool
            .find_agents_with_capabilities(&required_caps)
            .await;

        if agents.is_empty() {
            return Err(CodexErr::Fatal(
                "No agents found with required capabilities".to_string(),
            ));
        }

        // Sort for deterministic ordering
        agents.sort_by(|a, b| a.0.cmp(&b.0));

        // Get and increment counter
        let mut counter = self.round_robin_counter.write().await;
        let index = *counter % agents.len();
        *counter = (*counter + 1) % agents.len();

        Ok(agents[index].clone())
    }

    /// Least loaded distribution
    async fn least_loaded_distribute(&self, task: &DelegatedTask) -> Result<AgentId, CodexErr> {
        let required_caps: Vec<_> = task.required_capabilities.iter().cloned().collect();

        self.agent_pool
            .get_least_loaded_agent(&required_caps)
            .await
            .ok_or_else(|| {
                CodexErr::Fatal("No available agents with required capabilities".to_string())
            })
    }

    /// Best match distribution - selects agent with most matching capabilities
    async fn best_match_distribute(&self, task: &DelegatedTask) -> Result<AgentId, CodexErr> {
        let required_caps: Vec<_> = task.required_capabilities.iter().cloned().collect();
        let agents = self.agent_pool.list_agents().await;

        let mut best_agent: Option<(AgentId, usize)> = None;

        for (agent_id, spec) in agents {
            // Check if agent has all required capabilities
            if !required_caps.iter().all(|cap| spec.has_capability(cap)) {
                continue;
            }

            // Count how many capabilities match
            let match_count = spec.capabilities.len();

            match &mut best_agent {
                None => best_agent = Some((agent_id, match_count)),
                Some((_, current_count)) if match_count > *current_count => {
                    best_agent = Some((agent_id, match_count));
                }
                _ => {}
            }
        }

        best_agent.map(|(agent_id, _)| agent_id).ok_or_else(|| {
            CodexErr::Fatal("No agents found with required capabilities".to_string())
        })
    }

    /// Priority-based distribution
    async fn priority_distribute(&self, task: &DelegatedTask) -> Result<AgentId, CodexErr> {
        let required_caps: Vec<_> = task.required_capabilities.iter().cloned().collect();
        let agents = self.agent_pool.list_agents().await;

        let mut best_agent: Option<(AgentId, i32)> = None;

        for (agent_id, spec) in agents {
            // Check if agent has all required capabilities
            if !required_caps.iter().all(|cap| spec.has_capability(cap)) {
                continue;
            }

            match &mut best_agent {
                None => best_agent = Some((agent_id, spec.priority)),
                Some((_, current_priority)) if spec.priority > *current_priority => {
                    best_agent = Some((agent_id, spec.priority));
                }
                _ => {}
            }
        }

        best_agent.map(|(agent_id, _)| agent_id).ok_or_else(|| {
            CodexErr::Fatal("No agents found with required capabilities".to_string())
        })
    }

    /// Random distribution
    async fn random_distribute(&self, task: &DelegatedTask) -> Result<AgentId, CodexErr> {
        let required_caps: Vec<_> = task.required_capabilities.iter().cloned().collect();
        let agents = self
            .agent_pool
            .find_agents_with_capabilities(&required_caps)
            .await;

        if agents.is_empty() {
            return Err(CodexErr::Fatal(
                "No agents found with required capabilities".to_string(),
            ));
        }

        // Simple random selection using the length and rand
        let mut rng = rand::rng();
        let idx = rng.random_range(0..agents.len());
        Ok(agents[idx].clone())
    }

    /// Get all current task assignments
    pub async fn get_all_assignments(&self) -> HashMap<String, AgentId> {
        let assignments = self.task_assignments.read().await;
        assignments.clone()
    }

    /// Clear all task assignments
    pub async fn clear_assignments(&self) {
        let mut assignments = self.task_assignments.write().await;
        assignments.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::multi_agent::agent_pool::AgentPoolConfig;
    use crate::multi_agent::types::AgentSpec;

    // Note: Full tests would require setting up sessions and services
    // These are simplified unit tests

    #[test]
    fn test_task_distributor_creation() {
        // This test would require a full setup with SessionServices
        // Skipping for now as it requires complex mocking
    }
}
