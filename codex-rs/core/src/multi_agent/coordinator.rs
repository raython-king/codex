//! Multi-agent coordinator for orchestrating collaborative development

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use tracing::{debug, info, warn};

use codex_protocol::user_input::UserInput;

use crate::codex::{Session, TurnContext};
use crate::config::Config;
use crate::error::CodexErr;
use crate::AuthManager;

use super::agent_pool::AgentPool;
use super::config::MultiAgentConfig;
use super::result_aggregator::{AgentResult, AggregatedResult, ResultAggregator};
use super::task_decomposer::{DecompositionContext, DependencyType, SubTask, TaskDecomposer};

/// Coordinates multiple agents working on decomposed tasks
pub struct MultiAgentCoordinator {
    config: MultiAgentConfig,
    agent_pool: AgentPool,
    task_decomposer: TaskDecomposer,
    result_aggregator: ResultAggregator,
}

impl MultiAgentCoordinator {
    pub fn new(
        config: MultiAgentConfig,
        base_config: Config,
        auth_manager: Arc<AuthManager>,
        parent_session: Arc<Session>,
        parent_ctx: Arc<TurnContext>,
    ) -> Self {
        let agent_pool = AgentPool::new(
            config.max_concurrent_agents,
            base_config,
            auth_manager,
            parent_session,
            parent_ctx,
        );

        let task_decomposer = TaskDecomposer::new(config.task_decomposition_strategy);
        let result_aggregator = ResultAggregator::new(config.conflict_resolution);

        Self {
            config,
            agent_pool,
            task_decomposer,
            result_aggregator,
        }
    }

    /// Execute a complex task using multiple agents
    pub async fn execute(
        &self,
        input: Vec<UserInput>,
        context: DecompositionContext,
        cancel_token: CancellationToken,
    ) -> Result<AggregatedResult, CodexErr> {
        info!("Starting multi-agent task execution");

        // Step 1: Decompose the task
        let sub_tasks = self
            .task_decomposer
            .decompose(&input, &context)
            .await
            .map_err(|e| CodexErr::Fatal(format!("Task decomposition failed: {}", e)))?;

        info!("Task decomposed into {} sub-tasks", sub_tasks.len());

        // If only one sub-task, no need for multi-agent coordination
        if sub_tasks.len() == 1 {
            warn!("Only one sub-task, executing directly without multi-agent coordination");
            return self.execute_single_task(sub_tasks.into_iter().next().unwrap(), cancel_token).await;
        }

        // Step 2: Build dependency graph and execution plan
        let execution_plan = self.build_execution_plan(&sub_tasks)?;

        // Step 3: Execute tasks according to the plan
        let results = self
            .execute_plan(execution_plan, sub_tasks, cancel_token)
            .await?;

        // Step 4: Aggregate results
        let aggregated = self
            .result_aggregator
            .aggregate(results)
            .await
            .map_err(|e| CodexErr::Fatal(format!("Result aggregation failed: {}", e)))?;

        info!(
            "Multi-agent execution completed: {} agents, {} conflicts",
            aggregated.agent_count, aggregated.conflict_count
        );

        Ok(aggregated)
    }

    /// Build an execution plan respecting dependencies
    fn build_execution_plan(&self, sub_tasks: &[SubTask]) -> Result<ExecutionPlan, CodexErr> {
        let mut plan = ExecutionPlan::new();

        // Build dependency map
        let mut task_map: HashMap<String, &SubTask> = HashMap::new();
        for task in sub_tasks {
            task_map.insert(task.id.clone(), task);
        }

        // Topological sort to determine execution order
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();

        for task in sub_tasks {
            in_degree.entry(task.id.clone()).or_insert(0);
            adjacency.entry(task.id.clone()).or_insert_with(Vec::new);

            for dep in &task.dependencies {
                if dep.dependency_type == DependencyType::Blocking {
                    *in_degree.entry(task.id.clone()).or_insert(0) += 1;
                    adjacency
                        .entry(dep.task_id.clone())
                        .or_insert_with(Vec::new)
                        .push(task.id.clone());
                }
            }
        }

        // Kahn's algorithm for topological sort
        let mut queue: VecDeque<String> = in_degree
            .iter()
            .filter_map(|(id, &degree)| if degree == 0 { Some(id.clone()) } else { None })
            .collect();

        let mut wave_idx = 0;
        while !queue.is_empty() {
            // All tasks in the current queue can execute in parallel (same wave)
            let current_wave: Vec<String> = queue.drain(..).collect();

            for task_id in &current_wave {
                plan.add_to_wave(wave_idx, task_id.clone());

                // Decrease in-degree of dependent tasks
                if let Some(dependents) = adjacency.get(task_id) {
                    for dependent_id in dependents {
                        if let Some(degree) = in_degree.get_mut(dependent_id) {
                            *degree -= 1;
                            if *degree == 0 {
                                queue.push_back(dependent_id.clone());
                            }
                        }
                    }
                }
            }

            wave_idx += 1;
        }

        // Check for cycles
        if plan.total_tasks() != sub_tasks.len() {
            return Err(CodexErr::Fatal(
                "Circular dependency detected in task graph".to_string(),
            ));
        }

        debug!("Execution plan: {} waves", plan.waves.len());
        for (idx, wave) in plan.waves.iter().enumerate() {
            debug!("  Wave {}: {} tasks", idx, wave.len());
        }

        Ok(plan)
    }

    /// Execute tasks according to the plan
    async fn execute_plan(
        &self,
        plan: ExecutionPlan,
        sub_tasks: Vec<SubTask>,
        cancel_token: CancellationToken,
    ) -> Result<Vec<AgentResult>, CodexErr> {
        let mut all_results = Vec::new();
        let task_map: HashMap<String, SubTask> =
            sub_tasks.into_iter().map(|t| (t.id.clone(), t)).collect();

        // Execute wave by wave
        for (wave_idx, wave) in plan.waves.iter().enumerate() {
            if cancel_token.is_cancelled() {
                warn!("Multi-agent execution cancelled at wave {}", wave_idx);
                break;
            }

            info!("Executing wave {} with {} tasks", wave_idx, wave.len());

            // Spawn agents for all tasks in this wave
            let mut handles = Vec::new();
            for task_id in wave {
                let sub_task = task_map
                    .get(task_id)
                    .ok_or_else(|| CodexErr::Fatal(format!("Task {} not found", task_id)))?
                    .clone();

                let handle = self
                    .agent_pool
                    .spawn_agent(sub_task, cancel_token.child_token())
                    .await?;
                handles.push(handle);
            }

            // Wait for all agents in this wave to complete
            for handle in handles {
                let result = handle.await_result().await?;
                all_results.push(result);
            }

            info!("Wave {} completed", wave_idx);
        }

        Ok(all_results)
    }

    /// Execute a single task (fallback when no decomposition is needed)
    async fn execute_single_task(
        &self,
        task: SubTask,
        cancel_token: CancellationToken,
    ) -> Result<AggregatedResult, CodexErr> {
        let handle = self
            .agent_pool
            .spawn_agent(task, cancel_token)
            .await?;

        let result = handle.await_result().await?;

        Ok(AggregatedResult {
            success: result.success,
            final_changes: result.modified_files,
            messages: result.message.into_iter().collect(),
            errors: result.errors,
            total_execution_time_ms: result.execution_time_ms,
            conflict_count: 0,
            agent_count: 1,
        })
    }
}

/// Execution plan with waves of parallel tasks
#[derive(Debug, Clone)]
struct ExecutionPlan {
    /// Each wave contains tasks that can execute in parallel
    waves: Vec<Vec<String>>,
}

impl ExecutionPlan {
    fn new() -> Self {
        Self { waves: Vec::new() }
    }

    fn add_to_wave(&mut self, wave_idx: usize, task_id: String) {
        while self.waves.len() <= wave_idx {
            self.waves.push(Vec::new());
        }
        self.waves[wave_idx].push(task_id);
    }

    fn total_tasks(&self) -> usize {
        self.waves.iter().map(|w| w.len()).sum()
    }
}
