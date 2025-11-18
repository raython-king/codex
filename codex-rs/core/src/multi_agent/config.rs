//! Configuration for multi-agent collaborative system

use serde::{Deserialize, Serialize};

/// Configuration for multi-agent task execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiAgentConfig {
    /// Maximum number of agents that can run concurrently
    pub max_concurrent_agents: usize,

    /// Strategy for decomposing tasks into sub-tasks
    pub task_decomposition_strategy: TaskDecompositionStrategy,

    /// How to handle conflicts when merging results from multiple agents
    pub conflict_resolution: ConflictResolution,

    /// Timeout for individual agent tasks (in seconds)
    pub agent_task_timeout_secs: Option<u64>,

    /// Enable inter-agent communication
    pub enable_agent_communication: bool,

    /// Enable shared context between agents
    pub enable_shared_context: bool,
}

impl Default for MultiAgentConfig {
    fn default() -> Self {
        Self {
            max_concurrent_agents: 3,
            task_decomposition_strategy: TaskDecompositionStrategy::Automatic,
            conflict_resolution: ConflictResolution::MergeWithReview,
            agent_task_timeout_secs: Some(600), // 10 minutes per agent
            enable_agent_communication: true,
            enable_shared_context: true,
        }
    }
}

/// Strategy for decomposing a complex task into sub-tasks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskDecompositionStrategy {
    /// Automatically analyze the task and determine optimal decomposition
    Automatic,

    /// Decompose by file or module boundaries
    ByFile,

    /// Decompose by functionality (frontend, backend, tests, etc.)
    ByFunction,

    /// Decompose by dependency graph
    ByDependency,

    /// Use a planning agent to create the decomposition
    PlannerBased,
}

/// Strategy for resolving conflicts when multiple agents modify the same files
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictResolution {
    /// Merge results automatically and create a review task for conflicts
    MergeWithReview,

    /// Use a dedicated merge agent to resolve conflicts
    MergeAgent,

    /// Execute tasks sequentially to avoid conflicts
    Sequential,

    /// Fail on conflicts and require manual resolution
    FailOnConflict,

    /// Last write wins (dangerous, use with caution)
    LastWriteWins,
}
