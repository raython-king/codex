/// Agent coordinator for multi-agent task distribution and result aggregation.
///
/// This module provides:
/// - Task management and coordination
/// - Multiple allocation strategies (Round-Robin, Load-Based, Capability-Based, Priority-Aware)
/// - Result aggregation with various strategies
/// - Concurrent task execution support

pub mod aggregator;
pub mod coordinator;
pub mod strategy;
pub mod task;

// Re-export main types for convenience
pub use aggregator::{AggregationStrategy, ResultAggregator};
pub use coordinator::{AgentCoordinator, CoordinatorConfig};
pub use strategy::{
    AllocationStrategy, CapabilityBasedStrategy, LoadBasedStrategy, PriorityAwareStrategy,
    RoundRobinStrategy,
};
pub use task::{
    CoordinatedTask, TaskPriority, TaskResult, TaskStatus, TaskType,
};
