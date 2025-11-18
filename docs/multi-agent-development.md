# Multi-Agent Collaborative Development

## Overview

The multi-agent collaborative development system enables Codex to decompose complex development tasks into smaller sub-tasks and execute them concurrently using multiple AI agents. This dramatically improves performance for large-scale development tasks.

## Features

### 🚀 Key Capabilities

- **Automatic Task Decomposition**: Intelligently breaks down complex tasks into manageable sub-tasks
- **Parallel Execution**: Multiple agents work concurrently on independent sub-tasks
- **Dependency Management**: Respects dependencies between tasks using topological sorting
- **Conflict Resolution**: Handles file conflicts when multiple agents modify the same files
- **Result Aggregation**: Combines results from all agents into a cohesive final output

### 🏗️ Architecture

```
User Request
     ↓
MultiAgentTask (SessionTask)
     ↓
MultiAgentCoordinator
     ↓
TaskDecomposer → SubTasks
     ↓
AgentPool → Multiple Codex Agents (parallel)
     ↓
ResultAggregator
     ↓
Final Result
```

## Components

### 1. MultiAgentTask

A `SessionTask` implementation that integrates multi-agent execution into the Codex task framework.

**Usage:**
```rust
let task = MultiAgentTask::new(MultiAgentConfig {
    max_concurrent_agents: 3,
    task_decomposition_strategy: TaskDecompositionStrategy::Automatic,
    conflict_resolution: ConflictResolution::MergeWithReview,
    ..Default::default()
});
```

### 2. TaskDecomposer

Breaks down complex tasks using different strategies:

- **Automatic**: Analyzes the task and determines optimal decomposition
- **ByFile**: Groups sub-tasks by file or module boundaries
- **ByFunction**: Separates by functionality (frontend, backend, tests, etc.)
- **ByDependency**: Uses dependency graph analysis
- **PlannerBased**: Uses a dedicated planning agent

**Example:**
```rust
let decomposer = TaskDecomposer::new(TaskDecompositionStrategy::Automatic);
let sub_tasks = decomposer.decompose(&input, &context).await?;
```

### 3. MultiAgentCoordinator

Orchestrates the execution of multiple agents:

- Builds execution plan respecting dependencies
- Executes tasks in waves (parallel within wave, sequential between waves)
- Monitors progress and handles cancellation

**Key Methods:**
```rust
coordinator.execute(input, context, cancel_token).await?
```

### 4. AgentPool

Manages a pool of concurrent agents with:

- Semaphore-based concurrency control
- Agent lifecycle management
- Result collection

**Example:**
```rust
let pool = AgentPool::new(
    max_concurrent,
    base_config,
    auth_manager,
    parent_session,
    parent_ctx
);

let handle = pool.spawn_agent(sub_task, cancel_token).await?;
let result = handle.await_result().await?;
```

### 5. ResultAggregator

Combines results from multiple agents:

- Detects file conflicts
- Applies conflict resolution strategies
- Aggregates messages and errors

**Conflict Resolution Strategies:**
- `MergeWithReview`: Merge non-conflicting changes, mark conflicts for review
- `MergeAgent`: Use a dedicated agent to resolve conflicts
- `Sequential`: Prevent conflicts by running sequentially
- `FailOnConflict`: Fail immediately on conflicts
- `LastWriteWins`: Simple last-write-wins (use with caution)

## Configuration

### MultiAgentConfig

```rust
pub struct MultiAgentConfig {
    /// Maximum number of agents running concurrently
    pub max_concurrent_agents: usize,

    /// Strategy for decomposing tasks
    pub task_decomposition_strategy: TaskDecompositionStrategy,

    /// How to handle conflicts
    pub conflict_resolution: ConflictResolution,

    /// Timeout for individual agents (seconds)
    pub agent_task_timeout_secs: Option<u64>,

    /// Enable inter-agent communication
    pub enable_agent_communication: bool,

    /// Enable shared context between agents
    pub enable_shared_context: bool,
}
```

**Defaults:**
```rust
MultiAgentConfig {
    max_concurrent_agents: 3,
    task_decomposition_strategy: TaskDecompositionStrategy::Automatic,
    conflict_resolution: ConflictResolution::MergeWithReview,
    agent_task_timeout_secs: Some(600),
    enable_agent_communication: true,
    enable_shared_context: true,
}
```

## Task Decomposition Example

### Input
```
"Implement a new feature with frontend UI components and backend API endpoints,
and also write comprehensive tests for all functionality"
```

### Decomposed Sub-Tasks

1. **Frontend** (Priority: 5, Can Parallelize: Yes)
   - Target files: `src/frontend/components.tsx`
   - No dependencies

2. **Backend** (Priority: 10, Can Parallelize: Yes)
   - Target files: `src/backend/api.rs`
   - No dependencies

3. **Tests** (Priority: 3, Can Parallelize: No)
   - Target files: `tests/integration_test.rs`
   - Dependencies: Frontend, Backend (blocking)

### Execution Plan

**Wave 0**: Frontend + Backend (parallel)
**Wave 1**: Tests (after frontend and backend complete)

## Dependency Management

The system uses **Kahn's algorithm** for topological sorting to build an execution plan:

```rust
// Example dependency graph
SubTask {
    id: "tests",
    dependencies: vec![
        TaskDependency {
            task_id: "frontend",
            dependency_type: DependencyType::Blocking,
        },
        TaskDependency {
            task_id: "backend",
            dependency_type: DependencyType::Blocking,
        },
    ],
    can_parallelize: false,
}
```

**Dependency Types:**
- `Blocking`: Must complete before this task starts
- `Soft`: Should complete before, but not required
- `DataFlow`: Needs output from the dependent task

## Examples

### Example 1: Simple Multi-Agent Task

```rust
use codex_core::tasks::MultiAgentTask;
use codex_core::multi_agent::MultiAgentConfig;

// Create task with default config
let task = MultiAgentTask::with_defaults();

// Or customize
let task = MultiAgentTask::new(MultiAgentConfig {
    max_concurrent_agents: 5,
    ..Default::default()
});
```

### Example 2: Custom Decomposition Strategy

```rust
let config = MultiAgentConfig {
    task_decomposition_strategy: TaskDecompositionStrategy::ByFunction,
    conflict_resolution: ConflictResolution::MergeAgent,
    ..Default::default()
};
```

### Example 3: Conservative (No Conflicts)

```rust
let config = MultiAgentConfig {
    max_concurrent_agents: 1,  // Sequential execution
    conflict_resolution: ConflictResolution::Sequential,
    ..Default::default()
};
```

## Performance Characteristics

### Best Case: Independent Tasks
- **Speedup**: Near-linear with number of agents
- **Example**: 3 independent tasks with 3 agents = ~3x faster

### Typical Case: Some Dependencies
- **Speedup**: Depends on dependency graph depth
- **Example**: Frontend + Backend (parallel) → Tests (sequential) = ~2x faster

### Worst Case: Linear Dependencies
- **Speedup**: Minimal (sequential execution required)
- **Example**: A → B → C → D = similar to single agent

## Limitations

1. **Memory Usage**: Each agent requires its own context and memory
2. **Token Usage**: Multiple agents increase API token consumption
3. **Coordination Overhead**: Small tasks may not benefit from decomposition
4. **File Conflicts**: Heavy file overlap requires conflict resolution

## Best Practices

1. **Task Size**: Works best for tasks that can be decomposed into 2-5 sub-tasks
2. **File Isolation**: Better performance when sub-tasks modify different files
3. **Clear Boundaries**: Tasks with clear functional boundaries decompose better
4. **Testing Strategy**: Let tests run after implementation for better validation

## Future Enhancements

- [ ] Inter-agent communication protocol
- [ ] Shared context/memory between agents
- [ ] Dynamic agent scaling based on task complexity
- [ ] Intelligent file change tracking
- [ ] Dedicated merge agent implementation
- [ ] Cost/performance optimization
- [ ] Agent specialization (frontend expert, backend expert, etc.)

## Troubleshooting

### Issue: Circular dependency error
**Solution**: Check task dependencies for cycles. Each task should only depend on tasks that don't transitively depend on it.

### Issue: Conflicts detected
**Solution**: Choose appropriate conflict resolution strategy or reduce parallelism.

### Issue: Poor performance
**Solution**: Task may be too simple for multi-agent execution. Use single agent instead.

### Issue: High token usage
**Solution**: Reduce `max_concurrent_agents` or use more conservative decomposition strategy.

## See Also

- [Task System](../codex-rs/core/src/tasks/mod.rs)
- [Sub-Agent Delegation](../codex-rs/core/src/codex_delegate.rs)
- [Tool Parallelization](../codex-rs/core/src/tools/parallel.rs)
