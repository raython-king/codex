//! Multi-Agent Collaborative Development System
//!
//! This module implements a multi-agent architecture that enables multiple AI agents
//! to work together on complex development tasks. The system provides:
//!
//! - Task decomposition and assignment
//! - Agent coordination and synchronization
//! - Result aggregation and conflict resolution
//! - Parallel execution with dependency management
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────┐
//! │         MultiAgentTask (SessionTask)            │
//! │  - Receives complex development task            │
//! │  - Orchestrates multiple sub-agents             │
//! └────────────────┬────────────────────────────────┘
//!                  │
//!                  ▼
//! ┌─────────────────────────────────────────────────┐
//! │        MultiAgentCoordinator                    │
//! │  - Task decomposition strategy                  │
//! │  - Agent scheduling and load balancing          │
//! │  - Result aggregation                           │
//! │  - Conflict resolution                          │
//! └────────────┬────────────────────────────────────┘
//!              │
//!              ▼
//!     ┌────────┴────────┐
//!     │                 │
//!     ▼                 ▼
//! ┌───────┐         ┌───────┐
//! │Agent 1│ ... ... │Agent N│
//! │(Codex)│         │(Codex)│
//! └───────┘         └───────┘
//! ```
//!
//! # Example Usage
//!
//! ```rust,ignore
//! // Create a multi-agent task
//! let task = MultiAgentTask::new(
//!     MultiAgentConfig {
//!         max_concurrent_agents: 3,
//!         task_decomposition_strategy: TaskDecompositionStrategy::Automatic,
//!         conflict_resolution: ConflictResolution::MergeWithReview,
//!     }
//! );
//!
//! // The task automatically:
//! // 1. Analyzes the request
//! // 2. Breaks it into sub-tasks
//! // 3. Assigns sub-tasks to agents
//! // 4. Monitors progress
//! // 5. Aggregates results
//! ```

mod config;
mod coordinator;
mod task;
mod task_decomposer;
mod result_aggregator;
mod agent_pool;

#[cfg(test)]
mod tests;

pub(crate) use config::{MultiAgentConfig, TaskDecompositionStrategy, ConflictResolution};
pub(crate) use coordinator::MultiAgentCoordinator;
pub(crate) use task::MultiAgentTask;
pub(crate) use task_decomposer::{TaskDecomposer, SubTask, TaskDependency};
pub(crate) use result_aggregator::{ResultAggregator, AgentResult};
pub(crate) use agent_pool::{AgentPool, AgentHandle};
