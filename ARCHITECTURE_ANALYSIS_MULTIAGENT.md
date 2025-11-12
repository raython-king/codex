# Codex Single-Agent Architecture Analysis
## Multi-Agent Upgrade Design Foundation

**Analysis Date:** November 12, 2025  
**Scope:** Very Thorough  
**Key Files Analyzed:** 20+ core files across codex-rs/core/src/

---

## 1. CURRENT AGENT IMPLEMENTATION & ENTRY POINTS

### 1.1 High-Level Architecture Pattern: Queue-Based Async System

The Codex system follows a **Submission Queue (SQ) / Event Queue (EQ)** pattern:
- Single `Codex` struct acts as the public interface
- Asynchronous channel-based communication: submissions → events
- One long-running `submission_loop` task processes all incoming operations
- Session-scoped state managed through `Arc<Session>` and `Mutex`-protected state

**File:** `/home/user/codex/codex-rs/core/src/codex.rs:137-215`

```rust
pub struct Codex {
    pub(crate) next_id: AtomicU64,
    pub(crate) tx_sub: Sender<Submission>,      // Input channel
    pub(crate) rx_event: Receiver<Event>,       // Output channel
}

impl Codex {
    pub async fn spawn(config, auth_manager, conversation_history, session_source) 
        -> CodexResult<CodexSpawnOk>;
    
    pub async fn submit(&self, op: Op) -> CodexResult<String>;
    pub async fn next_event(&self) -> CodexResult<Event>;
}
```

### 1.2 Session: The Core Abstraction

**File:** `/home/user/codex/codex-rs/core/src/codex.rs:251-258`

```rust
pub(crate) struct Session {
    conversation_id: ConversationId,
    tx_event: Sender<Event>,
    state: Mutex<SessionState>,
    active_turn: Mutex<Option<ActiveTurn>>,
    services: SessionServices,
    next_internal_sub_id: AtomicU64,
}
```

- **At most 1 running task per session** (enforced via `active_turn` mutex)
- Can be interrupted by user input
- Manages conversation history, rate limits, MCP connections
- Service layer includes: MCP manager, unified exec manager, notifier, rollout recorder, auth

### 1.3 Turn Context: Request-Scoped Configuration

**File:** `/home/user/codex/codex-rs/core/src/codex.rs:260-280`

```rust
pub(crate) struct TurnContext {
    pub(crate) sub_id: String,
    pub(crate) client: ModelClient,
    pub(crate) cwd: PathBuf,
    pub(crate) approval_policy: AskForApproval,
    pub(crate) sandbox_policy: SandboxPolicy,
    pub(crate) tools_config: ToolsConfig,
    pub(crate) final_output_json_schema: Option<Value>,
    // ... other turn-specific settings
}
```

Created fresh for each turn, can override model, working directory, policies, schemas

### 1.4 Submission Loop: Central Message Dispatcher

**File:** `/home/user/codex/codex-rs/core/src/codex.rs:1259-1340`

```rust
async fn submission_loop(sess: Arc<Session>, config: Arc<Config>, rx_sub: Receiver<Submission>)
```

Processes operations in sequence:
- `Interrupt` - aborts current task
- `OverrideTurnContext` - updates defaults for future turns
- `UserInput` / `UserTurn` - spawns a `RegularTask`
- `ExecApproval` / `PatchApproval` - routes decisions to pending approval handlers
- `Undo` / `Compact` - spawns specialized task types
- `Review` - spawns review thread
- `Shutdown` - graceful termination

**Key Pattern:** One loop, one session, strict serialization of submission handling.

---

## 2. CONVERSATION LOOP (codex.rs)

### 2.1 Turn Execution Flow

**File:** `/home/user/codex/codex-rs/core/src/codex.rs:1872-1964`

```rust
async fn run_turn(
    sess: Arc<Session>,
    turn_context: Arc<TurnContext>,
    turn_diff_tracker: SharedTurnDiffTracker,
    input: Vec<ResponseItem>,
    cancellation_token: CancellationToken,
) -> CodexResult<TurnRunResult>
```

**Retry Loop with Stream Management:**
1. Build tool specs from MCP tools + configured tools
2. Create `Prompt` with tools and input items
3. Call `try_run_turn()` in retry loop:
   - Stream errors trigger exponential backoff (max 3 retries by default)
   - `ContextWindowExceeded` and `UsageLimitReached` are terminal
   - `TurnAborted` and `Interrupted` propagate immediately
4. Collect token usage and rate limits

### 2.2 Core Turn Processing Loop

**File:** `/home/user/codex/codex-rs/core/src/codex.rs:1983-2100+`

```rust
async fn try_run_turn(
    router: Arc<ToolRouter>,
    sess: Arc<Session>,
    turn_context: Arc<TurnContext>,
    turn_diff_tracker: SharedTurnDiffTracker,
    prompt: &Prompt,
    cancellation_token: CancellationToken,
) -> CodexResult<TurnRunResult>
```

**Per-Event Processing:**
1. Stream from LLM via `turn_context.client.stream(prompt)`
2. Use `FuturesOrdered` to process tool calls concurrently (respecting serialization policy)
3. For each response item:
   - **Tool Call:** Route through `ToolCallRuntime::handle_tool_call()`
   - **Non-Tool Item:** Emit `ItemStarted` → `ItemCompleted` events
   - **Error Items:** Respond with function call output (structured error)
4. Collect all processed items with optional responses
5. Return on stream completion or cancellation

### 2.3 Tool Call Dispatch

**File:** `/home/user/codex/codex-rs/core/src/tools/parallel.rs:29-95`

```rust
pub(crate) struct ToolCallRuntime {
    router: Arc<ToolRouter>,
    session: Arc<Session>,
    turn_context: Arc<TurnContext>,
    tracker: SharedTurnDiffTracker,
    parallel_execution: Arc<RwLock<()>>,  // Serialization gate
}

impl ToolCallRuntime {
    pub(crate) fn handle_tool_call(
        &self, call: ToolCall, cancellation_token: CancellationToken
    ) -> impl Future<Output = Result<ResponseInputItem, CodexErr>>
}
```

**Parallelism Control:**
- Parallel tools: acquire read lock (multiple can run)
- Serial tools: acquire write lock (exclusive)
- All tools gated by `tool_call_gate` readiness flag (prevents premature execution)

### 2.4 Task Lifecycle

**File:** `/home/user/codex/codex-rs/core/src/tasks/mod.rs:100-151`

```rust
pub async fn spawn_task<T: SessionTask>(
    self: &Arc<Self>,
    turn_context: Arc<TurnContext>,
    input: Vec<UserInput>,
    task: T,
)
```

**Lifecycle:**
1. Abort any previous task (`abort_all_tasks(TurnAbortReason::Replaced)`)
2. Create cancellation token and done notifier
3. Spawn tokio task that:
   - Executes `task.run(session_ctx, ctx, input, token)`
   - Flushes rollout on completion
   - Emits `TaskComplete` event
   - Notifies done flag
4. Register task in `active_turn` mutex

**Task Types:**
- `RegularTask` - main conversation loop (`run_turn()`)
- `CompactTask` - history summarization
- `UndoTask` - roll back last turn
- `ReviewTask` - specialized review workflow
- `GhostSnapshotTask` - fork for offline analysis
- `UserShellCommandTask` - direct shell execution

---

## 3. TOOL EXECUTION & HANDLING

### 3.1 Tool Router: Registry & Dispatcher

**File:** `/home/user/codex/codex-rs/core/src/tools/router.rs:20-50`

```rust
pub struct ToolRouter {
    registry: ToolRegistry,
    specs: Vec<ConfiguredToolSpec>,
}

impl ToolRouter {
    pub fn from_config(config: &ToolsConfig, mcp_tools: Option<HashMap<String, mcp_types::Tool>>) -> Self
    
    pub fn specs(&self) -> Vec<ToolSpec>
    
    pub fn tool_supports_parallel(&self, tool_name: &str) -> bool
    
    pub fn build_tool_call(session: &Session, item: ResponseItem) 
        -> Result<Option<ToolCall>, FunctionCallError>
}
```

**Tool Call Construction:**
- MCP tools: parse server/tool name → `ToolPayload::Mcp { server, tool, raw_arguments }`
- UnifiedExec: special case for `unified_exec` tool
- Regular function tools: `ToolPayload::Function { arguments }`
- Custom tools: `ToolPayload::Custom { input }`

### 3.2 Tool Handler Trait

**File:** `/home/user/codex/codex-rs/core/src/tools/registry.rs:21-34`

```rust
#[async_trait]
pub trait ToolHandler: Send + Sync {
    fn kind(&self) -> ToolKind;  // Function or Mcp
    
    fn matches_kind(&self, payload: &ToolPayload) -> bool;
    
    async fn handle(&self, invocation: ToolInvocation) -> Result<ToolOutput, FunctionCallError>;
}

pub struct ToolRegistry {
    handlers: HashMap<String, Arc<dyn ToolHandler>>,
}
```

**Available Handlers (codex-rs/core/src/tools/handlers/):**
1. `ShellHandler` - local_shell, shell tools
2. `UnifiedExecHandler` - unified_exec tool (newer abstraction)
3. `ApplyPatchHandler` - apply_patch tool
4. `ReadFileHandler` - read_file tool
5. `ListDirHandler` - list_dir tool
6. `GrepFilesHandler` - grep_files tool
7. `McpHandler` - MCP function tools
8. `McpResourceHandler` - MCP resource tools
9. `PlanHandler` - custom plan tool
10. `TestSyncHandler` - test sync tool
11. `ViewImageHandler` - image viewing

### 3.3 Tool Invocation Context

**File:** `/home/user/codex/codex-rs/core/src/tools/context.rs:23-78`

```rust
pub struct ToolInvocation {
    pub session: Arc<Session>,
    pub turn: Arc<TurnContext>,
    pub tracker: SharedTurnDiffTracker,
    pub call_id: String,
    pub tool_name: String,
    pub payload: ToolPayload,
}

pub enum ToolPayload {
    Function { arguments: String },
    Custom { input: String },
    LocalShell { params: ShellToolCallParams },
    UnifiedExec { arguments: String },
    Mcp { server: String, tool: String, raw_arguments: String },
}

pub enum ToolOutput {
    Function { content: String, content_items: Option<Vec<...>>, success: Option<bool> },
    Mcp { result: Result<CallToolResult, String> },
}
```

### 3.4 Tool Runtimes: Approval & Sandbox Orchestration

**File:** `/home/user/codex/codex-rs/core/src/tools/orchestrator.rs:22-180`

```rust
pub(crate) struct ToolOrchestrator {
    sandbox: SandboxManager,
}

pub async fn run<Rq, Out, T>(
    &mut self, tool: &mut T, req: &Rq, tool_ctx: &ToolCtx, 
    turn_ctx: &TurnContext, approval_policy: AskForApproval
) -> Result<Out, ToolError>
where T: ToolRuntime<Rq, Out>
```

**Orchestration Flow:**
1. **Initial Approval Check** (if required by tool + policy)
   - Assess sandbox risk if needed
   - Send `ExecApprovalRequestEvent` / `ApplyPatchApprovalRequestEvent`
   - Wait for user decision via oneshot channel
   - Cache `ApprovedForSession` decisions

2. **Select Initial Sandbox**
   - Tool preference: Auto/Require/Forbid
   - Escalated tools skip sandbox on first attempt

3. **First Attempt**
   - Execute under selected sandbox

4. **Failure Handling**
   - If sandbox denied and escalation enabled:
     - Re-request approval (unless policy forbids)
     - Retry without sandbox (no re-approval if already approved)

### 3.5 ToolRuntime Trait

**File:** `/home/user/codex/codex-rs/core/src/tools/sandboxing.rs:179+`

```rust
pub(crate) trait ToolRuntime<Rq, Out>: Approvable<Rq> + Sandboxable {
    async fn run(&mut self, req: &Rq, attempt: &SandboxAttempt, tool_ctx: &ToolCtx) 
        -> Result<Out, ToolError>;
}

pub(crate) trait Approvable<Req> {
    type ApprovalKey: Hash + Eq + Clone + Debug + Serialize;
    fn approval_key(&self, req: &Req) -> Self::ApprovalKey;
    fn wants_initial_approval(&self, req: &Req, policy: AskForApproval, sandbox_policy: &SandboxPolicy) -> bool;
    fn wants_escalated_first_attempt(&self, req: &Req) -> bool { false }
    fn should_bypass_approval(&self, policy: AskForApproval, already_approved: bool) -> bool;
    async fn start_approval_async<'a>(&'a mut self, req: &'a Req, ctx: ApprovalCtx<'a>) -> ReviewDecision;
}

pub(crate) trait Sandboxable {
    fn sandbox_preference(&self) -> SandboxablePreference;  // Auto/Require/Forbid
    fn escalate_on_failure(&self) -> bool;
    fn wants_no_sandbox_approval(&self, policy: AskForApproval) -> bool;
}
```

**Concrete Implementations:**
- `ShellRuntime` - shell commands with approval & sandbox
- `ApplyPatchRuntime` - patch application with approval
- MCP tools - delegated to MCP server

---

## 4. PROTOCOL & EVENT SYSTEM

### 4.1 Protocol: Submission → Event Flow

**File:** `/home/user/codex/codex-rs/protocol/src/protocol.rs:50-150`

**Submission (Input):**
```rust
pub struct Submission {
    pub id: String,
    pub op: Op,
}

pub enum Op {
    Interrupt,
    UserInput { items: Vec<UserInput> },
    UserTurn { items, cwd, approval_policy, sandbox_policy, model, effort, summary, ... },
    OverrideTurnContext { cwd, approval_policy, sandbox_policy, model, effort, summary },
    ExecApproval { id: String, decision: ReviewDecision },
    PatchApproval { id: String, decision: ReviewDecision },
    AddToHistory { text: String },
    GetHistoryEntryRequest { offset, log_id },
    ListMcpTools,
    ListCustomPrompts,
    Undo,
    Compact,
    RunUserShellCommand { command },
    Review { review_request },
    Shutdown,
    // ... more
}
```

**Event (Output):**
```rust
pub struct Event {
    pub id: String,  // Submission ID for correlation
    pub msg: EventMsg,
}

pub enum EventMsg {
    SessionConfigured { session_id, model, reasoning_effort, history_log_id, ... },
    ItemStarted { thread_id, turn_id, item },
    ItemCompleted { thread_id, turn_id, item },
    RawResponseItem { item },
    TurnDiff { items, reason },
    FunctionCallOutput { call_id, output, success },
    ExecApprovalRequest { call_id, command, cwd, reason, risk },
    ApplyPatchApprovalRequest { call_id, changes, reason, grant_root },
    TaskComplete { last_agent_message },
    TurnAborted { reason },
    TokenCount { ... },
    Error { message },
    // ... 20+ more event types
}
```

### 4.2 Event Management

**File:** `/home/user/codex/codex-rs/core/src/codex.rs:749-775`

```rust
pub(crate) async fn send_event(&self, turn_context: &TurnContext, msg: EventMsg) {
    let legacy_source = msg.clone();
    let event = Event {
        id: turn_context.sub_id.clone(),
        msg,
    };
    self.send_event_raw(event).await;

    // Emit legacy events for backward compatibility
    let show_raw_agent_reasoning = self.show_raw_agent_reasoning();
    for legacy in legacy_source.as_legacy_events(show_raw_agent_reasoning) {
        let legacy_event = Event {
            id: turn_context.sub_id.clone(),
            msg: legacy,
        };
        self.send_event_raw(legacy_event).await;
    }
}

pub(crate) async fn send_event_raw(&self, event: Event) {
    // Persist to rollout recorder
    let rollout_items = vec![RolloutItem::EventMsg(event.msg.clone())];
    self.persist_rollout_items(&rollout_items).await;
    // Send to client
    if let Err(e) = self.tx_event.send(event).await {
        error!("failed to send tool call event: {e}");
    }
}
```

### 4.3 Approval System

**File:** `/home/user/codex/codex-rs/core/src/codex.rs:828-928`

```rust
pub async fn request_command_approval(
    &self, turn_context: &TurnContext, call_id: String, command: Vec<String>,
    cwd: PathBuf, reason: Option<String>, risk: Option<SandboxCommandAssessment>
) -> ReviewDecision

pub async fn request_patch_approval(
    &self, turn_context: &TurnContext, call_id: String, 
    changes: HashMap<PathBuf, FileChange>, reason: Option<String>, grant_root: Option<PathBuf>
) -> oneshot::Receiver<ReviewDecision>

pub async fn notify_approval(&self, sub_id: &str, decision: ReviewDecision) {
    // Route decision to pending approval channel
}
```

**Key Pattern:**
- Request emits event and creates oneshot channel
- Decision is awaited via channel (blocks tool execution)
- Response routed via `notify_approval()` from submission loop

### 4.4 Turn Item Events (Item-Level Progress)

**File:** `/home/user/codex/codex-rs/core/src/codex.rs:777-798`

```rust
async fn emit_turn_item_started(&self, turn_context: &TurnContext, item: &TurnItem) {
    self.send_event(turn_context, EventMsg::ItemStarted(...)).await;
}

async fn emit_turn_item_completed(&self, turn_context: &TurnContext, item: TurnItem) {
    self.send_event(turn_context, EventMsg::ItemCompleted(...)).await;
}
```

Items represent granular work: reasoning blocks, tool calls, text generation

---

## 5. SESSION & STATE MANAGEMENT

### 5.1 Session-Wide State

**File:** `/home/user/codex/codex-rs/core/src/state/session.rs:1-72`

```rust
pub(crate) struct SessionState {
    pub(crate) session_configuration: SessionConfiguration,  // Current model/policies/cwd
    pub(crate) history: ContextManager,                      // Conversation history
    pub(crate) latest_rate_limits: Option<RateLimitSnapshot>, // Rate limit state
}
```

**Immutable per Session:**
- Conversation ID (UUID)
- Auth manager
- MCP connection manager (lazy-initialized)
- Rollout recorder (history persistence)
- User notifier (desktop notifications)
- OTEL event manager (telemetry)

### 5.2 Turn-Scoped State

**File:** `/home/user/codex/codex-rs/core/src/state/turn.rs:18-107`

```rust
pub(crate) struct ActiveTurn {
    pub(crate) tasks: IndexMap<String, RunningTask>,
    pub(crate) turn_state: Arc<Mutex<TurnState>>,
}

pub(crate) struct TurnState {
    pending_approvals: HashMap<String, oneshot::Sender<ReviewDecision>>,
    pending_input: Vec<ResponseInputItem>,
}
```

**Purpose:**
- Track currently running task(s)
- Buffer approval responses
- Buffer tool outputs for next turn input

### 5.3 History Management

**File:** `/home/user/codex/codex-rs/core/src/context_manager/`

```rust
// Conversation history stored as sequence of ResponseItems
// Updated after each turn, compacted when needed
// Sent to model on next turn as input context
```

- Uses `ContextManager` to track tokens and manage windows
- Auto-compaction triggered on context window approach
- Rollout recorder persists all history to disk

### 5.4 Rollout Recording

**File:** `/home/user/codex/codex-rs/core/src/codex.rs:643-654`

```rust
pub(crate) async fn flush_rollout(&self) {
    let recorder = {
        let guard = self.services.rollout.lock().await;
        guard.clone()
    };
    if let Some(rec) = recorder && let Err(e) = rec.flush().await {
        warn!("failed to flush rollout recorder: {e}");
    }
}
```

- All events, items, and turn contexts persisted to filesystem
- Enables resuming sessions, forking, replaying
- Session ID + conversation ID form primary key

---

## 6. EXISTING AGENT-RELATED ABSTRACTIONS

### 6.1 SessionTask Trait: Pluggable Task Types

**File:** `/home/user/codex/codex-rs/core/src/tasks/mod.rs:68-98`

```rust
#[async_trait]
pub(crate) trait SessionTask: Send + Sync + 'static {
    fn kind(&self) -> TaskKind;
    
    async fn run(
        self: Arc<Self>,
        session: Arc<SessionTaskContext>,
        ctx: Arc<TurnContext>,
        input: Vec<UserInput>,
        cancellation_token: CancellationToken,
    ) -> Option<String>;
    
    async fn abort(&self, session: Arc<SessionTaskContext>, ctx: Arc<TurnContext>) {
        // default: no-op
    }
}
```

**Current Implementations:**
- `RegularTask` - main conversation loop
- `CompactTask` - history compaction
- `UndoTask` - undo last turn
- `ReviewTask` - specialized review workflow
- `GhostSnapshotTask` - fork for analysis
- `UserShellCommandTask` - direct exec

### 6.2 ModelClient: LLM Abstraction

**File:** `/home/user/codex/codex-rs/core/src/client.rs:82-120`

```rust
pub struct ModelClient {
    config: Arc<Config>,
    auth_manager: Option<Arc<AuthManager>>,
    otel_event_manager: OtelEventManager,
    client: CodexHttpClient,
    provider: ModelProviderInfo,
    conversation_id: ConversationId,
    effort: Option<ReasoningEffortConfig>,
    summary: ReasoningSummaryConfig,
    session_source: SessionSource,
}

impl ModelClient {
    pub async fn stream(&self, prompt: &Prompt) -> Result<ResponseStream>
    
    pub fn get_model_context_window(&self) -> Option<i64>
    pub fn get_auto_compact_token_limit(&self) -> Option<i64>
}
```

**Flexibility:**
- Supports multiple providers: OpenAI, OpenRouter, custom OSS models
- Model per turn (via `OverrideTurnContext`)
- Reasoning effort/summary configs

### 6.3 ToolRouter: Polymorphic Tool Dispatch

**File:** `/home/user/codex/codex-rs/core/src/tools/router.rs`

```rust
pub struct ToolRouter {
    registry: ToolRegistry,
    specs: Vec<ConfiguredToolSpec>,
}
```

- Registry maintains mapping: tool name → handler
- Each tool has configurable parallel/serial behavior
- MCP tools dynamically added at runtime

### 6.4 ApprovalStore: Generic Approval Caching

**File:** `/home/user/codex/codex-rs/core/src/tools/sandboxing.rs:28-78`

```rust
pub(crate) struct ApprovalStore {
    map: HashMap<String, ReviewDecision>,
}

pub(crate) async fn with_cached_approval<K, F, Fut>(
    services: &SessionServices,
    key: K,
    fetch: F,
) -> ReviewDecision
where
    K: Serialize + Clone,
    F: FnOnce() -> Fut,
    Fut: Future<Output = ReviewDecision>,
```

- Caches approval decisions keyed by serialized request
- Reuses `ApprovedForSession` across multiple invocations
- Pluggable approval key generation per tool

---

## 7. KEY ARCHITECTURAL CONSTRAINTS & PATTERNS

### 7.1 Single-Agent Constraints

1. **One active task per session at a time**
   - Enforced via `active_turn: Mutex<Option<ActiveTurn>>`
   - New task replaces previous with `Replaced` abort reason
   - Cannot have parallel independent agents

2. **Strict submission ordering**
   - Single `submission_loop` task processes Submissions sequentially
   - No concurrent operation execution at submission level
   - Guarantees linearizable session state transitions

3. **Turn-level parallelism only**
   - Within a single turn: tool calls can run in parallel (respecting serial/parallel modes)
   - Across turns: strictly sequential
   - Serialization gate in `ToolCallRuntime`

4. **Cancellation is broadcast**
   - Single `CancellationToken` per task
   - Aborting task cancels ALL active tools
   - No selective tool cancellation per agent

### 7.2 Extensibility Points

1. **SessionTask implementations**
   - Add new task types without modifying submission loop
   - Must respect task lifecycle (run, abort, TaskComplete event)

2. **ToolHandler implementations**
   - Add new tools as new handler implementations
   - Register in tool registry
   - Can be static or dynamic (MCP)

3. **ToolRuntime trait**
   - Customize approval/sandbox behavior per tool
   - Currently used by shell and apply_patch runtimes

4. **Model providers**
   - `ModelProviderInfo` abstraction allows plugging different LLMs
   - Per-turn model selection via `OverrideTurnContext`

---

## 8. WHAT NEEDS TO BE PRESERVED

### Critical Invariants

1. **Event ordering guarantees**
   - All events for a turn must be correlated via `submission.id`
   - Events must be emitted in causality order
   - Multi-agent system must maintain per-agent event stream ordering

2. **Session isolation**
   - Each session is independent with unique ID
   - State must not leak between sessions
   - Auth, config, and history are per-session

3. **Approval system semantics**
   - Approvals block tool execution (must await decision)
   - Approvals can be cached per session (`ApprovedForSession`)
   - Denial/Abort both terminate tool execution

4. **Rollout persistence**
   - All state changes must be persisted before returning to client
   - Enables resuming, forking, replaying conversations
   - Rollout path is immutable per session

5. **Cancellation semantics**
   - `Interrupt` op should gracefully cancel current task
   - Tool output still collected (don't corrupt state)
   - Timeout-based cancellation respected

### Protected Interfaces

1. **Codex public API** (stable)
   - `spawn()` - session initialization
   - `submit()` / `submit_with_id()` - operation submission
   - `next_event()` - event consumption

2. **Protocol types** (stable)
   - `Op` enum (non-exhaustive, extensible)
   - `Event` / `EventMsg` types
   - `ReviewDecision`, `AskForApproval`, `SandboxPolicy`

3. **Session interface** (internal, but important)
   - `spawn_task()`, `abort_all_tasks()`
   - `send_event()`, `send_event_raw()`
   - `new_turn()`, `update_settings()`

---

## 9. INJECTION POINTS FOR MULTI-AGENT CAPABILITIES

### 9.1 Submission Loop Enhancement

**Current:** Single sequential loop processing one operation at a time

**Multi-Agent Extensions:**
1. **Agent Router**: Map submissions to agent instances
   - Add optional `agent_id` or `agent_name` to `Op`
   - Router selects target agent, creates if needed
   - Maintain registry of active agents per session

2. **Parallel Submission Handling**
   - Spawn independent submission handlers per agent
   - Use `tokio::spawn()` instead of sequential loop
   - Collect responses and emit in consistent order

### 9.2 Task Scheduling

**Current:** One active task per session (stored in `active_turn`)

**Multi-Agent Extensions:**
1. **Task Queue per Agent**
   - Extend `ActiveTurn` → `ActiveAgent { task_queue: VecDeque<RunningTask>, ... }`
   - Support multiple agents with independent task queues
   - Task priority / scheduling policy

2. **Agent Lifecycle**
   ```rust
   struct Agent {
       id: String,
       task_queue: Arc<Mutex<VecDeque<RunningTask>>>,
       turn_context: Arc<TurnContext>,
       state: Arc<Mutex<AgentState>>,
   }
   ```

### 9.3 Event Routing

**Current:** All events flow to single event queue with submission ID correlation

**Multi-Agent Extensions:**
1. **Hierarchical Event ID**
   - Replace `submission.id: String` with `{ agent_id, operation_id }`
   - Clients can filter by agent
   - Maintain causality guarantees per agent

2. **Agent-Scoped Event Queue** (optional)
   - Each agent gets own event channel
   - Multiplexer combines into session-level queue
   - Or: single queue with agent context in each event

### 9.4 State Partitioning

**Current:** Shared session state with global history

**Multi-Agent Extensions:**
1. **Agent-Local Context**
   - Each agent has its own working directory? (or shared?)
   - Agent-specific approval cache
   - Agent-specific tool preferences

2. **Shared vs. Agent-Local State**
   - **Shared:** conversation history, session metadata, auth
   - **Local:** current working directory (per agent), active task, pending approvals

### 9.5 Tool Execution Model

**Current:** Sequential tool calls within a turn, respecting parallel mode

**Multi-Agent Extensions:**
1. **Cross-Agent Tool Coordination**
   - Tools called by different agents might conflict (e.g., file writes)
   - Need coordination layer: distributed locks, transactions, versioning
   - Or: exclusive access per agent (simpler but less parallel)

2. **Agent Affinity**
   - Tools inherit execution context from calling agent
   - Some tools might be agent-restricted (e.g., agent A can only read in /home/alice)

### 9.6 Approval & Sandbox Policy

**Current:** Per-turn approval/sandbox policy override

**Multi-Agent Extensions:**
1. **Agent-Specific Policies**
   - Agents can have different approval policies
   - Some agents always require approval; others never
   - UI can surface which agent is requesting approval

2. **Distributed Approval**
   - Approval request includes agent context
   - User can approve/deny per agent
   - Cache per (agent, request) pair

---

## 10. DESIGN RECOMMENDATIONS FOR MULTI-AGENT UPGRADE

### Phase 1: Minimal Viable Multi-Agent (Low Risk)

**Approach:** Thin shim layer, reuse single-agent logic

```rust
// Agent router inside submission_loop
enum Op {
    // New variant
    AgentOp { agent_id: String, op: OriginalOp },
    // or: add optional agent_id field (backward compatible)
}

struct AgentRegistry {
    agents: HashMap<String, Arc<AgentInstance>>,
}

struct AgentInstance {
    id: String,
    session: Arc<Session>,  // shared
    task_queue: Arc<Mutex<Option<RunningTask>>>,  // dedicated per agent
    state: Arc<Mutex<AgentState>>,
}

// In submission_loop:
match op {
    Op::AgentOp { agent_id, op } => {
        let agent = registry.get_or_create(&agent_id).await;
        agent.handle_operation(op).await;
    }
    // ... existing ops work on "default" agent
}
```

**Advantages:**
- Minimal changes to existing code paths
- Agents still share session state (history, auth)
- Backward compatible (agents are optional)

**Challenges:**
- Still single submission loop (not truly concurrent)
- Agents compete for task slot (one active per agent, but only one running overall)
- Rollout storage might need per-agent partitioning

### Phase 2: Concurrent Agent Execution

**Approach:** Independent submission handling per agent

```rust
struct AgentExecutor {
    agent_id: String,
    rx_sub: Receiver<Op>,
    tx_event: Sender<Event>,
}

async fn agent_executor_loop(agent: Arc<Agent>) {
    while let Ok(op) = agent.rx_sub.recv().await {
        match op {
            Op::UserInput { items } => {
                // agent.spawn_task(RegularTask)
            }
            // ... agent-specific operations
        }
    }
}

// Per-agent spawning in main submission_loop:
for (agent_id, agent) in agents.iter() {
    let (tx_sub, rx_sub) = async_channel::bounded(64);
    agent.rx_sub = rx_sub;
    let agent_clone = Arc::clone(agent);
    tokio::spawn(async move {
        agent_executor_loop(agent_clone).await;
    });
}
```

**Advantages:**
- Truly concurrent agent execution
- Cleaner separation of concerns
- Scales to many agents

**Challenges:**
- Shared session history needs synchronization (mutex)
- Cross-agent tool conflicts require coordination
- Event ordering guarantees harder to maintain

### Phase 3: Advanced Features

- **Agent-to-Agent Communication:** One agent calls a tool provided by another agent
- **Hierarchical Agents:** Parent agents coordinate child agents
- **Dynamic Agent Creation:** Agents can spawn sub-agents on demand
- **Agent Policies:** Different access levels, tool restrictions, approval requirements

---

## 11. SPECIFIC FILES TO MODIFY FOR MULTI-AGENT

| File | Current Role | Multi-Agent Change |
|------|--------------|-------------------|
| `codex.rs:1259-1340` | submission_loop | Add agent routing/multiplexing |
| `codex.rs:251-258` | Session struct | Add agents: HashMap<String, Arc<Agent>> |
| `codex.rs:255` | active_turn | Extend to per-agent or move to Agent struct |
| `codex.rs:700+` | new_turn() | Make agent-aware (whose turn?) |
| `state/turn.rs` | ActiveTurn, TurnState | Replicate per agent or consolidate |
| `tools/orchestrator.rs` | ToolOrchestrator | Extend approval key to include agent_id |
| `tools/sandboxing.rs` | ApprovalStore | Key by (agent_id, approval_key) |
| `protocol.rs` | Op, Event | Add agent_id field (optional, backward compatible) |
| `tasks/mod.rs` | spawn_task() | Make agent-aware |

**Implementation Strategy:**
1. Add `Option<String>` agent_id to relevant Op variants
2. Keep agent_id absent for backward compatibility (defaults to "default")
3. Create AgentRegistry wrapper around Session
4. Refactor submission_loop to delegate to agents
5. Update approval caching to include agent context
6. Update rollout to track agent ID for events

---

## SUMMARY

The current Codex architecture is **well-suited for single-agent, event-driven processing** with clear separation between:
- **Control Flow:** Async submission loop, task-based execution
- **Data Flow:** Event queue with turn-scoped processing
- **Extensibility:** SessionTask trait, ToolHandler trait, ModelClient abstraction

**For multi-agent upgrade:**
- **Low-hanging fruit:** Agent router in submission loop, per-agent task queue
- **Core challenge:** Shared session state (history, auth) vs. agent-local state
- **Key invariant:** Maintain event ordering and causality guarantees per agent
- **Risk mitigation:** Phase in changes, keep single-agent mode as default, add comprehensive tests

The system's abstraction layer (SessionTask, ToolHandler, ToolRuntime traits) provides excellent hooks for multi-agent extension without requiring massive rewrites.
