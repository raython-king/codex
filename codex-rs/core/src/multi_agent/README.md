# Multi-Agent Collaborative Development Module

## Quick Start

```rust
use codex_core::multi_agent::{MultiAgentTask, MultiAgentConfig};

// Create a multi-agent task
let task = MultiAgentTask::with_defaults();

// Or with custom config
let task = MultiAgentTask::new(MultiAgentConfig {
    max_concurrent_agents: 5,
    task_decomposition_strategy: TaskDecompositionStrategy::Automatic,
    conflict_resolution: ConflictResolution::MergeWithReview,
    ..Default::default()
});
```

## Module Structure

```
multi_agent/
├── mod.rs                  # Module definitions and exports
├── config.rs               # Configuration types
├── coordinator.rs          # Multi-agent orchestration
├── task.rs                 # MultiAgentTask implementation
├── task_decomposer.rs      # Task decomposition logic
├── result_aggregator.rs    # Result aggregation and conflict resolution
├── agent_pool.rs           # Agent lifecycle management
├── tests.rs                # Unit tests
└── README.md               # This file
```

## Key Components

### MultiAgentTask
The main entry point - a `SessionTask` that integrates multi-agent execution into Codex.

### TaskDecomposer
Analyzes user input and breaks it into sub-tasks using various strategies:
- Automatic: Intelligent analysis-based decomposition
- ByFile: Group by file/module boundaries
- ByFunction: Separate frontend, backend, tests, etc.
- ByDependency: Use code dependency graph
- PlannerBased: Use a planning agent

### MultiAgentCoordinator
Orchestrates parallel agent execution:
- Builds execution plan from task dependencies
- Executes tasks in waves (parallel within wave)
- Handles cancellation and errors

### AgentPool
Manages concurrent agent instances:
- Semaphore-based concurrency control
- Agent spawning and lifecycle management
- Result collection

### ResultAggregator
Combines results from multiple agents:
- Conflict detection
- Multiple resolution strategies
- Error aggregation

## Configuration Options

```rust
pub struct MultiAgentConfig {
    // Max concurrent agents (default: 3)
    pub max_concurrent_agents: usize,

    // Task decomposition strategy
    pub task_decomposition_strategy: TaskDecompositionStrategy,

    // Conflict resolution strategy
    pub conflict_resolution: ConflictResolution,

    // Agent timeout in seconds (default: 600)
    pub agent_task_timeout_secs: Option<u64>,

    // Enable agent communication (default: true)
    pub enable_agent_communication: bool,

    // Enable shared context (default: true)
    pub enable_shared_context: bool,
}
```

## Example Use Cases

### 1. Full-Stack Feature Implementation
```
Input: "Build a user authentication system with login UI, API endpoints, and tests"

Decomposition:
- Agent 1: Frontend login form (React/UI)
- Agent 2: Backend API (authentication logic)
- Agent 3: Tests (after 1 & 2 complete)
```

### 2. Multi-Module Refactoring
```
Input: "Refactor authentication module across multiple files"

Decomposition:
- Agent 1: auth/login.rs
- Agent 2: auth/signup.rs
- Agent 3: api/routes.rs
```

### 3. Documentation Generation
```
Input: "Add documentation to all public functions"

Decomposition:
- Agent 1: src/module_a.rs
- Agent 2: src/module_b.rs
- Agent 3: src/module_c.rs
```

## Performance

**Best Case**: 3x speedup with 3 independent tasks
**Typical Case**: 2x speedup with some dependencies
**Overhead**: Small tasks (<1 min) may not benefit

## Testing

Run tests:
```bash
cargo test --package codex-core --lib multi_agent::tests
```

## Documentation

See [Multi-Agent Development Guide](../../../../docs/multi-agent-development.md) for detailed documentation.

## Future Work

- [ ] Inter-agent messaging protocol
- [ ] Shared memory/context
- [ ] Agent specialization
- [ ] Dynamic scaling
- [ ] Cost optimization
- [ ] Advanced merge agent
