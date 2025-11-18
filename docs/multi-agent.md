# Multi-Agent Collaboration

Codex now supports multi-agent collaboration, allowing multiple specialized agents to work together on complex tasks.

## Overview

The multi-agent system enables:

- **Agent Pools**: Manage multiple agent instances with different capabilities
- **Task Distribution**: Automatically route tasks to agents based on their skills
- **Event Aggregation**: Collect and merge events from all agents
- **Inter-Agent Communication**: Agents can communicate and coordinate with each other
- **Hierarchical Coordination**: A coordinator agent orchestrates worker agents

## Architecture

### Components

1. **AgentPool**: Manages a collection of agent sessions
2. **CoordinatorTask**: Orchestrates multiple agents to work on tasks
3. **TaskDistributor**: Routes tasks to appropriate agents
4. **EventAggregator**: Collects events from all agents
5. **InterAgentProtocol**: Handles communication between agents

### Agent Capabilities

Agents can be configured with specific capabilities:

- `CodeGeneration`: Writing and modifying code
- `CodeReview`: Reviewing and analyzing code
- `Testing`: Creating and running tests
- `Documentation`: Writing documentation
- `Debugging`: Finding and fixing bugs
- `Architecture`: System design and architecture
- `Research`: Information gathering and research
- `ShellExecution`: Running shell commands
- `FileOperations`: File system operations
- `WebSearch`: Web searching
- `Custom(String)`: Custom capabilities

## Usage

### Basic Example (Rust)

```rust
use codex_core::multi_agent::{
    AgentPool, AgentPoolConfig, AgentSpec, AgentRole, AgentCapability,
    CoordinatorTask, CoordinatorConfig, TaskDistributionStrategy,
};

// Create agent specifications
let code_agent = AgentSpec::new("code-agent", AgentRole::Specialist {
    domain: "coding".to_string()
})
.with_capability(AgentCapability::CodeGeneration)
.with_capability(AgentCapability::Debugging)
.with_max_concurrent_tasks(2);

let review_agent = AgentSpec::new("review-agent", AgentRole::Specialist {
    domain: "review".to_string()
})
.with_capability(AgentCapability::CodeReview)
.with_capability(AgentCapability::Testing)
.with_max_concurrent_tasks(2);

// Create coordinator configuration
let config = CoordinatorConfig {
    pool_config: AgentPoolConfig {
        max_agents: 5,
        max_concurrent_tasks: 10,
        enable_event_aggregation: true,
    },
    distribution_strategy: TaskDistributionStrategy::BestMatch,
    agent_specs: vec![code_agent, review_agent],
    verbose: true,
};

// Create and use coordinator task
let coordinator = CoordinatorTask::new(config);
// The coordinator will automatically delegate tasks to appropriate agents
```

### Distribution Strategies

Choose how tasks are distributed to agents:

1. **RoundRobin**: Distribute tasks evenly across agents
2. **LeastLoaded**: Send tasks to the least busy agent
3. **BestMatch**: Select agent with the most matching capabilities
4. **Priority**: Choose highest priority agent
5. **Random**: Random distribution

### Agent Pool Management

```rust
use codex_core::multi_agent::{AgentPool, AgentPoolConfig};

// Create an agent pool
let (pool, event_rx) = AgentPool::new(
    AgentPoolConfig::default(),
    shared_services,
);

// Add an agent
pool.add_agent(agent_spec, session_config).await?;

// Find agents with specific capabilities
let agents = pool.find_agents_with_capabilities(&[
    AgentCapability::CodeGeneration,
    AgentCapability::Testing,
]).await;

// Get pool statistics
let stats = pool.get_stats().await;
println!("Active agents: {}", stats.total_agents);
println!("Active tasks: {}", stats.total_active_tasks);
```

### Inter-Agent Communication

Agents can communicate using the `InterAgentProtocol`:

```rust
use codex_core::multi_agent::{InterAgentMessage, InterAgentProtocol};

// Create protocol instance
let mut protocol = InterAgentProtocol::new(agent_id);

// Delegate a task
let message = InterAgentProtocol::create_delegate_task_message(
    task,
    context,
);

// Request approval
let approval_msg = InterAgentProtocol::create_approval_request_message(
    request_id,
    from_agent,
    action,
    context,
);

// Send status update
let status_msg = InterAgentProtocol::create_status_update_message(
    agent_id,
    AgentStatus::Busy,
    Some("Processing task".to_string()),
);
```

## Configuration Example

In your Codex configuration, you can define multi-agent setups:

```toml
[multi_agent]
enabled = true
max_agents = 10
distribution_strategy = "best_match"

[[multi_agent.agents]]
id = "backend-specialist"
role = "specialist"
domain = "backend"
capabilities = ["code-generation", "debugging", "testing"]
model = "claude-sonnet-4-5-20250929"
max_concurrent_tasks = 3

[[multi_agent.agents]]
id = "frontend-specialist"
role = "specialist"
domain = "frontend"
capabilities = ["code-generation", "documentation"]
model = "claude-sonnet-4-5-20250929"
max_concurrent_tasks = 2

[[multi_agent.agents]]
id = "reviewer"
role = "specialist"
domain = "review"
capabilities = ["code-review", "testing"]
model = "claude-sonnet-4-5-20250929"
max_concurrent_tasks = 1
```

## Use Cases

### 1. Parallel Development

Multiple agents work on different parts of a project simultaneously:

- Agent A: Implements backend API
- Agent B: Creates frontend components
- Agent C: Writes tests
- Coordinator: Integrates the work

### 2. Code Review Pipeline

Sequential workflow with specialized agents:

1. Code agent writes the implementation
2. Review agent checks for issues
3. Test agent creates comprehensive tests
4. Documentation agent updates docs

### 3. Complex Debugging

Multiple agents investigate different aspects:

- Agent A: Analyzes logs
- Agent B: Reviews recent code changes
- Agent C: Reproduces the issue
- Coordinator: Synthesizes findings

### 4. Research and Implementation

Parallel research and development:

- Research agents gather information
- Architecture agent designs solution
- Implementation agents build it
- Review agents validate quality

## Event Aggregation

When event aggregation is enabled, the coordinator receives events from all agents:

```rust
// Events include the source agent ID
struct AggregatedEvent {
    agent_id: AgentId,
    event: Event,
    timestamp_ms: u64,
}

// Filter events by agent
let filter = EventFilter::new()
    .include_agents(vec![agent1_id, agent2_id])
    .exclude_agents(vec![agent3_id]);
```

## Best Practices

1. **Capability Matching**: Define clear capabilities for each agent
2. **Resource Limits**: Set appropriate max_concurrent_tasks
3. **Event Monitoring**: Enable event aggregation for visibility
4. **Error Handling**: Handle agent failures gracefully
5. **Task Granularity**: Break down large tasks appropriately
6. **Communication**: Use inter-agent messages for coordination

## Performance Considerations

- **Agent Pool Size**: Balance between parallelism and resource usage
- **Task Distribution**: Choose strategy based on workload
- **Event Volume**: Consider filtering events for high-traffic scenarios
- **Shared Services**: MCP, exec, and auth are shared across agents
- **Concurrency Limits**: Configure based on available resources

## Limitations

- Maximum 10 agents by default (configurable)
- Agents share the same session services (MCP, exec, auth)
- Event aggregation adds overhead for high-frequency events
- Task distribution is based on simple capability matching

## Future Enhancements

Planned improvements include:

- Learning-based agent selection
- Dynamic agent scaling
- Distributed agent execution
- Advanced coordination patterns
- Cross-session agent collaboration
- Agent skill learning and improvement

## Examples

See the `examples/` directory for complete examples:

- `multi_agent_basic.rs`: Basic multi-agent setup
- `multi_agent_pipeline.rs`: Sequential pipeline with multiple agents
- `multi_agent_parallel.rs`: Parallel task execution
- `multi_agent_research.rs`: Research and implementation workflow

## Troubleshooting

### Agent Not Receiving Tasks

- Check agent capabilities match task requirements
- Verify agent is not at max_concurrent_tasks limit
- Ensure agent pool is initialized

### Events Not Aggregating

- Confirm enable_event_aggregation is true
- Check event receiver is being consumed
- Verify agents are sending events

### Poor Task Distribution

- Review distribution strategy selection
- Check agent load and capabilities
- Consider priority-based distribution

## API Reference

See the inline documentation in:

- `codex-rs/core/src/multi_agent/mod.rs`
- `codex-rs/core/src/multi_agent/types.rs`
- `codex-rs/core/src/multi_agent/agent_pool.rs`
- `codex-rs/core/src/multi_agent/coordinator_task.rs`
