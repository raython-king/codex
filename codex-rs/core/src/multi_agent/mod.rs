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
//! - `AlgorithmSpecialist`: Specialized agents for algorithm engineering
//! - `AlgorithmCoordinator`: Coordinator optimized for algorithm tasks
//! - `CodeSpecialist`: Specialized agents for software development
//! - `CodeCoordinator`: Coordinator optimized for code development tasks
//! - `DeepLearningSpecialist`: Specialized agents for deep learning development
//! - `DeepLearningCoordinator`: Coordinator optimized for deep learning tasks
//! - `FoundationModelSpecialist`: Specialized agents for CV foundation models and scaling laws
//! - `FoundationModelCoordinator`: Coordinator optimized for foundation model tasks

pub mod agent_pool;
pub mod algorithm_coordinator;
pub mod algorithm_specialist;
pub mod code_coordinator;
pub mod code_specialist;
pub mod coordinator_task;
pub mod deep_learning_coordinator;
pub mod deep_learning_specialist;
pub mod event_aggregator;
pub mod foundation_model_coordinator;
pub mod foundation_model_specialist;
pub mod inter_agent_protocol;
pub mod task_distributor;
pub mod types;

pub use agent_pool::{AgentPool, AgentPoolConfig};
pub use algorithm_coordinator::{AlgorithmCoordinator, AlgorithmCoordinatorConfig};
pub use algorithm_specialist::{
    presets as algorithm_presets, AlgorithmAgent, AlgorithmCapability, PerceptionDomain,
};
pub use code_coordinator::{CodeCoordinator, CodeCoordinatorConfig};
pub use code_specialist::{
    presets as code_presets, DevCapability, DevelopmentAgent, DevelopmentDomain,
    ProgrammingLanguage,
};
pub use coordinator_task::CoordinatorTask;
pub use deep_learning_coordinator::{DeepLearningCoordinator, DeepLearningCoordinatorConfig};
pub use deep_learning_specialist::{
    presets as dl_presets, ApplicationDomain, DeepLearningAgent, DeepLearningCapability,
    DeepLearningFramework, ModelArchitecture, TrainingStrategy,
};
pub use event_aggregator::EventAggregator;
pub use foundation_model_coordinator::{
    FoundationModelCoordinator, FoundationModelCoordinatorConfig,
};
pub use foundation_model_specialist::{
    presets as fm_presets, DatasetScale, FoundationModelAgent, FoundationModelArchitecture,
    FoundationModelCapability, ModelScale, PretrainingStrategy,
};
pub use inter_agent_protocol::{InterAgentMessage, InterAgentProtocol};
pub use task_distributor::{TaskDistributionStrategy, TaskDistributor};
pub use types::{AgentCapability, AgentId, AgentRole, AgentSpec, DelegatedTask};
