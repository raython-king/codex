/// Agent coordinator - manages task distribution and result aggregation.
///
/// The coordinator is responsible for:
/// - Distributing tasks to agents using allocation strategies
/// - Collecting and aggregating results from multiple agents
/// - Handling task timeouts and retries
/// - Managing concurrent task execution

use crate::agent::{AgentId, AgentRegistry};
use crate::coordinator::{
    aggregator::{AggregationStrategy, ResultAggregator},
    strategy::AllocationStrategy,
    task::{CoordinatedTask, TaskPriority, TaskResult, TaskStatus, TaskType},
};
use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use tokio::sync::Mutex;

/// Coordinator configuration
#[derive(Debug, Clone)]
pub struct CoordinatorConfig {
    /// Maximum number of concurrent tasks
    pub max_concurrent_tasks: usize,

    /// Default task timeout in milliseconds
    pub task_timeout_ms: u64,

    /// Maximum retry attempts for failed tasks
    pub max_retries: u32,

    /// Allow partial success in parallel tasks
    pub allow_partial_success: bool,
}

impl Default for CoordinatorConfig {
    fn default() -> Self {
        Self {
            max_concurrent_tasks: 10,
            task_timeout_ms: 30000, // 30 seconds
            max_retries: 3,
            allow_partial_success: false,
        }
    }
}

/// Agent coordinator - manages multi-agent task execution
pub struct AgentCoordinator {
    /// Reference to agent registry
    registry: Arc<RwLock<AgentRegistry>>,

    /// Task queue (priority-based)
    task_queue: Arc<Mutex<VecDeque<CoordinatedTask>>>,

    /// Allocation strategy
    allocation_strategy: Arc<dyn AllocationStrategy + Send + Sync>,

    /// Result aggregator
    result_aggregator: Arc<Mutex<ResultAggregator>>,

    /// Configuration
    config: CoordinatorConfig,
}

impl AgentCoordinator {
    /// Creates a new agent coordinator
    pub fn new(
        registry: Arc<RwLock<AgentRegistry>>,
        allocation_strategy: Arc<dyn AllocationStrategy + Send + Sync>,
        config: CoordinatorConfig,
    ) -> Self {
        Self {
            registry,
            task_queue: Arc::new(Mutex::new(VecDeque::new())),
            allocation_strategy,
            result_aggregator: Arc::new(Mutex::new(ResultAggregator::new_all())),
            config,
        }
    }

    /// Creates a coordinator with default configuration
    pub fn with_strategy(
        registry: Arc<RwLock<AgentRegistry>>,
        strategy: Arc<dyn AllocationStrategy + Send + Sync>,
    ) -> Self {
        Self::new(registry, strategy, CoordinatorConfig::default())
    }

    /// Submits a task to the coordinator
    pub async fn submit_task(&self, task: CoordinatedTask) -> Result<String, String> {
        // Validate task
        self.validate_task(&task)?;

        // Add to queue
        let mut queue = self.task_queue.lock().await;
        let task_id = task.id.clone();

        // Insert based on priority (simple priority queue)
        let insert_pos = queue
            .iter()
            .position(|t| t.priority < task.priority)
            .unwrap_or(queue.len());

        queue.insert(insert_pos, task);
        drop(queue);

        // Process queue (async)
        self.process_queue().await?;

        Ok(task_id)
    }

    /// Validates a task before submission
    fn validate_task(&self, task: &CoordinatedTask) -> Result<(), String> {
        // Check task ID is not empty
        if task.id.is_empty() {
            return Err("Task ID cannot be empty".to_string());
        }

        // Validate task type
        match &task.task_type {
            TaskType::Parallel { agent_count } if *agent_count == 0 => {
                return Err("Parallel task must have agent_count > 0".to_string());
            }
            TaskType::Sequential { subtasks } if subtasks.is_empty() => {
                return Err("Sequential task must have at least one subtask".to_string());
            }
            _ => {}
        }

        Ok(())
    }

    /// Processes the task queue
    async fn process_queue(&self) -> Result<(), String> {
        let mut queue = self.task_queue.lock().await;

        while let Some(mut task) = queue.pop_front() {
            // Check if we've reached max concurrent tasks
            // (In a real implementation, this would check active task count)

            // Allocate task based on type
            match task.task_type.clone() {
                TaskType::Single => {
                    self.allocate_single_task(&mut task).await?;
                }
                TaskType::Parallel { agent_count } => {
                    self.allocate_parallel_task(&mut task, agent_count).await?;
                }
                TaskType::Broadcast => {
                    self.allocate_broadcast_task(&mut task).await?;
                }
                TaskType::Sequential { subtasks } => {
                    self.allocate_sequential_task(&mut task, subtasks).await?;
                }
            }

            // For now, we process one task at a time
            // In a production system, this would be more sophisticated
            break;
        }

        Ok(())
    }

    /// Allocates a single task to one agent
    async fn allocate_single_task(&self, task: &mut CoordinatedTask) -> Result<(), String> {
        let registry = self.registry.read().unwrap();

        // Select agent
        let agent_id = if let Some(target) = &task.target_agent {
            target.clone()
        } else {
            self.allocation_strategy.select_agent(task, &registry)?
        };

        // Update task status
        task.status = TaskStatus::Assigned {
            agent_id: agent_id.clone(),
        };

        // In a real implementation, we would:
        // 1. Send a message to the agent via MessageQueue
        // 2. Wait for the response
        // 3. Add the result to the aggregator

        Ok(())
    }

    /// Allocates a parallel task to multiple agents
    async fn allocate_parallel_task(
        &self,
        task: &mut CoordinatedTask,
        agent_count: usize,
    ) -> Result<(), String> {
        let registry = self.registry.read().unwrap();
        let agents = registry.list_agents();

        if agents.len() < agent_count {
            return Err(format!(
                "Not enough agents: need {}, have {}",
                agent_count,
                agents.len()
            ));
        }

        // Allocate to multiple agents
        for i in 0..agent_count {
            // In a real implementation, create subtasks and allocate them
            // For now, we just verify we have enough agents
        }

        Ok(())
    }

    /// Allocates a broadcast task to all agents
    async fn allocate_broadcast_task(&self, task: &mut CoordinatedTask) -> Result<(), String> {
        let registry = self.registry.read().unwrap();
        let agents = registry.list_agents();

        if agents.is_empty() {
            return Err("No agents available for broadcast".to_string());
        }

        // Broadcast to all agents
        // In a real implementation, send to all agents

        Ok(())
    }

    /// Allocates a sequential task with dependent subtasks
    async fn allocate_sequential_task(
        &self,
        _task: &mut CoordinatedTask,
        subtasks: Vec<CoordinatedTask>,
    ) -> Result<(), String> {
        // Execute subtasks one by one
        for _subtask in subtasks {
            // In a real implementation, wait for each subtask to complete
            // before starting the next one
        }

        Ok(())
    }

    /// Waits for a task to complete
    pub async fn wait_for_task(
        &self,
        task_id: &str,
        timeout_ms: u64,
    ) -> Result<TaskResult, String> {
        let start = std::time::SystemTime::now();

        loop {
            // Check timeout
            if let Ok(elapsed) = start.elapsed() {
                if elapsed.as_millis() as u64 > timeout_ms {
                    return Err("Task timed out".to_string());
                }
            }

            // Check if result is available
            let aggregator = self.result_aggregator.lock().await;
            if aggregator.is_complete(task_id, 1) {
                if let Some(result_value) = aggregator.get_aggregated_result(task_id) {
                    let result = TaskResult::new(
                        task_id.to_string(),
                        AgentId::new("unknown"),
                        result_value,
                        start.elapsed().unwrap().as_millis() as u64,
                    );
                    return Ok(result);
                }
            }
            drop(aggregator);

            // Wait before checking again
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }

    /// Adds a result for a task
    pub async fn add_task_result(&self, task_id: &str, result: TaskResult) {
        let mut aggregator = self.result_aggregator.lock().await;
        aggregator.add_result(task_id, result);
    }

    /// Gets pending task count
    pub async fn pending_task_count(&self) -> usize {
        self.task_queue.lock().await.len()
    }

    /// Clears completed task results
    pub async fn cleanup_task(&self, task_id: &str) {
        let mut aggregator = self.result_aggregator.lock().await;
        aggregator.cleanup(task_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{AgentConfig, AgentRole};
    use crate::coordinator::strategy::RoundRobinStrategy;
    use serde_json::json;

    fn create_test_coordinator() -> AgentCoordinator {
        let mut registry = AgentRegistry::new();

        // Add test agents
        for i in 0..3 {
            let config = AgentConfig::new(
                AgentId::new(format!("agent-{}", i)),
                format!("Agent {}", i),
            );
            registry.register(config).unwrap();
        }

        let registry = Arc::new(RwLock::new(registry));
        let strategy = Arc::new(RoundRobinStrategy::new());

        AgentCoordinator::with_strategy(registry, strategy)
    }

    #[tokio::test]
    async fn test_coordinator_creation() {
        let coordinator = create_test_coordinator();
        assert_eq!(coordinator.pending_task_count().await, 0);
    }

    #[tokio::test]
    async fn test_submit_single_task() {
        let coordinator = create_test_coordinator();

        let task = CoordinatedTask::new("task-1".to_string(), TaskType::Single, json!({}));

        let result = coordinator.submit_task(task).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "task-1");
    }

    #[tokio::test]
    async fn test_submit_task_with_priority() {
        let coordinator = create_test_coordinator();

        let low_task =
            CoordinatedTask::new("low".to_string(), TaskType::Single, json!({}))
                .with_priority(TaskPriority::Low);

        let high_task =
            CoordinatedTask::new("high".to_string(), TaskType::Single, json!({}))
                .with_priority(TaskPriority::High);

        coordinator.submit_task(low_task).await.unwrap();
        coordinator.submit_task(high_task).await.unwrap();

        // High priority task should be processed first
        // (actual verification would require more sophisticated queue inspection)
    }

    #[tokio::test]
    async fn test_validate_task() {
        let coordinator = create_test_coordinator();

        // Empty task ID should fail
        let invalid_task = CoordinatedTask::new("".to_string(), TaskType::Single, json!({}));
        assert!(coordinator.validate_task(&invalid_task).is_err());

        // Valid task should pass
        let valid_task = CoordinatedTask::new("task-1".to_string(), TaskType::Single, json!({}));
        assert!(coordinator.validate_task(&valid_task).is_ok());
    }

    #[tokio::test]
    async fn test_parallel_task_insufficient_agents() {
        let coordinator = create_test_coordinator();

        // Request more agents than available (have 3, need 5)
        let task = CoordinatedTask::new(
            "parallel-task".to_string(),
            TaskType::Parallel { agent_count: 5 },
            json!({}),
        );

        let result = coordinator.submit_task(task).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_and_retrieve_result() {
        let coordinator = create_test_coordinator();

        let result = TaskResult::new(
            "task-1".to_string(),
            AgentId::new("agent-0"),
            json!({"status": "success"}),
            100,
        );

        coordinator.add_task_result("task-1", result).await;

        // Verify result was added
        let aggregator = coordinator.result_aggregator.lock().await;
        assert!(aggregator.is_complete("task-1", 1));
    }

    #[tokio::test]
    async fn test_cleanup_task() {
        let coordinator = create_test_coordinator();

        let result = TaskResult::new(
            "task-1".to_string(),
            AgentId::new("agent-0"),
            json!({"status": "success"}),
            100,
        );

        coordinator.add_task_result("task-1", result).await;
        coordinator.cleanup_task("task-1").await;

        // Verify result was cleaned up
        let aggregator = coordinator.result_aggregator.lock().await;
        assert_eq!(aggregator.pending_count("task-1"), 0);
    }
}
