# Multi-Agent Migration Guide

**Version:** 1.0
**Date:** 2025-11-12
**Applies to:** Phase 1 Multi-Agent Foundation

---

## Overview

This guide helps you adopt multi-agent features in Codex. The great news: **existing code requires zero changes** and continues to work perfectly!

This guide covers:
1. Why your existing code still works (backward compatibility)
2. How to adopt multi-agent features (if you want them)
3. Examples for common scenarios
4. Best practices

---

## ✅ For Existing Code: No Changes Required

### Why It Just Works

All multi-agent fields are **optional**:

```rust
pub enum Op {
    UserInput {
        items: Vec<UserInput>,
        agent_id: Option<AgentId>,  // ← Optional, defaults to None
    },
    // ... other operations
}
```

- `None` = use the default agent (current behavior)
- `Some(agent_id)` = use a specific agent (new feature)

### The Default Agent

A **default agent** is automatically created for every session:
- Always available
- Cannot be unregistered
- Handles all requests when no agent is specified
- Exactly like the old single-agent behavior

### Your Code Before (Still Works!)

```rust
// This code doesn't change at all
codex.submit(Op::UserInput {
    items: vec![UserInput::Text {
        text: "Hello".to_string()
    }],
    // No agent_id field = uses default agent
}).await?;
```

### Your Code After (Identical!)

```rust
// Exactly the same - agent_id is optional
codex.submit(Op::UserInput {
    items: vec![UserInput::Text {
        text: "Hello".to_string()
    }],
    // Still no agent_id = still uses default agent
}).await?;
```

---

## 🚀 Adopting Multi-Agent Features

Ready to use multiple agents? Here's how!

### Step 1: Register Additional Agents

```rust
use codex_protocol::{AgentConfig, AgentId, Op};

// Create a specialized planner agent
let planner_config = AgentConfig {
    id: AgentId::new("planner"),
    name: "Task Planner".to_string(),
    role: "planner".to_string(),
    system_prompt: Some("You are a task planning specialist. Break down complex tasks into steps.".to_string()),
    allowed_tools: Some(vec!["Read".to_string(), "Grep".to_string()]),
    max_concurrent_tasks: 2,
};

// Register it
codex.submit(Op::RegisterAgent {
    config: planner_config
}).await?;

// Wait for confirmation
let event = events.recv().await?;
match event.msg {
    EventMsg::AgentRegistered(e) => {
        println!("Agent {} registered!", e.name);
    }
    EventMsg::Error(e) => {
        eprintln!("Failed to register agent: {}", e.message);
    }
    _ => {}
}
```

### Step 2: Direct Work to Specific Agents

```rust
// Send planning work to the planner
codex.submit(Op::UserInput {
    items: vec![UserInput::Text {
        text: "Plan how to implement the login feature".to_string()
    }],
    agent_id: Some(AgentId::new("planner")),  // ← Specify the agent
}).await?;

// Send coding work to default agent (or another agent)
codex.submit(Op::UserInput {
    items: vec![UserInput::Text {
        text: "Implement the login endpoint".to_string()
    }],
    agent_id: None,  // ← Use default agent
}).await?;
```

### Step 3: Track Which Agent Did What

```rust
// Monitor events to see agent activity
while let Some(event) = event_stream.next().await {
    match event.msg {
        EventMsg::ItemStarted(e) => {
            if let Some(agent_id) = e.agent_id {
                println!("Agent {} started working", agent_id.as_str());
            } else {
                println!("Default agent started working");
            }
        }
        EventMsg::ItemCompleted(e) => {
            if let Some(agent_id) = e.agent_id {
                println!("Agent {} completed work", agent_id.as_str());
            }
        }
        _ => {}
    }
}
```

---

## 📖 Common Scenarios

### Scenario 1: Simple Multi-Agent Workflow

**Use case:** Separate planning from execution

```rust
// 1. Register a planner agent (read-only)
let planner = AgentConfig {
    id: AgentId::new("planner"),
    name: "Planner".to_string(),
    role: "planner".to_string(),
    system_prompt: Some("You analyze and plan tasks. You cannot execute code.".to_string()),
    allowed_tools: Some(vec!["Read".to_string(), "Grep".to_string(), "Glob".to_string()]),
    max_concurrent_tasks: 1,
};
codex.submit(Op::RegisterAgent { config: planner }).await?;

// 2. Register an executor agent (can write and run)
let executor = AgentConfig {
    id: AgentId::new("executor"),
    name: "Executor".to_string(),
    role: "executor".to_string(),
    system_prompt: Some("You implement code based on plans.".to_string()),
    allowed_tools: Some(vec!["Read".to_string(), "Write".to_string(), "Edit".to_string(), "Bash".to_string()]),
    max_concurrent_tasks: 1,
};
codex.submit(Op::RegisterAgent { config: executor }).await?;

// 3. Use them in sequence
// First, planning
codex.submit(Op::UserInput {
    items: vec![UserInput::Text {
        text: "Analyze this codebase and create a plan for adding authentication".to_string()
    }],
    agent_id: Some(AgentId::new("planner")),
}).await?;

// Then, execution (after reviewing the plan)
codex.submit(Op::UserInput {
    items: vec![UserInput::Text {
        text: "Implement the authentication plan from above".to_string()
    }],
    agent_id: Some(AgentId::new("executor")),
}).await?;
```

### Scenario 2: Role-Based Agent Team

**Use case:** Multiple specialized agents for different tasks

```rust
// Create a team of agents
let agents = vec![
    AgentConfig {
        id: AgentId::new("architect"),
        name: "System Architect".to_string(),
        role: "planner".to_string(),
        allowed_tools: Some(vec!["Read".to_string(), "Grep".to_string()]),
        system_prompt: Some("You design system architecture.".to_string()),
        max_concurrent_tasks: 1,
    },
    AgentConfig {
        id: AgentId::new("coder"),
        name: "Code Writer".to_string(),
        role: "coder".to_string(),
        allowed_tools: Some(vec!["Read".to_string(), "Write".to_string(), "Edit".to_string()]),
        system_prompt: Some("You write clean, tested code.".to_string()),
        max_concurrent_tasks: 2,
    },
    AgentConfig {
        id: AgentId::new("reviewer"),
        name: "Code Reviewer".to_string(),
        role: "reviewer".to_string(),
        allowed_tools: Some(vec!["Read".to_string(), "Grep".to_string()]),
        system_prompt: Some("You review code for quality and best practices.".to_string()),
        max_concurrent_tasks: 1,
    },
    AgentConfig {
        id: AgentId::new("tester"),
        name: "Test Engineer".to_string(),
        role: "tester".to_string(),
        allowed_tools: Some(vec!["Read".to_string(), "Write".to_string(), "Bash".to_string()]),
        system_prompt: Some("You write and run comprehensive tests.".to_string()),
        max_concurrent_tasks: 1,
    },
];

// Register all agents
for config in agents {
    codex.submit(Op::RegisterAgent { config }).await?;
}

// Now assign work based on role
codex.submit(Op::UserInput {
    items: vec![UserInput::Text { text: "Design the auth system".to_string() }],
    agent_id: Some(AgentId::new("architect")),
}).await?;

codex.submit(Op::UserInput {
    items: vec![UserInput::Text { text: "Implement the design".to_string() }],
    agent_id: Some(AgentId::new("coder")),
}).await?;

codex.submit(Op::UserInput {
    items: vec![UserInput::Text { text: "Review the implementation".to_string() }],
    agent_id: Some(AgentId::new("reviewer")),
}).await?;

codex.submit(Op::UserInput {
    items: vec![UserInput::Text { text: "Write tests for auth".to_string() }],
    agent_id: Some(AgentId::new("tester")),
}).await?;
```

### Scenario 3: Tool Isolation for Safety

**Use case:** Limit dangerous operations to specific agents

```rust
// Read-only analyst agent - no file modifications or execution
let analyst = AgentConfig {
    id: AgentId::new("analyst"),
    name: "Code Analyst".to_string(),
    role: "generic".to_string(),
    allowed_tools: Some(vec![
        "Read".to_string(),
        "Grep".to_string(),
        "Glob".to_string(),
    ]),
    system_prompt: Some("You analyze code. You cannot modify files or run commands.".to_string()),
    max_concurrent_tasks: 3,
};

// Restricted executor - can modify but with limited tools
let safe_executor = AgentConfig {
    id: AgentId::new("safe-executor"),
    name: "Safe Executor".to_string(),
    role: "executor".to_string(),
    allowed_tools: Some(vec![
        "Read".to_string(),
        "Write".to_string(),
        "Edit".to_string(),
        // Note: No Bash - cannot execute arbitrary commands
    ]),
    system_prompt: Some("You modify files but cannot execute shell commands.".to_string()),
    max_concurrent_tasks: 1,
};

// Unrestricted agent - can do everything (use carefully!)
let admin = AgentConfig {
    id: AgentId::new("admin"),
    name: "Admin Agent".to_string(),
    role: "generic".to_string(),
    allowed_tools: None,  // None = all tools allowed
    system_prompt: Some("You have full system access.".to_string()),
    max_concurrent_tasks: 1,
};

// Register agents
codex.submit(Op::RegisterAgent { config: analyst }).await?;
codex.submit(Op::RegisterAgent { config: safe_executor }).await?;
codex.submit(Op::RegisterAgent { config: admin }).await?;

// Use analyst for safe exploration
codex.submit(Op::UserInput {
    items: vec![UserInput::Text {
        text: "Analyze security vulnerabilities in this codebase".to_string()
    }],
    agent_id: Some(AgentId::new("analyst")),
}).await?;

// If the analyst tries to use Bash, it will be denied
// Only admin can run potentially dangerous commands
```

### Scenario 4: Unregister Agents When Done

**Use case:** Clean up temporary agents

```rust
// Register a temporary agent for a specific task
let temp_agent = AgentConfig::new("temp-worker", "Temporary Worker");
codex.submit(Op::RegisterAgent { config: temp_agent }).await?;

// ... use the agent for work ...

// When done, unregister it
codex.submit(Op::UnregisterAgent {
    agent_id: AgentId::new("temp-worker")
}).await?;

// Verify it's gone
match events.recv().await?.msg {
    EventMsg::AgentUnregistered(e) => {
        println!("Agent {} removed", e.agent_id.as_str());
    }
    _ => {}
}
```

---

## 🛡️ Best Practices

### 1. Use Descriptive Agent Names

```rust
// ❌ Bad - unclear names
AgentId::new("a1")
AgentId::new("agent2")

// ✅ Good - descriptive names
AgentId::new("security-analyzer")
AgentId::new("ui-component-builder")
AgentId::new("database-migrator")
```

### 2. Limit Tools Based on Role

```rust
// ✅ Planner: Read-only tools
allowed_tools: Some(vec!["Read", "Grep", "Glob"])

// ✅ Coder: File modification tools
allowed_tools: Some(vec!["Read", "Write", "Edit", "Grep"])

// ✅ Executor: Everything including shell
allowed_tools: None  // or explicitly list all tools

// ❌ Bad: Giving planners shell access
allowed_tools: Some(vec!["Read", "Bash"])  // Why does a planner need Bash?
```

### 3. Use System Prompts for Specialization

```rust
// ✅ Good - clear role and boundaries
system_prompt: Some("You are a React component specialist. Focus on creating reusable, accessible UI components. Follow React best practices and use TypeScript.")

// ❌ Bad - too vague
system_prompt: Some("You write code.")
```

### 4. Handle Errors Gracefully

```rust
// ✅ Good - handle both success and failure
codex.submit(Op::RegisterAgent { config }).await?;

let event = events.recv().await?;
match event.msg {
    EventMsg::AgentRegistered(e) => {
        println!("✅ Agent {} ready", e.name);
    }
    EventMsg::Error(e) => {
        eprintln!("❌ Registration failed: {}", e.message);
        // Handle failure - maybe retry or use default agent
    }
    _ => {}
}

// ❌ Bad - assuming success
codex.submit(Op::RegisterAgent { config }).await?;
// Immediately try to use it without waiting for confirmation
```

### 5. Don't Unregister the Default Agent

```rust
// ❌ This will fail
codex.submit(Op::UnregisterAgent {
    agent_id: AgentId::default()
}).await?;
// Error: "Cannot unregister default agent"

// ✅ Only unregister custom agents
codex.submit(Op::UnregisterAgent {
    agent_id: AgentId::new("my-custom-agent")
}).await?;
```

### 6. Track Agent Work via Events

```rust
// ✅ Good - know which agent did what
while let Some(event) = events.recv().await {
    match event.msg {
        EventMsg::ItemStarted(e) => {
            println!("Agent {:?} started item {}", e.agent_id, e.item_id);
        }
        EventMsg::ItemCompleted(e) => {
            println!("Agent {:?} completed item {}", e.agent_id, e.item_id);
        }
        _ => {}
    }
}
```

---

## 🔍 Troubleshooting

### Problem: Agent registration fails with "already exists"

**Cause:** Trying to register an agent with a duplicate ID

**Solution:**
```rust
// Make sure agent IDs are unique
let agents = vec![
    AgentId::new("planner-1"),   // ✅ Unique
    AgentId::new("planner-2"),   // ✅ Unique
    AgentId::new("planner-1"),   // ❌ Duplicate!
];
```

### Problem: Cannot unregister default agent

**Cause:** Default agent cannot be removed

**Solution:**
```rust
// Don't try to unregister the default agent
// If you need to customize it, create a new agent instead
```

### Problem: Agent can't use a tool

**Cause:** Tool not in `allowed_tools` list

**Solution:**
```rust
// Check agent's allowed_tools
let config = AgentConfig {
    // ...
    allowed_tools: Some(vec![
        "Read".to_string(),
        "Write".to_string(),
        // Add the tool you need
    ]),
};
```

### Problem: Events don't show agent_id

**Cause:** Using default agent (agent_id may be None or Some(default))

**Solution:**
```rust
// Check for both cases
match event.msg {
    EventMsg::ItemStarted(e) => {
        match e.agent_id {
            Some(id) => println!("Agent {} working", id.as_str()),
            None => println!("Default agent working"),
        }
    }
    _ => {}
}
```

---

## 📋 Quick Reference

### Agent Configuration

```rust
AgentConfig {
    id: AgentId::new("my-agent"),           // Required: Unique ID
    name: "My Agent".to_string(),            // Required: Display name
    role: "generic".to_string(),             // Optional: "planner", "coder", etc.
    system_prompt: Some("...".to_string()),  // Optional: Custom system prompt
    allowed_tools: Some(vec![...]),          // Optional: None = all tools
    max_concurrent_tasks: 2,                 // Optional: Default is 1
}
```

### Operations with Agent Support

```rust
Op::UserInput { items, agent_id }          // Send input to specific agent
Op::UserTurn { ..., agent_id }             // Full turn with agent
Op::Interrupt { agent_id }                 // Interrupt specific or all agents
Op::RegisterAgent { config }               // Register new agent
Op::UnregisterAgent { agent_id }           // Remove agent
Op::ExecApproval { id, decision, agent_id }     // Approve for specific agent
Op::PatchApproval { id, decision, agent_id }    // Approve patch for agent
```

### Events with Agent Tracking

```rust
EventMsg::AgentRegistered { agent_id, name }    // Agent was registered
EventMsg::AgentUnregistered { agent_id }        // Agent was removed
EventMsg::ItemStarted { ..., agent_id }         // Agent started work
EventMsg::ItemCompleted { ..., agent_id }       // Agent completed work
// ... and more
```

---

## 🎓 Learning Path

### Level 1: Keep It Simple (No Changes)
- Continue using Codex as before
- Everything works with the default agent
- Zero learning curve

### Level 2: Add One Specialized Agent
- Register one agent for a specific task
- Start using `agent_id` in operations
- Learn event tracking

### Level 3: Multi-Agent Teams
- Register 3-5 specialized agents
- Route different tasks to different agents
- Implement tool isolation

### Level 4: Advanced Patterns
- Dynamic agent creation/removal
- Complex workflows
- Custom coordination

---

## ✅ Checklist for Migration

If you want to adopt multi-agent features:

- [ ] Identify tasks that would benefit from specialization
- [ ] Design agent roles (planner, coder, reviewer, etc.)
- [ ] Define tool permissions for each role
- [ ] Write system prompts for each agent
- [ ] Create `AgentConfig` for each agent
- [ ] Register agents at session start
- [ ] Update operations to specify `agent_id` where needed
- [ ] Track events to monitor agent activity
- [ ] Test agent isolation and permissions
- [ ] Clean up temporary agents when done

---

## 🆘 Need Help?

### Documentation
- Read `PHASE1_COMPLETE.md` for full details
- Check `MULTI_AGENT_UPGRADE_PLAN.md` for architecture
- Review code examples in `codex-rs/core/src/agent/`

### Testing
- Run integration tests: `cargo test multi_agent_lifecycle`
- Run all tests: `cargo test --workspace`
- Check examples in test files

### Support
- File issues on the repository
- Check API documentation in code

---

## 🎉 Conclusion

Multi-agent support in Codex is **100% backward compatible**. Your existing code works without changes, and you can adopt multi-agent features incrementally when you're ready.

Start small, experiment, and enjoy the power of specialized agents working together!

---

**Document Version:** 1.0
**Last Updated:** 2025-11-12
**Applies To:** Phase 1 Multi-Agent Foundation
