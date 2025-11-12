# Codex Architecture - Key Files Reference

## Core Session & Conversation Loop

| File | Lines | Purpose | Key Components |
|------|-------|---------|-----------------|
| `/home/user/codex/codex-rs/core/src/codex.rs` | 3,192 | Main agent loop and session management | `Codex`, `Session`, `TurnContext`, `submission_loop()`, `run_turn()` |
| `/home/user/codex/codex-rs/core/src/state/session.rs` | 72 | Session-wide mutable state | `SessionState`, history, rate limits |
| `/home/user/codex/codex-rs/core/src/state/turn.rs` | 116 | Turn-scoped state | `ActiveTurn`, `TurnState`, pending approvals |
| `/home/user/codex/codex-rs/core/src/tasks/mod.rs` | 240+ | Task lifecycle management | `SessionTask` trait, task spawning, abort handling |

## Tool System

| File | Purpose | Key Components |
|------|---------|-----------------|
| `/home/user/codex/codex-rs/core/src/tools/mod.rs` | Tool module exports | Format functions, telemetry limits |
| `/home/user/codex/codex-rs/core/src/tools/router.rs` | Tool dispatch | `ToolRouter`, `ToolCall` construction |
| `/home/user/codex/codex-rs/core/src/tools/registry.rs` | Tool registry | `ToolHandler` trait, `ToolRegistry` |
| `/home/user/codex/codex-rs/core/src/tools/context.rs` | Tool invocation context | `ToolInvocation`, `ToolPayload`, `ToolOutput` |
| `/home/user/codex/codex-rs/core/src/tools/orchestrator.rs` | Approval & sandbox | `ToolOrchestrator`, approval flow, retry logic |
| `/home/user/codex/codex-rs/core/src/tools/sandboxing.rs` | Approval/sandbox traits | `ToolRuntime`, `Approvable`, `Sandboxable`, `ApprovalStore` |
| `/home/user/codex/codex-rs/core/src/tools/parallel.rs` | Tool parallelism | `ToolCallRuntime`, serialization gate |
| `/home/user/codex/codex-rs/core/src/tools/spec.rs` | Tool specifications | Tool specs for model context |

### Tool Handlers

| File | Tool Name(s) | Handler |
|------|-------------|---------|
| `/home/user/codex/codex-rs/core/src/tools/handlers/shell.rs` | shell, local_shell | `ShellHandler` |
| `/home/user/codex/codex-rs/core/src/tools/handlers/unified_exec.rs` | unified_exec | `UnifiedExecHandler` |
| `/home/user/codex/codex-rs/core/src/tools/handlers/apply_patch.rs` | apply_patch | `ApplyPatchHandler` |
| `/home/user/codex/codex-rs/core/src/tools/handlers/read_file.rs` | read_file | `ReadFileHandler` |
| `/home/user/codex/codex-rs/core/src/tools/handlers/list_dir.rs` | list_dir | `ListDirHandler` |
| `/home/user/codex/codex-rs/core/src/tools/handlers/grep_files.rs` | grep_files | `GrepFilesHandler` |
| `/home/user/codex/codex-rs/core/src/tools/handlers/mcp.rs` | MCP tools | `McpHandler` |
| `/home/user/codex/codex-rs/core/src/tools/handlers/mcp_resource.rs` | MCP resources | `McpResourceHandler` |
| `/home/user/codex/codex-rs/core/src/tools/handlers/plan.rs` | plan | `PlanHandler` |
| `/home/user/codex/codex-rs/core/src/tools/handlers/test_sync.rs` | test_sync | `TestSyncHandler` |
| `/home/user/codex/codex-rs/core/src/tools/handlers/view_image.rs` | view_image | `ViewImageHandler` |

### Tool Runtimes

| File | Purpose | Runtime Implementation |
|------|---------|------------------------|
| `/home/user/codex/codex-rs/core/src/tools/runtimes/shell.rs` | Shell execution | `ShellRuntime` (ToolRuntime impl) |
| `/home/user/codex/codex-rs/core/src/tools/runtimes/apply_patch.rs` | Patch application | `ApplyPatchRuntime` (ToolRuntime impl) |
| `/home/user/codex/codex-rs/core/src/tools/runtimes/unified_exec.rs` | Unified exec | `UnifiedExecRuntime` |

## Protocol & Events

| File | Purpose | Key Components |
|------|---------|-----------------|
| `/home/user/codex/codex-rs/protocol/src/protocol.rs` | Protocol definition | `Submission`, `Op`, `Event`, `EventMsg` |
| `/home/user/codex/codex-rs/protocol/src/items.rs` | Turn items | `TurnItem` types for granular progress |
| `/home/user/codex/codex-rs/protocol/src/approvals.rs` | Approval events | `ExecApprovalRequestEvent`, `ApplyPatchApprovalRequestEvent` |
| `/home/user/codex/codex-rs/core/src/event_mapping.rs` | Event conversion | Legacy event compatibility |

## Client & Model

| File | Purpose | Key Components |
|------|---------|-----------------|
| `/home/user/codex/codex-rs/core/src/client.rs` | LLM client | `ModelClient`, streaming |
| `/home/user/codex/codex-rs/core/src/chat_completions.rs` | Chat API | OpenAI chat completions |
| `/home/user/codex/codex-rs/core/src/client_common.rs` | Common client logic | `Prompt`, `ResponseStream` |
| `/home/user/codex/codex-rs/core/src/model_provider_info.rs` | Provider abstraction | `ModelProviderInfo`, multiple providers |
| `/home/user/codex/codex-rs/core/src/model_family.rs` | Model families | Model capabilities by family |

## Task Implementations

| File | Task Type | Purpose |
|------|-----------|---------|
| `/home/user/codex/codex-rs/core/src/tasks/regular.rs` | `RegularTask` | Main conversation loop |
| `/home/user/codex/codex-rs/core/src/tasks/compact.rs` | `CompactTask` | History summarization |
| `/home/user/codex/codex-rs/core/src/tasks/undo.rs` | `UndoTask` | Roll back last turn |
| `/home/user/codex/codex-rs/core/src/tasks/review.rs` | `ReviewTask` | Review workflow |
| `/home/user/codex/codex-rs/core/src/tasks/ghost_snapshot.rs` | `GhostSnapshotTask` | Offline analysis fork |
| `/home/user/codex/codex-rs/core/src/tasks/user_shell.rs` | `UserShellCommandTask` | Direct shell execution |

## Supporting Systems

| File | Purpose | Key Components |
|------|---------|-----------------|
| `/home/user/codex/codex-rs/core/src/context_manager/` | History management | Conversation history, token tracking |
| `/home/user/codex/codex-rs/core/src/response_processing.rs` | Response handling | Item processing pipeline |
| `/home/user/codex/codex-rs/core/src/turn_diff_tracker.rs` | Change tracking | Diff tracking within turn |
| `/home/user/codex/codex-rs/core/src/approval.rs` | Approval management | Sandbox assessment, approval caching |
| `/home/user/codex/codex-rs/core/src/sandboxing/` | Sandbox system | Command validation, containment |
| `/home/user/codex/codex-rs/core/src/rollout/` | Session persistence | Conversation recording, resumption |
| `/home/user/codex/codex-rs/core/src/mcp_connection_manager.rs` | MCP integration | Model Context Protocol servers |
| `/home/user/codex/codex-rs/core/src/auth.rs` | Authentication | Auth manager, token handling |

## Configuration

| File | Purpose |
|------|---------|
| `/home/user/codex/codex-rs/core/src/config/mod.rs` | Config loading & management |
| `/home/user/codex/codex-rs/core/src/config_loader/` | Config parsing |
| `/home/user/codex/codex-rs/protocol/src/config_types.rs` | Protocol config types |

---

## Architecture Layers (Top to Bottom)

### Layer 1: Public API
- `Codex::spawn()` - initialize session
- `Codex::submit()` - submit operation
- `Codex::next_event()` - consume event

### Layer 2: Session Management
- `Session` - session container
- `SessionState` - persistent state
- `TurnContext` - per-turn config
- `ActiveTurn` - runtime state

### Layer 3: Task Execution
- `SessionTask` trait - pluggable task types
- Task spawning & lifecycle
- Cancellation handling

### Layer 4: Turn Processing
- `run_turn()` - main loop
- LLM streaming
- Item processing

### Layer 5: Tool System
- `ToolRouter` - dispatch
- `ToolHandler` trait - implementations
- `ToolOrchestrator` - approval/sandbox

### Layer 6: Protocol
- `Submission` - input
- `Event` - output
- Correlation via submission.id

---

## Multi-Agent Modification Impact

### Must Modify
1. **codex.rs** - submission_loop (add agent routing)
2. **codex.rs** - Session struct (add agents registry)
3. **codex.rs** - active_turn (extend or move to agent)
4. **tools/orchestrator.rs** - approval key includes agent_id
5. **tools/sandboxing.rs** - ApprovalStore keyed by (agent_id, key)
6. **protocol.rs** - Op, Event with optional agent_id

### Should Consider
1. **state/turn.rs** - per-agent ActiveTurn/TurnState
2. **rollout** - per-agent event partitioning
3. **tasks/mod.rs** - agent-aware spawning
4. **context_manager** - shared history synchronization

### Likely Unaffected
1. Individual task implementations
2. Individual tool handlers
3. Tool runtime implementations
4. Protocol core types (backward compatible extensions)
5. Client/model code
6. Sandbox/approval trait definitions

---

## Key Abstractions for Extension

### SessionTask Trait
Located: `/home/user/codex/codex-rs/core/src/tasks/mod.rs:68-98`

Allows adding new task types without modifying core loop.

### ToolHandler Trait  
Located: `/home/user/codex/codex-rs/core/src/tools/registry.rs:21-34`

Allows adding new tools without modifying dispatcher.

### ToolRuntime Trait
Located: `/home/user/codex/codex-rs/core/src/tools/sandboxing.rs:179+`

Allows customizing approval/sandbox per tool.

### ModelClient
Located: `/home/user/codex/codex-rs/core/src/client.rs:82-120`

Abstracts LLM provider (OpenAI, OpenRouter, OSS).

---

## Testing & Exploration Files

| File | Purpose |
|------|---------|
| `/home/user/codex/codex-rs/core/tests/` | Integration tests |
| `/home/user/codex/codex-rs/core/tests/suite/client.rs` | Client tests |

