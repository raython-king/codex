//! Algorithm engineer specialized coordinator
//!
//! This coordinator is optimized for algorithm development workflows,
//! understanding algorithm-specific tasks and routing them to appropriate experts.

use super::agent_pool::{AgentPool, AgentPoolConfig};
use super::algorithm_specialist::{AlgorithmAgent, AlgorithmCapability};
use super::task_distributor::{TaskDistributionStrategy, TaskDistributor};
use super::types::{AgentCapability, DelegatedTask};
use crate::codex::TurnContext;
use crate::error::CodexErr;
use crate::state::TaskKind;
use crate::tasks::{SessionTask, SessionTaskContext};
use async_trait::async_trait;
use codex_protocol::user_input::UserInput;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

/// Configuration for algorithm coordinator
#[derive(Debug, Clone)]
pub struct AlgorithmCoordinatorConfig {
    /// Agent pool configuration
    pub pool_config: AgentPoolConfig,
    /// Task distribution strategy
    pub distribution_strategy: TaskDistributionStrategy,
    /// Perception experts to include
    pub perception_experts: Vec<AlgorithmAgent>,
    /// Enable automatic capability inference
    pub auto_infer_capabilities: bool,
    /// Enable verbose logging
    pub verbose: bool,
}

impl Default for AlgorithmCoordinatorConfig {
    fn default() -> Self {
        Self {
            pool_config: AgentPoolConfig {
                max_agents: 15,
                max_concurrent_tasks: 30,
                enable_event_aggregation: true,
            },
            distribution_strategy: TaskDistributionStrategy::BestMatch,
            perception_experts: Vec::new(),
            auto_infer_capabilities: true,
            verbose: false,
        }
    }
}

/// Specialized coordinator for algorithm engineering tasks
pub struct AlgorithmCoordinator {
    /// Configuration
    config: AlgorithmCoordinatorConfig,
    /// Agent pool
    agent_pool: Arc<RwLock<Option<Arc<AgentPool>>>>,
    /// Task distributor
    task_distributor: Arc<RwLock<Option<Arc<TaskDistributor>>>>,
    /// Next task ID
    next_task_id: Arc<RwLock<usize>>,
}

impl AlgorithmCoordinator {
    /// Create a new algorithm coordinator
    pub fn new(config: AlgorithmCoordinatorConfig) -> Self {
        Self {
            config,
            agent_pool: Arc::new(RwLock::new(None)),
            task_distributor: Arc::new(RwLock::new(None)),
            next_task_id: Arc::new(RwLock::new(0)),
        }
    }

    /// Initialize the agent pool with perception experts
    async fn initialize_agents(&self) -> Result<Arc<AgentPool>, CodexErr> {
        let pool = Arc::new(AgentPool::new(self.config.pool_config.clone()));

        // Add all perception experts
        for expert in &self.config.perception_experts {
            pool.add_agent(expert.clone().into_spec()).await?;
        }

        Ok(pool)
    }

    /// Infer algorithm capabilities from task description
    fn infer_algorithm_capabilities(&self, description: &str) -> HashSet<AlgorithmCapability> {
        let mut capabilities = HashSet::new();
        let desc_lower = description.to_lowercase();

        // Machine learning keywords
        if desc_lower.contains("machine learning")
            || desc_lower.contains("ml model")
            || desc_lower.contains("classifier")
            || desc_lower.contains("regression")
        {
            capabilities.insert(AlgorithmCapability::MachineLearning);
        }

        // Deep learning keywords
        if desc_lower.contains("deep learning")
            || desc_lower.contains("neural network")
            || desc_lower.contains("pytorch")
            || desc_lower.contains("tensorflow")
            || desc_lower.contains("transformer")
        {
            capabilities.insert(AlgorithmCapability::DeepLearning);
        }

        // Computer vision keywords
        if desc_lower.contains("computer vision")
            || desc_lower.contains("image")
            || desc_lower.contains("object detection")
            || desc_lower.contains("segmentation")
            || desc_lower.contains("opencv")
            || desc_lower.contains("yolo")
            || desc_lower.contains("cnn")
        {
            capabilities.insert(AlgorithmCapability::ComputerVision);
        }

        // NLP keywords
        if desc_lower.contains("nlp")
            || desc_lower.contains("natural language")
            || desc_lower.contains("text")
            || desc_lower.contains("bert")
            || desc_lower.contains("gpt")
            || desc_lower.contains("tokeniz")
            || desc_lower.contains("embedding")
        {
            capabilities.insert(AlgorithmCapability::NaturalLanguageProcessing);
        }

        // Reinforcement learning keywords
        if desc_lower.contains("reinforcement")
            || desc_lower.contains("rl")
            || desc_lower.contains("policy")
            || desc_lower.contains("reward")
            || desc_lower.contains("agent")
            || desc_lower.contains("gym")
        {
            capabilities.insert(AlgorithmCapability::ReinforcementLearning);
        }

        // Time series keywords
        if desc_lower.contains("time series")
            || desc_lower.contains("forecast")
            || desc_lower.contains("temporal")
            || desc_lower.contains("lstm")
            || desc_lower.contains("sequential")
        {
            capabilities.insert(AlgorithmCapability::TimeSeriesAnalysis);
        }

        // Optimization keywords
        if desc_lower.contains("optimiz")
            || desc_lower.contains("hyperparameter")
            || desc_lower.contains("tuning")
            || desc_lower.contains("gradient")
        {
            capabilities.insert(AlgorithmCapability::Optimization);
        }

        // Data engineering keywords
        if desc_lower.contains("data")
            || desc_lower.contains("preprocess")
            || desc_lower.contains("feature engineering")
            || desc_lower.contains("etl")
            || desc_lower.contains("pipeline")
        {
            capabilities.insert(AlgorithmCapability::DataEngineering);
        }

        // Model evaluation keywords
        if desc_lower.contains("evaluat")
            || desc_lower.contains("metric")
            || desc_lower.contains("validation")
            || desc_lower.contains("accuracy")
            || desc_lower.contains("performance")
        {
            capabilities.insert(AlgorithmCapability::ModelEvaluation);
        }

        // Statistical analysis keywords
        if desc_lower.contains("statistic")
            || desc_lower.contains("probability")
            || desc_lower.contains("distribution")
            || desc_lower.contains("hypothesis")
        {
            capabilities.insert(AlgorithmCapability::StatisticalAnalysis);
        }

        // Clustering keywords
        if desc_lower.contains("cluster")
            || desc_lower.contains("kmeans")
            || desc_lower.contains("dbscan")
            || desc_lower.contains("grouping")
        {
            capabilities.insert(AlgorithmCapability::Clustering);
        }

        // Anomaly detection keywords
        if desc_lower.contains("anomaly")
            || desc_lower.contains("outlier")
            || desc_lower.contains("fraud detection")
        {
            capabilities.insert(AlgorithmCapability::AnomalyDetection);
        }

        capabilities
    }

    /// Infer general capabilities from task description
    fn infer_general_capabilities(&self, description: &str) -> HashSet<AgentCapability> {
        let mut capabilities = HashSet::new();
        let desc_lower = description.to_lowercase();

        if desc_lower.contains("implement")
            || desc_lower.contains("code")
            || desc_lower.contains("write")
        {
            capabilities.insert(AgentCapability::CodeGeneration);
        }

        if desc_lower.contains("review") || desc_lower.contains("check") {
            capabilities.insert(AgentCapability::CodeReview);
        }

        if desc_lower.contains("test") || desc_lower.contains("verify") {
            capabilities.insert(AgentCapability::Testing);
        }

        if desc_lower.contains("research")
            || desc_lower.contains("find")
            || desc_lower.contains("paper")
        {
            capabilities.insert(AgentCapability::Research);
        }

        // Default to code generation if nothing specific
        if capabilities.is_empty() {
            capabilities.insert(AgentCapability::CodeGeneration);
        }

        capabilities
    }

    /// Plan task delegation for algorithm tasks
    async fn plan_algorithm_delegation(
        &self,
        input: &[UserInput],
    ) -> Result<Vec<DelegatedTask>, CodexErr> {
        let message = input
            .iter()
            .filter_map(|i| match i {
                UserInput::Text { text } => Some(text.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n");

        let task_id = {
            let mut next_id = self.next_task_id.write().await;
            let id = format!("algo-task-{next_id}");
            *next_id += 1;
            id
        };

        // Infer both algorithm and general capabilities
        let _algo_capabilities = if self.config.auto_infer_capabilities {
            self.infer_algorithm_capabilities(&message)
        } else {
            HashSet::new()
        };

        let general_capabilities = self.infer_general_capabilities(&message);

        // Get distributor
        let distributor = {
            let dist_opt = self.task_distributor.read().await;
            dist_opt
                .as_ref()
                .ok_or_else(|| CodexErr::Fatal("Task distributor not initialized".to_string()))?
                .clone()
        };

        // Create delegated task
        let mut task = DelegatedTask::new(task_id, "", message.clone());
        task.required_capabilities = general_capabilities;

        // Try to find best matching agent
        let agent_id = distributor.distribute_task(&task).await?;
        task.agent_id = agent_id;

        Ok(vec![task])
    }

    /// Generate execution plan summary
    fn generate_plan_summary(
        &self,
        tasks: &[DelegatedTask],
        algo_caps: &HashSet<AlgorithmCapability>,
    ) -> String {
        let mut report = String::from("Algorithm Engineering Task Plan\n");
        report.push_str("=".repeat(50).as_str());
        report.push('\n');

        // Algorithm capabilities detected
        if !algo_caps.is_empty() {
            report.push_str("\nDetected Algorithm Capabilities:\n");
            for cap in algo_caps {
                report.push_str(&format!("  • {}\n", cap));
            }
        }

        // Task delegation plan
        report.push_str("\nTask Delegation:\n");
        for task in tasks {
            report.push_str(&format!(
                "\nTask: {}\nAssigned to: {}\nRequired Capabilities:\n",
                task.description, task.agent_id
            ));
            for cap in &task.required_capabilities {
                report.push_str(&format!("  - {:?}\n", cap));
            }
        }

        report
    }
}

#[async_trait]
impl SessionTask for AlgorithmCoordinator {
    fn kind(&self) -> TaskKind {
        TaskKind::Regular
    }

    async fn run(
        self: Arc<Self>,
        _session: Arc<SessionTaskContext>,
        _ctx: Arc<TurnContext>,
        input: Vec<UserInput>,
        _cancellation_token: CancellationToken,
    ) -> Option<String> {
        let start_time = Instant::now();

        // Initialize agents
        let pool = match self.initialize_agents().await {
            Ok(p) => p,
            Err(e) => {
                return Some(format!("Error initializing algorithm agents: {e}"));
            }
        };

        {
            let mut pool_opt = self.agent_pool.write().await;
            *pool_opt = Some(pool.clone());
        }

        // Initialize task distributor
        let distributor = Arc::new(TaskDistributor::new(
            pool.clone(),
            self.config.distribution_strategy.clone(),
        ));

        {
            let mut dist_opt = self.task_distributor.write().await;
            *dist_opt = Some(distributor);
        }

        // Extract task description for capability inference
        let message = input
            .iter()
            .filter_map(|i| match i {
                UserInput::Text { text } => Some(text.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n");

        let algo_caps = self.infer_algorithm_capabilities(&message);

        // Plan task delegation
        let tasks = match self.plan_algorithm_delegation(&input).await {
            Ok(t) => t,
            Err(e) => {
                return Some(format!("Error planning task delegation: {e}"));
            }
        };

        let elapsed = start_time.elapsed();

        // Generate execution plan
        let mut report = self.generate_plan_summary(&tasks, &algo_caps);
        report.push_str(&format!(
            "\nPlan generated in {:.2}s\n",
            elapsed.as_secs_f64()
        ));

        // Add pool statistics
        let stats = pool.get_stats().await;
        report.push_str(&format!(
            "\nAgent Pool Status:\n  Active Agents: {}\n  Active Tasks: {}\n  Available Capacity: {}\n",
            stats.total_agents, stats.total_active_tasks, stats.available_capacity
        ));

        Some(report)
    }

    async fn abort(&self, _session: Arc<SessionTaskContext>, _ctx: Arc<TurnContext>) {
        // Cleanup handled automatically
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::multi_agent::algorithm_specialist::presets;

    #[test]
    fn test_infer_cv_capabilities() {
        let config = AlgorithmCoordinatorConfig::default();
        let coordinator = AlgorithmCoordinator::new(config);

        let caps = coordinator
            .infer_algorithm_capabilities("Implement object detection using YOLO for images");

        assert!(caps.contains(&AlgorithmCapability::ComputerVision));
        assert!(caps.contains(&AlgorithmCapability::DeepLearning));
    }

    #[test]
    fn test_infer_nlp_capabilities() {
        let config = AlgorithmCoordinatorConfig::default();
        let coordinator = AlgorithmCoordinator::new(config);

        let caps = coordinator.infer_algorithm_capabilities("Build a BERT-based text classifier");

        assert!(caps.contains(&AlgorithmCapability::NaturalLanguageProcessing));
        assert!(caps.contains(&AlgorithmCapability::DeepLearning));
    }

    #[tokio::test]
    async fn test_coordinator_initialization() {
        let config = AlgorithmCoordinatorConfig {
            perception_experts: vec![presets::computer_vision_expert()],
            ..Default::default()
        };

        let coordinator = AlgorithmCoordinator::new(config);
        let pool = coordinator.initialize_agents().await.unwrap();
        let stats = pool.get_stats().await;

        assert_eq!(stats.total_agents, 1);
    }
}
