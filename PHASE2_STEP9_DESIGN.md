# Phase 2 Step 9: Agent 协调器 - 设计文档

**创建日期:** 2025-11-12
**状态:** 设计中
**依赖:** Step 8 完成 ✅

---

## 📋 概述

Agent 协调器是多 agent 系统的核心组件，负责：
1. **任务分配** - 将任务智能地分配给合适的 agents
2. **结果聚合** - 收集和合并多个 agents 的结果
3. **错误处理** - 处理失败、超时、重试
4. **并发控制** - 管理并发任务数量和资源

---

## 🏗️ 核心架构

### AgentCoordinator 结构

```rust
/// 协调器 - 管理多 agent 任务分配和结果聚合
pub struct AgentCoordinator {
    /// Agent 注册表引用
    registry: Arc<RwLock<AgentRegistry>>,

    /// 任务队列
    task_queue: Arc<Mutex<VecDeque<CoordinatedTask>>>,

    /// 分配策略
    allocation_strategy: Arc<dyn AllocationStrategy + Send + Sync>,

    /// 结果收集器
    result_aggregator: Arc<Mutex<ResultAggregator>>,

    /// 配置
    config: CoordinatorConfig,
}

/// 协调器配置
pub struct CoordinatorConfig {
    /// 最大并发任务数
    pub max_concurrent_tasks: usize,

    /// 任务超时（毫秒）
    pub task_timeout_ms: u64,

    /// 失败重试次数
    pub max_retries: u32,

    /// 是否允许部分成功
    pub allow_partial_success: bool,
}

/// 协调任务 - 需要协调器管理的任务
pub struct CoordinatedTask {
    /// 任务 ID
    pub id: String,

    /// 任务类型
    pub task_type: TaskType,

    /// 任务负载（JSON）
    pub payload: serde_json::Value,

    /// 目标 agent（None = 由协调器选择）
    pub target_agent: Option<AgentId>,

    /// 优先级
    pub priority: TaskPriority,

    /// 超时（毫秒）
    pub timeout_ms: Option<u64>,

    /// 创建时间
    pub created_at: SystemTime,

    /// 状态
    pub status: TaskStatus,
}

/// 任务类型
pub enum TaskType {
    /// 单一任务 - 分配给一个 agent
    Single,

    /// 并行任务 - 分配给多个 agents 并行执行
    Parallel { agent_count: usize },

    /// 顺序任务 - 多个子任务依次执行
    Sequential { subtasks: Vec<CoordinatedTask> },

    /// 广播任务 - 发送给所有 agents
    Broadcast,
}

/// 任务优先级
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// 任务状态
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    Assigned { agent_id: AgentId },
    Running { agent_id: AgentId },
    Completed { result: TaskResult },
    Failed { error: String },
    TimedOut,
    Cancelled,
}

/// 任务结果
#[derive(Debug, Clone)]
pub struct TaskResult {
    /// 任务 ID
    pub task_id: String,

    /// 执行的 agent
    pub agent_id: AgentId,

    /// 结果数据
    pub result: serde_json::Value,

    /// 执行时间（毫秒）
    pub duration_ms: u64,

    /// 完成时间
    pub completed_at: SystemTime,
}
```

---

## 🎯 任务分配策略

### AllocationStrategy Trait

```rust
/// 任务分配策略 - 决定将任务分配给哪个 agent
pub trait AllocationStrategy: Send + Sync {
    /// 为任务选择 agent
    fn select_agent(
        &self,
        task: &CoordinatedTask,
        registry: &AgentRegistry,
    ) -> Result<AgentId, String>;

    /// 策略名称
    fn name(&self) -> &str;
}
```

### 策略 1: Round-Robin

**特点:**
- 轮流分配给所有可用的 agents
- 简单、公平、无状态
- 适合任务类似、agent 能力相同的场景

```rust
pub struct RoundRobinStrategy {
    /// 当前索引（原子操作）
    current_index: AtomicUsize,
}

impl AllocationStrategy for RoundRobinStrategy {
    fn select_agent(&self, task: &CoordinatedTask, registry: &AgentRegistry) -> Result<AgentId, String> {
        let agents = registry.list_agents();
        if agents.is_empty() {
            return Err("No agents available".to_string());
        }

        let index = self.current_index.fetch_add(1, Ordering::SeqCst) % agents.len();
        Ok(agents[index].clone())
    }

    fn name(&self) -> &str {
        "RoundRobin"
    }
}
```

### 策略 2: Load-Based

**特点:**
- 选择当前负载最低的 agent
- 动态平衡负载
- 适合任务耗时不均的场景

```rust
pub struct LoadBasedStrategy;

impl AllocationStrategy for LoadBasedStrategy {
    fn select_agent(&self, task: &CoordinatedTask, registry: &AgentRegistry) -> Result<AgentId, String> {
        let agents = registry.list_agents();
        if agents.is_empty() {
            return Err("No agents available".to_string());
        }

        // 找到活跃任务数最少的 agent
        let mut min_load_agent = None;
        let mut min_load = usize::MAX;

        for agent_id in agents {
            if let Some(agent) = registry.get(&agent_id) {
                let active_tasks = agent.active_tasks();
                if active_tasks < min_load {
                    min_load = active_tasks;
                    min_load_agent = Some(agent_id.clone());
                }
            }
        }

        min_load_agent.ok_or_else(|| "No suitable agent found".to_string())
    }

    fn name(&self) -> &str {
        "LoadBased"
    }
}
```

### 策略 3: Capability-Based

**特点:**
- 根据 agent 的能力（allowed_tools）匹配任务
- 智能选择最适合的 agent
- 适合 agents 有不同专长的场景

```rust
pub struct CapabilityBasedStrategy {
    /// 任务类型到所需工具的映射
    task_capabilities: HashMap<String, Vec<String>>,
}

impl AllocationStrategy for CapabilityBasedStrategy {
    fn select_agent(&self, task: &CoordinatedTask, registry: &AgentRegistry) -> Result<AgentId, String> {
        let agents = registry.list_agents();
        if agents.is_empty() {
            return Err("No agents available".to_string());
        }

        // 获取任务所需的工具
        let required_tools = self.get_required_tools(task);

        // 找到能力匹配的 agent
        for agent_id in agents {
            if let Some(agent) = registry.get(&agent_id) {
                if Self::has_capabilities(&agent, &required_tools) {
                    return Ok(agent_id.clone());
                }
            }
        }

        Err("No agent with required capabilities".to_string())
    }

    fn name(&self) -> &str {
        "CapabilityBased"
    }
}
```

### 策略 4: Priority-Aware

**特点:**
- 考虑任务优先级
- 高优先级任务优先分配给负载低的 agent
- 低优先级任务可以等待

```rust
pub struct PriorityAwareStrategy {
    base_strategy: Box<dyn AllocationStrategy + Send + Sync>,
}

impl AllocationStrategy for PriorityAwareStrategy {
    fn select_agent(&self, task: &CoordinatedTask, registry: &AgentRegistry) -> Result<AgentId, String> {
        // 根据优先级调整策略
        match task.priority {
            TaskPriority::Critical | TaskPriority::High => {
                // 高优先级：选择最空闲的 agent
                LoadBasedStrategy.select_agent(task, registry)
            }
            _ => {
                // 普通/低优先级：使用基础策略
                self.base_strategy.select_agent(task, registry)
            }
        }
    }

    fn name(&self) -> &str {
        "PriorityAware"
    }
}
```

---

## 📦 结果聚合

### ResultAggregator

```rust
/// 结果聚合器 - 收集和合并多个 agent 的结果
pub struct ResultAggregator {
    /// 等待中的任务结果
    pending_results: HashMap<String, Vec<TaskResult>>,

    /// 聚合策略
    strategy: AggregationStrategy,
}

/// 聚合策略
pub enum AggregationStrategy {
    /// 等待所有结果（默认）
    All,

    /// 等待任意一个成功结果
    Any,

    /// 等待至少 N 个结果
    AtLeast(usize),

    /// 多数投票（用于一致性）
    Majority,

    /// 自定义聚合函数
    Custom(Box<dyn Fn(Vec<TaskResult>) -> serde_json::Value + Send + Sync>),
}

impl ResultAggregator {
    /// 添加一个任务结果
    pub fn add_result(&mut self, task_id: &str, result: TaskResult) {
        self.pending_results.entry(task_id.to_string())
            .or_insert_with(Vec::new)
            .push(result);
    }

    /// 检查任务是否完成
    pub fn is_complete(&self, task_id: &str, expected_count: usize) -> bool {
        match &self.strategy {
            AggregationStrategy::All => {
                self.pending_results.get(task_id)
                    .map(|results| results.len() >= expected_count)
                    .unwrap_or(false)
            }
            AggregationStrategy::Any => {
                self.pending_results.get(task_id)
                    .map(|results| !results.is_empty())
                    .unwrap_or(false)
            }
            AggregationStrategy::AtLeast(n) => {
                self.pending_results.get(task_id)
                    .map(|results| results.len() >= *n)
                    .unwrap_or(false)
            }
            _ => false,
        }
    }

    /// 获取聚合结果
    pub fn get_aggregated_result(&self, task_id: &str) -> Option<serde_json::Value> {
        let results = self.pending_results.get(task_id)?;

        match &self.strategy {
            AggregationStrategy::All => {
                // 合并所有结果到数组
                let merged = results.iter()
                    .map(|r| r.result.clone())
                    .collect::<Vec<_>>();
                Some(serde_json::json!(merged))
            }
            AggregationStrategy::Any => {
                // 返回第一个结果
                results.first().map(|r| r.result.clone())
            }
            AggregationStrategy::AtLeast(n) => {
                if results.len() >= *n {
                    let merged = results.iter()
                        .take(*n)
                        .map(|r| r.result.clone())
                        .collect::<Vec<_>>();
                    Some(serde_json::json!(merged))
                } else {
                    None
                }
            }
            AggregationStrategy::Majority => {
                // 多数投票
                Self::majority_vote(results)
            }
            AggregationStrategy::Custom(func) => {
                Some(func(results.clone()))
            }
        }
    }

    /// 清理已完成的任务结果
    pub fn cleanup(&mut self, task_id: &str) {
        self.pending_results.remove(task_id);
    }
}
```

---

## ⚙️ 协调器核心方法

### submit_task

提交任务给协调器：

```rust
impl AgentCoordinator {
    /// 提交任务
    pub async fn submit_task(&self, task: CoordinatedTask) -> Result<String, String> {
        // 1. 验证任务
        self.validate_task(&task)?;

        // 2. 添加到队列
        let mut queue = self.task_queue.lock().await;
        let task_id = task.id.clone();
        queue.push_back(task);
        drop(queue);

        // 3. 触发分配（异步）
        self.trigger_allocation().await?;

        Ok(task_id)
    }

    /// 分配任务给 agent
    async fn allocate_task(&self, task: &mut CoordinatedTask) -> Result<(), String> {
        let registry = self.registry.read().unwrap();

        // 1. 选择 agent
        let agent_id = if let Some(target) = &task.target_agent {
            target.clone()
        } else {
            self.allocation_strategy.select_agent(task, &registry)?
        };

        // 2. 更新任务状态
        task.status = TaskStatus::Assigned { agent_id: agent_id.clone() };

        // 3. 发送消息给 agent
        let message = AgentMessage::new(
            AgentId::new("coordinator"),
            agent_id.clone(),
            MessageType::Request,
            task.payload.clone(),
        );

        // 发送消息（使用 Step 8 实现的消息系统）
        // 实际实现中会调用 Session::send_agent_message

        Ok(())
    }

    /// 等待任务完成
    pub async fn wait_for_task(&self, task_id: &str, timeout_ms: u64) -> Result<TaskResult, String> {
        let start = SystemTime::now();

        loop {
            // 检查超时
            if let Ok(elapsed) = start.elapsed() {
                if elapsed.as_millis() as u64 > timeout_ms {
                    return Err("Task timed out".to_string());
                }
            }

            // 检查结果
            let aggregator = self.result_aggregator.lock().await;
            if aggregator.is_complete(task_id, 1) {
                if let Some(result) = aggregator.get_aggregated_result(task_id) {
                    return Ok(TaskResult {
                        task_id: task_id.to_string(),
                        agent_id: AgentId::new("unknown"), // 实际需要从结果中获取
                        result,
                        duration_ms: start.elapsed().unwrap().as_millis() as u64,
                        completed_at: SystemTime::now(),
                    });
                }
            }
            drop(aggregator);

            // 短暂等待
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }
}
```

---

## 🔄 工作流程

### 单一任务流程

```
Client
  │
  ↓ submit_task(task)
AgentCoordinator
  │
  ├─> Task Queue
  │
  ├─> select_agent() [AllocationStrategy]
  │     │
  │     ├─> RoundRobin
  │     ├─> LoadBased
  │     └─> CapabilityBased
  │
  ├─> send_message(agent, task) [via MessageQueue]
  │
Agent
  │ 执行任务
  ↓
  send_message(coordinator, result)
  │
  ↓
AgentCoordinator
  │
  ├─> ResultAggregator.add_result()
  │
  └─> return result to Client
```

### 并行任务流程

```
Client
  │
  ↓ submit_task(Parallel { agent_count: 3 })
AgentCoordinator
  │
  ├─> 选择 3 个 agents
  │     │
  │     ├─> Agent A
  │     ├─> Agent B
  │     └─> Agent C
  │
  ├─> 并行发送任务
  │
  ↓ 等待所有结果
ResultAggregator
  │
  ├─> [Result A] ✅
  ├─> [Result B] ✅
  └─> [Result C] ✅
  │
  ↓ 聚合结果
AgentCoordinator
  │
  └─> return aggregated result to Client
```

---

## 🧪 测试策略

### 单元测试

1. **AllocationStrategy 测试**
   - RoundRobin 均匀分配
   - LoadBased 选择最空闲 agent
   - CapabilityBased 能力匹配
   - PriorityAware 优先级处理

2. **ResultAggregator 测试**
   - All 策略等待所有结果
   - Any 策略返回第一个结果
   - Majority 投票正确性

3. **TaskQueue 测试**
   - 优先级排序
   - FIFO 在同优先级内

### 集成测试

1. **单任务端到端**
   - 提交 → 分配 → 执行 → 结果返回

2. **多任务并发**
   - 10 个任务同时提交
   - 验证正确分配
   - 验证所有结果返回

3. **错误场景**
   - Agent 失败时重试
   - 任务超时处理
   - 部分结果成功

4. **性能测试**
   - 100 个任务吞吐量
   - 10+ agents 并发性能
   - 延迟测试

---

## 📊 性能目标

| 指标 | 目标 |
|------|------|
| 任务分配延迟 | < 10ms |
| 支持并发任务 | 100+ |
| 支持 agents 数量 | 50+ |
| 结果聚合延迟 | < 5ms |
| 内存占用 | < 100MB (100 任务) |

---

## 🔐 安全考虑

1. **资源限制**
   - 每个 agent 最大并发任务数
   - 总任务队列大小限制
   - 超时强制终止

2. **权限控制**
   - 协调器可以访问所有 agents
   - Agent 不能直接修改协调器状态
   - 任务结果验证

3. **错误隔离**
   - 一个 agent 失败不影响其他
   - 死锁检测和恢复
   - 异常任务自动清理

---

## 🚀 实施计划

### Step 9.1: 设计协调器架构 ✅
- 本文档即为设计成果

### Step 9.2: 实现 AgentCoordinator
- 创建 `core/src/coordinator/mod.rs`
- 实现核心数据结构
- 实现 submit_task, allocate_task

**预计时间:** 2-3 小时

### Step 9.3: 任务分配策略
- 实现 4 种策略
- 策略单元测试

**预计时间:** 2-3 小时

### Step 9.4: 结果聚合机制
- 实现 ResultAggregator
- 支持多种聚合策略

**预计时间:** 1-2 小时

### Step 9.5: 协调器测试
- 集成测试
- 性能测试

**预计时间:** 2-3 小时

**Step 9 总计:** ~10 小时 (1-1.5 天)

---

## ✅ 验收标准

- [ ] AgentCoordinator 完整实现
- [ ] 至少 3 种分配策略可用
- [ ] ResultAggregator 支持 3+ 聚合策略
- [ ] 所有单元测试通过
- [ ] 集成测试通过（单任务、多任务、错误场景）
- [ ] 性能达标（延迟 < 10ms，支持 100+ 并发）
- [ ] 文档完整（API、使用示例）
- [ ] TypeScript 绑定生成

---

**设计版本:** 1.0
**状态:** 完成
**下一步:** 开始实施 Step 9.2

**作者:** Claude (Anthropic)
**日期:** 2025-11-12
