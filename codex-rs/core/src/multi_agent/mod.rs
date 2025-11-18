//! Multi-agent collaboration framework
//!
//! This module provides infrastructure for coordinating multiple Codex agents
//! to work together on complex tasks. Key components:
//!
//! - `AgentPool`: Manages a pool of agent sessions
//! - `CoordinatorTask`: SessionTask that orchestrates multiple child agents
//! - `TaskDistributor`: Routes tasks to appropriate agents
//! - `EventAggregator`: Collects and merges events from multiple agents
//! - `InterAgentProtocol`: Communication protocol between agents

pub mod agent_pool;
pub mod coordinator_task;
pub mod event_aggregator;
pub mod inter_agent_protocol;
pub mod task_distributor;
pub mod types;

pub use agent_pool::{AgentPool, AgentPoolConfig};
pub use coordinator_task::CoordinatorTask;
pub use event_aggregator::EventAggregator;
pub use inter_agent_protocol::{InterAgentMessage, InterAgentProtocol};
pub use task_distributor::{TaskDistributionStrategy, TaskDistributor};
pub use types::{AgentCapability, AgentId, AgentRole, AgentSpec, DelegatedTask};
