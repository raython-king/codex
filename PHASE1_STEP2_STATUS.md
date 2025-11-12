# Phase 1 Step 2: 协议扩展 - 状态报告

**日期:** 2025-11-12
**阶段:** Phase 1, Step 2 (部分完成)
**进度:** 约 40%
**状态:** 进行中

---

## ✅ 已完成工作

### 1. AgentId 重构到 Protocol Crate

**文件变更:**
- **新建:** `codex-rs/protocol/src/agent_id.rs` (120行)
- **修改:** `codex-rs/protocol/src/lib.rs`
- **删除:** `codex-rs/core/src/agent/id.rs` (已移至protocol)
- **修改:** `codex-rs/core/src/agent/{mod.rs, config.rs, state.rs, registry.rs}`

**关键成就:**
```rust
// protocol crate 中的 AgentId
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, TS)]
#[serde(transparent)]
#[ts(export)]
pub struct AgentId(pub String);
```

✅ **特性:**
- 完整的序列化支持（Serde, JsonSchema, TS）
- 向后兼容（默认agent: "default"）
- 类型安全（newtype pattern）
- 完整的测试覆盖
- 零依赖冲突

✅ **验证:**
```bash
✅ cargo check --package codex-protocol - PASS
✅ cargo check --package codex-core - PASS
✅ cargo test --package codex-protocol - ALL PASS
✅ cargo test --package codex-core agent - ALL PASS
```

### 2. 依赖关系优化

**问题:** 原先 AgentId 在 core crate，但 protocol 需要使用它（会造成循环依赖）

**解决方案:**
```
protocol (定义 AgentId)
    ↑
    | (使用)
    |
core (重新导出 AgentId)
```

**影响:**
- ✅ 避免了循环依赖
- ✅ Protocol 可以在 Op/Event 中使用 AgentId
- ✅ TypeScript 类型可以自动生成
- ✅ 所有现有代码继续工作

---

## 🔄 进行中工作

### 3. 协议扩展设计

**目标:** 在 Op 和 Event 中添加 agent_id 支持

**设计方案:**

#### A. 需要修改的 Op 变体

```rust
pub enum Op {
    // 修改前
    Interrupt,

    // 修改后
    Interrupt {
        /// Optional agent ID to interrupt. If None, interrupts all agents.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        agent_id: Option<AgentId>,
    },

    // 类似地修改以下变体：
    UserInput {
        items: Vec<UserInput>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        agent_id: Option<AgentId>,
    },

    UserTurn {
        items: Vec<UserInput>,
        cwd: PathBuf,
        // ... 其他字段
        #[serde(skip_serializing_if = "Option::is_none", default)]
        agent_id: Option<AgentId>,
    },

    ExecApproval {
        id: String,
        decision: ReviewDecision,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        agent_id: Option<AgentId>,
    },

    PatchApproval {
        id: String,
        decision: ReviewDecision,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        agent_id: Option<AgentId>,
    },
}
```

#### B. 新增的 Op 变体

```rust
pub enum Op {
    // ... 现有变体

    /// Register a new agent in the session.
    RegisterAgent {
        /// Configuration for the new agent.
        config: AgentConfig,
    },

    /// Unregister an existing agent.
    UnregisterAgent {
        /// ID of the agent to unregister.
        agent_id: AgentId,
    },
}
```

**挑战:** AgentConfig 需要在 protocol 中定义（不能从 core 导入）

**解决方案:** 创建简化的协议级别的 AgentConfig：

```rust
// codex-rs/protocol/src/agent_config.rs
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
pub struct AgentConfig {
    pub id: AgentId,
    pub name: String,
    pub role: String,  // 简化为String而不是enum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_tools: Option<Vec<String>>,
    #[serde(default = "default_max_concurrent_tasks")]
    pub max_concurrent_tasks: usize,
}
```

#### C. 需要修改的 Event

```rust
// ItemStartedEvent
#[derive(Debug, Clone, Deserialize, Serialize, TS, JsonSchema)]
pub struct ItemStartedEvent {
    pub thread_id: ConversationId,
    pub turn_id: String,
    pub item: TurnItem,
    /// Agent that started this item
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub agent_id: Option<AgentId>,
}

// ItemCompletedEvent
#[derive(Debug, Clone, Deserialize, Serialize, TS, JsonSchema)]
pub struct ItemCompletedEvent {
    pub thread_id: ConversationId,
    pub turn_id: String,
    pub item: TurnItem,
    /// Agent that completed this item
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub agent_id: Option<AgentId>,
}

// ExecApprovalRequestEvent, ApplyPatchApprovalRequestEvent 等
```

#### D. 新增的 Event

```rust
pub enum EventMsg {
    // ... 现有变体

    /// Notification that a new agent was registered.
    AgentRegistered(AgentRegisteredEvent),

    /// Notification that an agent was unregistered.
    AgentUnregistered(AgentUnregisteredEvent),
}

#[derive(Debug, Clone, Deserialize, Serialize, TS, JsonSchema)]
pub struct AgentRegisteredEvent {
    pub agent_id: AgentId,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, TS, JsonSchema)]
pub struct AgentUnregisteredEvent {
    pub agent_id: AgentId,
}
```

---

## ⏸️ 待完成工作

### 4. 实现协议修改

**任务列表:**
- [ ] 创建 `codex-rs/protocol/src/agent_config.rs`
- [ ] 修改 Op 枚举（5个变体 + 2个新变体）
- [ ] 修改 Event 枚举和相关结构体
- [ ] 添加新的事件类型
- [ ] 更新序列化测试
- [ ] 运行 TypeScript 类型生成
- [ ] 验证 JSON Schema 生成

**预计代码量:** ~300行修改

### 5. 向后兼容性验证

**测试场景:**
```rust
// Scenario 1: 旧代码（无agent_id）
let op = Op::UserInput {
    items: vec![...],
    agent_id: None,  // 应该使用默认agent
};

// Scenario 2: 新代码（指定agent）
let op = Op::UserInput {
    items: vec![...],
    agent_id: Some(AgentId::new("planner")),
};

// Scenario 3: JSON反序列化（旧格式）
let json = r#"{"type":"user_input","items":[...]}"#;
// 应该成功，agent_id为None
```

### 6. TypeScript 类型更新

**任务:**
```bash
# 生成新的 TypeScript 类型
cd codex-rs/app-server
cargo run -- generate-ts --out /tmp/types

# 验证类型正确
# 检查 AgentId, RegisterAgent, UnregisterAgent 等
```

---

## 📊 整体进度

### Phase 1 总进度: 约 20%

| 步骤 | 状态 | 进度 |
|------|------|------|
| Step 1: Agent基础模块 | ✅ 完成 | 100% |
| Step 2: 协议扩展 | 🔄 进行中 | 40% |
| Step 3: Session集成 | ⏳ 待开始 | 0% |
| Step 4: Submission Loop | ⏳ 待开始 | 0% |
| Step 5: 工具系统 | ⏳ 待开始 | 0% |
| Step 6: 事件系统 | ⏳ 待开始 | 0% |
| Step 7: 测试文档 | ⏳ 待开始 | 0% |

### Step 2 子任务进度: 40%

- ✅ AgentId 移到 protocol (100%)
- ✅ 依赖关系重构 (100%)
- ✅ 设计协议扩展方案 (100%)
- ⏳ 实现 AgentConfig (0%)
- ⏳ 修改 Op 枚举 (0%)
- ⏳ 修改 Event 枚举 (0%)
- ⏳ TypeScript 生成 (0%)
- ⏳ 测试 (0%)

---

## 🔍 技术决策

### 决策 1: AgentId 放在 Protocol Crate

**原因:**
- ✅ Protocol 需要在 Op/Event 中使用 AgentId
- ✅ 避免 protocol → core 循环依赖
- ✅ 支持 TypeScript 类型自动生成
- ✅ 保持 protocol crate 的最小依赖原则

### 决策 2: 使用 Option<AgentId> 确保向后兼容

**原因:**
- ✅ 旧代码不需要修改（agent_id 默认为 None）
- ✅ None 表示使用默认 agent
- ✅ Serde 的 `#[serde(default)]` 支持
- ✅ 不破坏现有 JSON 序列化格式

### 决策 3: Protocol 级别的 AgentConfig

**原因:**
- ✅ Protocol 不能依赖 core
- ✅ 简化配置结构（role 用 String）
- ✅ 只包含序列化必需的字段
- ✅ Core 可以将详细的 AgentConfig 转换为协议级别的

---

## 🚧 阻塞和风险

### 当前无阻塞

### 潜在风险

| 风险 | 等级 | 缓解措施 |
|------|------|----------|
| 大量文件修改导致冲突 | 低 | 小步提交，增量测试 |
| TypeScript 类型生成问题 | 低 | 提前验证，参考现有模式 |
| 向后兼容性破坏 | 中 | 详细测试旧格式，保持Option |
| 协议复杂度增加 | 中 | 清晰文档，示例代码 |

---

## 📝 下一步行动

### 立即行动（继续 Step 2）

1. **创建 AgentConfig in Protocol** (1-2小时)
   ```bash
   touch codex-rs/protocol/src/agent_config.rs
   # 实现简化的 AgentConfig
   ```

2. **修改 Op 枚举** (2-3小时)
   - 添加 agent_id 到5个现有变体
   - 添加 RegisterAgent / UnregisterAgent

3. **修改 Event** (2-3小时)
   - 添加 agent_id 到相关事件
   - 添加新事件类型

4. **测试和验证** (1-2小时)
   - 编译测试
   - 向后兼容性测试
   - TypeScript 生成

**预计完成 Step 2:** 1-2天

---

## 📚 参考文档

- **升级计划:** `MULTI_AGENT_UPGRADE_PLAN.md`
- **架构分析:** `ARCHITECTURE_ANALYSIS_MULTIAGENT.md`
- **进度总览:** `MULTI_AGENT_PROGRESS_SUMMARY.md`

---

## 💬 备注

### 为什么分阶段提交？

这次只提交了 AgentId 重构，而没有完成整个 Step 2，原因是：

1. **风险控制:** AgentId 移动是重要的架构变更，独立验证
2. **增量进展:** 确保每个提交都是可编译、可测试的状态
3. **便于回滚:** 如果后续步骤有问题，可以精确回滚
4. **清晰历史:** Git 历史更清晰，便于代码审查

### 继续开发

要继续 Step 2：
```bash
# 1. 创建 agent_config.rs
vim codex-rs/protocol/src/agent_config.rs

# 2. 修改 protocol.rs
vim codex-rs/protocol/src/protocol.rs

# 3. 测试
cargo test --package codex-protocol
```

---

**更新时间:** 2025-11-12
**作者:** Claude
**版本:** 1.0
