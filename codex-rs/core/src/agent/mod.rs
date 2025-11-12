//! Multi-agent support for Codex sessions.
//!
//! This module provides the infrastructure for running multiple agents within
//! a single Codex session. Each agent has its own task queue, configuration,
//! and state, while sharing session-level resources like conversation history
//! and authentication.

pub mod id;
pub mod config;
pub mod state;
pub mod registry;

pub use id::AgentId;
pub use config::{AgentConfig, AgentRole};
pub use state::AgentState;
pub use registry::AgentRegistry;
