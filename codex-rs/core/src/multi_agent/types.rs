//! Core types for multi-agent collaboration

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;

/// Unique identifier for an agent within a multi-agent system
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(pub String);

impl fmt::Display for AgentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for AgentId {
    fn from(s: String) -> Self {
        AgentId(s)
    }
}

impl From<&str> for AgentId {
    fn from(s: &str) -> Self {
        AgentId(s.to_string())
    }
}

/// Role of an agent in the multi-agent system
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentRole {
    /// Coordinator agent that orchestrates other agents
    Coordinator,
    /// Worker agent that executes specific tasks
    Worker,
    /// Specialized agent for specific domains
    Specialist { domain: String },
    /// General-purpose agent
    GeneralPurpose,
}

/// Capabilities that an agent possesses
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgentCapability {
    /// Code generation and modification
    CodeGeneration,
    /// Code review and analysis
    CodeReview,
    /// Testing and validation
    Testing,
    /// Documentation writing
    Documentation,
    /// Debugging and troubleshooting
    Debugging,
    /// Architecture and design
    Architecture,
    /// Research and information gathering
    Research,
    /// Shell command execution
    ShellExecution,
    /// File system operations
    FileOperations,
    /// Web search and browsing
    WebSearch,
    /// Custom capability
    Custom(String),
}

impl fmt::Display for AgentCapability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentCapability::CodeGeneration => write!(f, "code-generation"),
            AgentCapability::CodeReview => write!(f, "code-review"),
            AgentCapability::Testing => write!(f, "testing"),
            AgentCapability::Documentation => write!(f, "documentation"),
            AgentCapability::Debugging => write!(f, "debugging"),
            AgentCapability::Architecture => write!(f, "architecture"),
            AgentCapability::Research => write!(f, "research"),
            AgentCapability::ShellExecution => write!(f, "shell-execution"),
            AgentCapability::FileOperations => write!(f, "file-operations"),
            AgentCapability::WebSearch => write!(f, "web-search"),
            AgentCapability::Custom(name) => write!(f, "custom:{name}"),
        }
    }
}

/// Specification for an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSpec {
    /// Unique identifier
    pub id: AgentId,
    /// Agent role
    pub role: AgentRole,
    /// Capabilities this agent possesses
    pub capabilities: HashSet<AgentCapability>,
    /// Model to use for this agent (optional, inherits from parent if None)
    pub model: Option<String>,
    /// Custom instructions for this agent
    pub instructions: Option<String>,
    /// Maximum number of concurrent tasks this agent can handle
    pub max_concurrent_tasks: usize,
    /// Priority level (higher = more priority)
    pub priority: i32,
}

impl AgentSpec {
    /// Create a new agent specification
    pub fn new(id: impl Into<AgentId>, role: AgentRole) -> Self {
        Self {
            id: id.into(),
            role,
            capabilities: HashSet::new(),
            model: None,
            instructions: None,
            max_concurrent_tasks: 1,
            priority: 0,
        }
    }

    /// Add a capability to this agent
    pub fn with_capability(mut self, capability: AgentCapability) -> Self {
        self.capabilities.insert(capability);
        self
    }

    /// Add multiple capabilities to this agent
    pub fn with_capabilities(mut self, capabilities: Vec<AgentCapability>) -> Self {
        self.capabilities.extend(capabilities);
        self
    }

    /// Set the model for this agent
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    /// Set custom instructions for this agent
    pub fn with_instructions(mut self, instructions: impl Into<String>) -> Self {
        self.instructions = Some(instructions.into());
        self
    }

    /// Set maximum concurrent tasks
    pub fn with_max_concurrent_tasks(mut self, max: usize) -> Self {
        self.max_concurrent_tasks = max;
        self
    }

    /// Set priority level
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Check if this agent has a specific capability
    pub fn has_capability(&self, capability: &AgentCapability) -> bool {
        self.capabilities.contains(capability)
    }
}

/// A task delegated to a specific agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegatedTask {
    /// Unique task identifier
    pub task_id: String,
    /// Agent assigned to this task
    pub agent_id: AgentId,
    /// Task description
    pub description: String,
    /// Required capabilities for this task
    pub required_capabilities: HashSet<AgentCapability>,
    /// Task priority
    pub priority: i32,
    /// Parent task ID (if this is a subtask)
    pub parent_task_id: Option<String>,
    /// User inputs for this task
    pub inputs: Vec<String>,
}

impl DelegatedTask {
    /// Create a new delegated task
    pub fn new(
        task_id: impl Into<String>,
        agent_id: impl Into<AgentId>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            task_id: task_id.into(),
            agent_id: agent_id.into(),
            description: description.into(),
            required_capabilities: HashSet::new(),
            priority: 0,
            parent_task_id: None,
            inputs: Vec::new(),
        }
    }

    /// Add a required capability
    pub fn with_required_capability(mut self, capability: AgentCapability) -> Self {
        self.required_capabilities.insert(capability);
        self
    }

    /// Set the priority
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Set the parent task ID
    pub fn with_parent_task(mut self, parent_task_id: impl Into<String>) -> Self {
        self.parent_task_id = Some(parent_task_id.into());
        self
    }

    /// Add an input
    pub fn with_input(mut self, input: impl Into<String>) -> Self {
        self.inputs.push(input.into());
        self
    }
}

/// Status of a delegated task
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    /// Task is pending assignment
    Pending,
    /// Task has been assigned to an agent
    Assigned,
    /// Task is currently being executed
    InProgress,
    /// Task completed successfully
    Completed,
    /// Task failed with an error
    Failed { error: String },
    /// Task was cancelled
    Cancelled,
}

/// Result of a delegated task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    /// Task identifier
    pub task_id: String,
    /// Agent that executed the task
    pub agent_id: AgentId,
    /// Final status
    pub status: TaskStatus,
    /// Result message or output
    pub output: Option<String>,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}
