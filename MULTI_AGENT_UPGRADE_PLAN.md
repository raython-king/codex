# Codex 多Agent系统升级计划

**制定日期:** 2025年11月12日
**目标:** 将单agent系统升级为支持多agent协作的架构
**风险等级:** Phase 1 - 低风险
**预计工期:** 2-3周

---

## 一、升级目标

### 1.1 核心目标
- ✅ 支持同一会话中多个独立的agent
- ✅ 每个agent可以有不同的角色和专业领域
- ✅ Agent之间可以协作完成复杂任务
- ✅ 保持向后兼容性（不破坏现有API）
- ✅ 保留现有的工具系统和审批机制

### 1.2 Phase 1 具体目标（本次实施）
1. **Agent识别系统**: 为每个agent分配唯一ID
2. **Agent注册表**: 管理多个agent实例
3. **路由机制**: 根据agent_id路由请求到对应agent
4. **独立任务队列**: 每个agent有自己的任务队列
5. **共享会话状态**: Agent共享conversation history和认证

---

## 二、架构设计

### 2.1 当前架构（单Agent）

```
User Input → Submission Queue → submission_loop → Session (single active task) → Events
                                       ↓
                                  spawn_task()
                                       ↓
                                   run_turn()
                                       ↓
                                  ToolRouter
```

### 2.2 目标架构（多Agent）

```
User Input (with agent_id) → AgentRegistry → Agent Queue → Agent Executor → Events
                                   ↓
                      Agent 1 (Planner)    Agent 2 (Coder)    Agent 3 (Reviewer)
                           ↓                     ↓                    ↓
                    Independent Tasks    Independent Tasks    Independent Tasks
                           ↓                     ↓                    ↓
                      Shared ToolRouter & SessionServices
```

### 2.3 核心组件设计

#### AgentId
```rust
/// Unique identifier for an agent within a session
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(String);

impl AgentId {
    pub fn new(id: impl Into<String>) -> Self;
    pub fn default_agent() -> Self; // "default" for backward compatibility
}
```

#### AgentConfig
```rust
/// Configuration for a specific agent
pub struct AgentConfig {
    pub id: AgentId,
    pub name: String,
    pub role: AgentRole,
    pub system_prompt: Option<String>,
    pub allowed_tools: Option<Vec<String>>,
    pub max_concurrent_tasks: usize,
}

pub enum AgentRole {
    Planner,      // 规划和分解任务
    Coder,        // 编写代码
    Reviewer,     // 代码审查
    Tester,       // 测试
    Documenter,   // 文档编写
    Generic,      // 通用agent
}
```

#### AgentState
```rust
/// Per-agent state within a session
pub struct AgentState {
    pub id: AgentId,
    pub config: AgentConfig,
    pub active_task: Mutex<Option<ActiveTask>>,
    pub pending_approvals: Mutex<HashMap<String, PendingApproval>>,
    pub task_history: Vec<TaskRecord>,
}
```

#### AgentRegistry
```rust
/// Registry of all agents in a session
pub struct AgentRegistry {
    agents: HashMap<AgentId, Arc<AgentState>>,
    default_agent: AgentId,
}

impl AgentRegistry {
    pub fn new() -> Self;
    pub fn register(&mut self, config: AgentConfig) -> Result<AgentId>;
    pub fn get(&self, id: &AgentId) -> Option<Arc<AgentState>>;
    pub fn default(&self) -> Arc<AgentState>;
}
```

---

## 三、实施步骤

### Phase 1: 基础多Agent框架（本次实施）

#### 步骤1: 扩展协议 (2-3天)
**文件**: `codex-rs/protocol/src/protocol.rs`

**任务**:
- [ ] 在 `Op` 枚举中添加可选的 `agent_id` 字段
- [ ] 在 `Event` 中添加 `agent_id` 字段用于标识事件来源
- [ ] 创建新的 `AgentId` 类型
- [ ] 添加 `RegisterAgent` 和 `UnregisterAgent` 操作
- [ ] 确保向后兼容（agent_id为None时使用默认agent）

**修改示例**:
```rust
// 修改前
pub enum Op {
    UserInput { input: Vec<UserInput> },
    Interrupt,
    // ...
}

// 修改后
pub enum Op {
    UserInput {
        input: Vec<UserInput>,
        #[serde(default)]
        agent_id: Option<AgentId>,
    },
    RegisterAgent {
        config: AgentConfig,
    },
    Interrupt {
        #[serde(default)]
        agent_id: Option<AgentId>,
    },
    // ...
}
```

#### 步骤2: 实现Agent核心结构 (3-4天)
**文件**: `codex-rs/core/src/agent/mod.rs` (新建)

**任务**:
- [ ] 创建 `agent/` 模块目录
- [ ] 实现 `AgentId`, `AgentConfig`, `AgentRole`
- [ ] 实现 `AgentState` 管理单个agent状态
- [ ] 实现 `AgentRegistry` 管理所有agents
- [ ] 添加单元测试

**子文件**:
- `agent/mod.rs` - 模块入口
- `agent/id.rs` - AgentId类型
- `agent/config.rs` - AgentConfig和AgentRole
- `agent/state.rs` - AgentState
- `agent/registry.rs` - AgentRegistry

#### 步骤3: 修改Session结构 (2-3天)
**文件**: `codex-rs/core/src/codex.rs`

**任务**:
- [ ] 在 `Session` 中添加 `agent_registry: AgentRegistry`
- [ ] 修改 `spawn_task()` 接受 `agent_id` 参数
- [ ] 更新 `ActiveTurn` 为 per-agent
- [ ] 确保审批系统支持per-agent keying

**修改示例**:
```rust
pub(crate) struct Session {
    conversation_id: ConversationId,
    tx_event: Sender<Event>,
    state: Mutex<SessionState>,
    agent_registry: Arc<RwLock<AgentRegistry>>,  // 新增
    services: SessionServices,
    next_internal_sub_id: AtomicU64,
}
```

#### 步骤4: 更新Submission Loop (3-4天)
**文件**: `codex-rs/core/src/codex.rs:1259-1340`

**任务**:
- [ ] 修改 `submission_loop` 路由逻辑
- [ ] 添加 `RegisterAgent` / `UnregisterAgent` 处理
- [ ] 根据 `agent_id` 路由到对应agent
- [ ] 处理 `agent_id=None` 的向后兼容

**伪代码**:
```rust
async fn submission_loop(sess: Arc<Session>, config: Arc<Config>, rx_sub: Receiver<Submission>) {
    while let Ok(submission) = rx_sub.recv().await {
        match submission.op {
            Op::RegisterAgent { config } => {
                // 注册新agent
                let agent_id = sess.register_agent(config)?;
                sess.emit_event(Event::AgentRegistered { agent_id });
            }
            Op::UserInput { input, agent_id } => {
                let agent_id = agent_id.unwrap_or_else(|| AgentId::default_agent());
                let agent = sess.get_agent(&agent_id)?;
                // 路由到对应agent
                agent.spawn_task(turn_context, input, RegularTask)?;
            }
            Op::Interrupt { agent_id } => {
                if let Some(aid) = agent_id {
                    // 中断特定agent
                    sess.get_agent(&aid)?.abort_task();
                } else {
                    // 中断所有agents
                    sess.abort_all_agents();
                }
            }
            // ... 其他操作
        }
    }
}
```

#### 步骤5: 更新工具和审批系统 (2-3天)
**文件**:
- `codex-rs/core/src/tools/orchestrator.rs`
- `codex-rs/core/src/tools/sandboxing.rs`

**任务**:
- [ ] 修改审批key包含 `agent_id`
- [ ] 更新 `ApprovalStore` keying逻辑
- [ ] 确保工具执行隔离per-agent
- [ ] 更新审批请求事件包含 `agent_id`

#### 步骤6: 更新事件发射 (1-2天)
**文件**: `codex-rs/core/src/codex.rs` (事件发射相关代码)

**任务**:
- [ ] 确保所有事件包含 `agent_id`
- [ ] 更新 `emit_event()` 自动添加agent上下文
- [ ] 测试事件关联和追踪

#### 步骤7: 集成测试 (2-3天)
**文件**: `codex-rs/core/tests/` (新增测试)

**任务**:
- [ ] 创建多agent基础测试
- [ ] 测试agent注册/注销
- [ ] 测试并发任务执行
- [ ] 测试事件正确路由
- [ ] 测试向后兼容性
- [ ] 测试审批系统隔离

---

### Phase 2: Agent协作机制（未来实施）

#### 步骤8: Agent间通信 (3-4天)
- [ ] 实现 `AgentMessage` 协议
- [ ] 添加 `SendToAgent` 操作
- [ ] 实现消息队列和路由

#### 步骤9: 协调器 (4-5天)
- [ ] 实现 `AgentCoordinator`
- [ ] 任务分配策略
- [ ] 结果聚合机制

#### 步骤10: 高级特性 (未来)
- [ ] 动态创建agent
- [ ] Agent层次结构
- [ ] Agent性能监控
- [ ] Agent负载均衡

---

## 四、技术细节

### 4.1 关键设计决策

#### 1. 共享 vs 独立状态
**决策**: Agent共享SessionServices，但有独立的任务队列

**原因**:
- ✅ 共享: conversation history, auth, MCP connections（减少资源消耗）
- ✅ 独立: 任务执行、审批决策（隔离和并发）

#### 2. 向后兼容策略
**决策**: `agent_id` 为 `Option<AgentId>`，默认值为 "default"

**实现**:
```rust
impl Op {
    pub fn agent_id(&self) -> AgentId {
        self.agent_id.clone().unwrap_or_else(AgentId::default_agent)
    }
}
```

#### 3. 审批隔离
**决策**: 审批key格式 `"{agent_id}:{original_key}"`

**原因**: 不同agent可能对同一命令有不同的审批决策

#### 4. 事件关联
**决策**: 事件包含 `(submission_id, agent_id)`

**原因**: 客户端需要知道事件来自哪个agent

### 4.2 性能考虑

1. **内存开销**: 每个agent约增加 ~100KB (任务队列+状态)
2. **CPU开销**: 多agent并发，需要测试CPU利用率
3. **IO开销**: 共享工具系统，IO不会翻倍

### 4.3 错误处理

1. **Agent不存在**: 返回 `AgentNotFound` 错误
2. **重复注册**: 返回 `AgentAlreadyExists` 错误
3. **任务冲突**: 每个agent独立，不会冲突

---

## 五、测试计划

### 5.1 单元测试
- [ ] `AgentId` 序列化/反序列化
- [ ] `AgentRegistry` 注册/查询/删除
- [ ] `AgentState` 状态管理
- [ ] 向后兼容性（无agent_id的操作）

### 5.2 集成测试
- [ ] 注册2个agent，分别执行任务
- [ ] 同时向2个agent发送输入
- [ ] 中断特定agent
- [ ] 审批隔离（agent1批准，agent2拒绝）
- [ ] 事件正确标记agent_id

### 5.3 性能测试
- [ ] 10个agent并发执行
- [ ] 内存使用监控
- [ ] CPU使用监控
- [ ] 响应延迟测试

---

## 六、风险评估

### 6.1 技术风险

| 风险 | 等级 | 缓解措施 |
|------|------|----------|
| 破坏现有API | 低 | 使用Option确保向后兼容 |
| 并发竞态 | 中 | 详细测试，使用Mutex保护共享状态 |
| 性能下降 | 低 | 基准测试，优化关键路径 |
| 事件顺序混乱 | 中 | 每个agent独立事件流，清晰标记 |

### 6.2 实施风险

| 风险 | 等级 | 缓解措施 |
|------|------|----------|
| 工期延误 | 中 | 分阶段实施，可独立交付 |
| 测试不充分 | 中 | 详细测试计划，代码审查 |
| 文档不足 | 低 | 边实现边写文档 |

---

## 七、交付标准

### 7.1 Phase 1 完成标准
- ✅ 所有代码编译通过，无警告
- ✅ 所有单元测试通过
- ✅ 所有集成测试通过
- ✅ 向后兼容性验证通过
- ✅ 文档完整（API文档 + 使用指南）
- ✅ 示例代码可运行

### 7.2 代码质量标准
- ✅ 遵循Rust最佳实践
- ✅ 通过所有Clippy lints
- ✅ 代码覆盖率 > 80%
- ✅ 无unsafe代码（除非必要且有注释）

---

## 八、时间线

### Week 1: 基础框架
- Day 1-2: 扩展协议 (步骤1)
- Day 3-5: Agent核心结构 (步骤2)

### Week 2: 核心集成
- Day 1-3: 修改Session (步骤3)
- Day 4-5: 更新Submission Loop (步骤4开始)

### Week 3: 完善和测试
- Day 1-2: 完成Submission Loop (步骤4)
- Day 3: 更新工具系统 (步骤5)
- Day 4: 更新事件系统 (步骤6)
- Day 5: 集成测试 (步骤7)

### Week 4: 文档和交付（缓冲）
- 文档编写
- 示例代码
- 性能测试
- Bug修复

---

## 九、成功指标

### 9.1 功能指标
- ✅ 可以注册至少5个不同的agent
- ✅ Agent可以并发执行任务
- ✅ 事件正确标记来源agent
- ✅ 审批系统正确隔离

### 9.2 性能指标
- ✅ 单agent性能不下降 >5%
- ✅ 10个agent并发时内存增长 <50%
- ✅ 事件延迟 <10ms

### 9.3 质量指标
- ✅ 0个破坏性变更
- ✅ 代码覆盖率 >80%
- ✅ 所有测试通过

---

## 十、下一步行动

**立即开始执行Phase 1，步骤1：扩展协议**

执行命令:
```bash
# 1. 创建agent模块目录
mkdir -p codex-rs/core/src/agent

# 2. 开始修改protocol.rs
cd codex-rs/protocol/src

# 3. 添加AgentId类型
# 4. 修改Op和Event枚举
# 5. 运行测试
cargo test --package codex-protocol
```

---

## 附录

### A. 参考文档
- ARCHITECTURE_ANALYSIS_MULTIAGENT.md
- ARCHITECTURE_SUMMARY.txt
- codex-rs/core/src/codex.rs

### B. 相关Issue/PR
- (待创建)

### C. 词汇表
- **Agent**: 独立的对话代理，有自己的角色和任务队列
- **AgentId**: Agent的唯一标识符
- **AgentRegistry**: Agent注册表，管理所有agent
- **Submission**: 用户提交的操作请求
- **Event**: 系统产生的事件，发送给客户端
