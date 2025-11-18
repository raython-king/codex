//! Foundation model coordinator
//!
//! This coordinator specializes in foundation model workflows,
//! routing tasks to appropriate experts based on architecture,
//! pretraining strategy, scaling requirements, and transfer learning needs.

use super::agent_pool::{AgentPool, AgentPoolConfig};
use super::foundation_model_specialist::{
    DatasetScale, FoundationModelAgent, FoundationModelArchitecture,
    FoundationModelCapability, ModelScale, PretrainingStrategy,
};
use super::task_distributor::{TaskDistributor, TaskDistributionStrategy};
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

/// Configuration for foundation model coordinator
#[derive(Debug, Clone)]
pub struct FoundationModelCoordinatorConfig {
    /// Agent pool configuration
    pub pool_config: AgentPoolConfig,
    /// Task distribution strategy
    pub distribution_strategy: TaskDistributionStrategy,
    /// Foundation model experts to include
    pub fm_experts: Vec<FoundationModelAgent>,
    /// Enable automatic capability inference
    pub auto_infer: bool,
    /// Enable verbose logging
    pub verbose: bool,
}

impl Default for FoundationModelCoordinatorConfig {
    fn default() -> Self {
        Self {
            pool_config: AgentPoolConfig {
                max_agents: 15,
                max_concurrent_tasks: 30,
                enable_event_aggregation: true,
            },
            distribution_strategy: TaskDistributionStrategy::BestMatch,
            fm_experts: Vec::new(),
            auto_infer: true,
            verbose: false,
        }
    }
}

/// Specialized coordinator for foundation model tasks
pub struct FoundationModelCoordinator {
    /// Configuration
    config: FoundationModelCoordinatorConfig,
    /// Agent pool
    agent_pool: Arc<RwLock<Option<Arc<AgentPool>>>>,
    /// Task distributor
    task_distributor: Arc<RwLock<Option<Arc<TaskDistributor>>>>,
    /// Next task ID
    next_task_id: Arc<RwLock<usize>>,
}

impl FoundationModelCoordinator {
    /// Create a new foundation model coordinator
    pub fn new(config: FoundationModelCoordinatorConfig) -> Self {
        Self {
            config,
            agent_pool: Arc::new(RwLock::new(None)),
            task_distributor: Arc::new(RwLock::new(None)),
            next_task_id: Arc::new(RwLock::new(0)),
        }
    }

    /// Initialize the agent pool with foundation model experts
    async fn initialize_agents(&self) -> Result<Arc<AgentPool>, CodexErr> {
        let pool = Arc::new(AgentPool::new(self.config.pool_config.clone()));

        // Add all foundation model experts
        for expert in &self.config.fm_experts {
            pool.add_agent(expert.clone().into_spec()).await?;
        }

        Ok(pool)
    }

    /// Infer foundation model architectures from task description
    fn infer_architectures(&self, description: &str) -> HashSet<FoundationModelArchitecture> {
        let mut architectures = HashSet::new();
        let desc_lower = description.to_lowercase();

        // Vision-Language Models
        if desc_lower.contains("clip") {
            architectures.insert(FoundationModelArchitecture::CLIP);
        }
        if desc_lower.contains("align") {
            architectures.insert(FoundationModelArchitecture::ALIGN);
        }
        if desc_lower.contains("blip") {
            architectures.insert(FoundationModelArchitecture::BLIP);
        }
        if desc_lower.contains("florence") {
            architectures.insert(FoundationModelArchitecture::Florence);
        }

        // Self-Supervised Vision
        if desc_lower.contains("mae") || desc_lower.contains("masked autoencoder") {
            architectures.insert(FoundationModelArchitecture::MAE);
        }
        if desc_lower.contains("beit") {
            architectures.insert(FoundationModelArchitecture::BEiT);
        }
        if desc_lower.contains("dino") {
            architectures.insert(FoundationModelArchitecture::DINOv2);
        }
        if desc_lower.contains("simclr") {
            architectures.insert(FoundationModelArchitecture::SimCLR);
        }
        if desc_lower.contains("moco") {
            architectures.insert(FoundationModelArchitecture::MoCo);
        }

        // Segmentation
        if desc_lower.contains("sam") || desc_lower.contains("segment anything") {
            architectures.insert(FoundationModelArchitecture::SAM);
        }

        // Multimodal
        if desc_lower.contains("flamingo") {
            architectures.insert(FoundationModelArchitecture::Flamingo);
        }
        if desc_lower.contains("llava") {
            architectures.insert(FoundationModelArchitecture::LLaVA);
        }
        if desc_lower.contains("gpt-4v") || desc_lower.contains("gpt4vision") {
            architectures.insert(FoundationModelArchitecture::GPT4Vision);
        }

        // Vision Transformers
        if desc_lower.contains("deit") {
            architectures.insert(FoundationModelArchitecture::DEIT);
        }
        if desc_lower.contains("swin") {
            architectures.insert(FoundationModelArchitecture::SwinTransformer);
        }

        architectures
    }

    /// Infer pretraining strategies from task description
    fn infer_pretraining_strategies(&self, description: &str) -> HashSet<PretrainingStrategy> {
        let mut strategies = HashSet::new();
        let desc_lower = description.to_lowercase();

        if desc_lower.contains("masked") || desc_lower.contains("mae") {
            strategies.insert(PretrainingStrategy::MaskedImageModeling);
        }

        if desc_lower.contains("contrastive") || desc_lower.contains("clip") {
            strategies.insert(PretrainingStrategy::ContrastiveLearning);
        }

        if desc_lower.contains("vision-language") || desc_lower.contains("vision language") {
            strategies.insert(PretrainingStrategy::VisionLanguageAlignment);
        }

        if desc_lower.contains("self-supervised") || desc_lower.contains("self supervised") {
            strategies.insert(PretrainingStrategy::ContrastiveLearning);
        }

        strategies
    }

    /// Infer foundation model capabilities from task description
    fn infer_fm_capabilities(&self, description: &str) -> HashSet<FoundationModelCapability> {
        let mut capabilities = HashSet::new();
        let desc_lower = description.to_lowercase();

        // Architecture
        if desc_lower.contains("foundation model") || desc_lower.contains("foundation-model") {
            capabilities.insert(FoundationModelCapability::FoundationModelDesign);
        }
        if desc_lower.contains("vision transformer") || desc_lower.contains("vit") {
            capabilities.insert(FoundationModelCapability::VisionTransformerDesign);
        }
        if desc_lower.contains("multimodal") {
            capabilities.insert(FoundationModelCapability::MultimodalArchitecture);
        }
        if desc_lower.contains("attention") {
            capabilities.insert(FoundationModelCapability::AttentionMechanisms);
        }

        // Pretraining
        if desc_lower.contains("pretrain") || desc_lower.contains("pre-train") {
            capabilities.insert(FoundationModelCapability::LargeScalePretraining);
        }
        if desc_lower.contains("self-supervised") || desc_lower.contains("self supervised") {
            capabilities.insert(FoundationModelCapability::SelfSupervisedPretraining);
        }
        if desc_lower.contains("contrastive") {
            capabilities.insert(FoundationModelCapability::ContrastivePretraining);
        }
        if desc_lower.contains("masked") {
            capabilities.insert(FoundationModelCapability::MaskedPretraining);
        }

        // Transfer Learning
        if desc_lower.contains("zero-shot") || desc_lower.contains("zero shot") {
            capabilities.insert(FoundationModelCapability::ZeroShotTransfer);
        }
        if desc_lower.contains("few-shot") || desc_lower.contains("few shot") {
            capabilities.insert(FoundationModelCapability::FewShotTransfer);
        }
        if desc_lower.contains("fine-tun") || desc_lower.contains("finetun") {
            capabilities.insert(FoundationModelCapability::FullFineTuning);
        }
        if desc_lower.contains("lora") || desc_lower.contains("adapter") || desc_lower.contains("peft") {
            capabilities.insert(FoundationModelCapability::ParameterEfficientFineTuning);
        }
        if desc_lower.contains("linear prob") {
            capabilities.insert(FoundationModelCapability::LinearProbing);
        }

        // Data
        if desc_lower.contains("web-scale") || desc_lower.contains("web scale") {
            capabilities.insert(FoundationModelCapability::WebScaleDataCuration);
        }
        if desc_lower.contains("data curation") {
            capabilities.insert(FoundationModelCapability::WebScaleDataCuration);
        }
        if desc_lower.contains("deduplication") {
            capabilities.insert(FoundationModelCapability::DataDeduplication);
        }

        // Scaling Laws
        if desc_lower.contains("scaling law") {
            capabilities.insert(FoundationModelCapability::ScalingLawAnalysis);
        }
        if desc_lower.contains("compute-optimal") || desc_lower.contains("compute optimal") {
            capabilities.insert(FoundationModelCapability::ComputeOptimalScaling);
        }
        if desc_lower.contains("emergent") {
            capabilities.insert(FoundationModelCapability::EmergentAbilityDetection);
        }

        // Efficiency
        if desc_lower.contains("model parallel") {
            capabilities.insert(FoundationModelCapability::ModelParallelism);
        }
        if desc_lower.contains("data parallel") {
            capabilities.insert(FoundationModelCapability::DataParallelism);
        }
        if desc_lower.contains("pipeline parallel") {
            capabilities.insert(FoundationModelCapability::PipelineParallelism);
        }
        if desc_lower.contains("zero") && desc_lower.contains("optim") {
            capabilities.insert(FoundationModelCapability::ZeROOptimization);
        }

        // Evaluation
        if desc_lower.contains("evaluat") || desc_lower.contains("benchmark") {
            capabilities.insert(FoundationModelCapability::ZeroShotEvaluation);
        }
        if desc_lower.contains("robust") {
            capabilities.insert(FoundationModelCapability::RobustnessEvaluation);
        }

        capabilities
    }

    /// Infer model scale from task description
    fn infer_model_scale(&self, description: &str) -> Option<ModelScale> {
        let desc_lower = description.to_lowercase();

        if desc_lower.contains("billion") || desc_lower.contains("100b") {
            Some(ModelScale::Gigantic)
        } else if desc_lower.contains("xxl") || desc_lower.contains("10b") {
            Some(ModelScale::XXLarge)
        } else if desc_lower.contains("xl") || desc_lower.contains("xlarge") {
            Some(ModelScale::XLarge)
        } else if desc_lower.contains("large") {
            Some(ModelScale::Large)
        } else if desc_lower.contains("base") {
            Some(ModelScale::Base)
        } else if desc_lower.contains("small") {
            Some(ModelScale::Small)
        } else if desc_lower.contains("tiny") {
            Some(ModelScale::Tiny)
        } else {
            None
        }
    }

    /// Infer dataset scale from task description
    fn infer_dataset_scale(&self, description: &str) -> Option<DatasetScale> {
        let desc_lower = description.to_lowercase();

        if desc_lower.contains("internet-scale") || desc_lower.contains("internet scale") {
            Some(DatasetScale::InternetScale)
        } else if desc_lower.contains("web-scale") || desc_lower.contains("web scale") {
            Some(DatasetScale::WebScale)
        } else if desc_lower.contains("large") {
            Some(DatasetScale::Large)
        } else if desc_lower.contains("medium") {
            Some(DatasetScale::Medium)
        } else if desc_lower.contains("small") {
            Some(DatasetScale::Small)
        } else {
            None
        }
    }

    /// Infer general agent capabilities
    fn infer_general_capabilities(&self, description: &str) -> HashSet<AgentCapability> {
        let mut capabilities = HashSet::new();
        let desc_lower = description.to_lowercase();

        if desc_lower.contains("implement")
            || desc_lower.contains("code")
            || desc_lower.contains("write")
        {
            capabilities.insert(AgentCapability::CodeGeneration);
        }

        if desc_lower.contains("research") || desc_lower.contains("analyze") {
            capabilities.insert(AgentCapability::Research);
        }

        if desc_lower.contains("test") || desc_lower.contains("evaluat") {
            capabilities.insert(AgentCapability::Testing);
        }

        if desc_lower.contains("architecture") || desc_lower.contains("design") {
            capabilities.insert(AgentCapability::Architecture);
        }

        // Default to code generation if nothing specific
        if capabilities.is_empty() {
            capabilities.insert(AgentCapability::CodeGeneration);
        }

        capabilities
    }

    /// Plan task delegation for foundation model tasks
    async fn plan_fm_delegation(
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
            let id = format!("fm-task-{next_id}");
            *next_id += 1;
            id
        };

        // Infer capabilities
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
        architectures: &HashSet<FoundationModelArchitecture>,
        strategies: &HashSet<PretrainingStrategy>,
        fm_caps: &HashSet<FoundationModelCapability>,
        model_scale: &Option<ModelScale>,
        dataset_scale: &Option<DatasetScale>,
    ) -> String {
        let mut report = String::from("Foundation Model Task Plan\n");
        report.push_str(&"=".repeat(50));
        report.push('\n');

        // Architectures detected
        if !architectures.is_empty() {
            report.push_str("\nDetected Foundation Model Architectures:\n");
            for arch in architectures {
                report.push_str(&format!("  • {}\n", arch));
            }
        }

        // Pretraining strategies
        if !strategies.is_empty() {
            report.push_str("\nPretraining Strategies:\n");
            for strategy in strategies {
                report.push_str(&format!("  • {}\n", strategy));
            }
        }

        // Model scale
        if let Some(scale) = model_scale {
            report.push_str(&format!("\nModel Scale: {}\n", scale));
        }

        // Dataset scale
        if let Some(scale) = dataset_scale {
            report.push_str(&format!("Dataset Scale: {:?}\n", scale));
        }

        // Foundation model capabilities
        if !fm_caps.is_empty() {
            report.push_str("\nRequired FM Capabilities:\n");
            for cap in fm_caps {
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
impl SessionTask for FoundationModelCoordinator {
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
                return Some(format!("Error initializing foundation model agents: {e}"));
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

        // Extract task description
        let message = input
            .iter()
            .filter_map(|i| match i {
                UserInput::Text { text } => Some(text.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n");

        // Infer context
        let architectures = if self.config.auto_infer {
            self.infer_architectures(&message)
        } else {
            HashSet::new()
        };

        let strategies = if self.config.auto_infer {
            self.infer_pretraining_strategies(&message)
        } else {
            HashSet::new()
        };

        let fm_caps = if self.config.auto_infer {
            self.infer_fm_capabilities(&message)
        } else {
            HashSet::new()
        };

        let model_scale = if self.config.auto_infer {
            self.infer_model_scale(&message)
        } else {
            None
        };

        let dataset_scale = if self.config.auto_infer {
            self.infer_dataset_scale(&message)
        } else {
            None
        };

        // Plan task delegation
        let tasks = match self.plan_fm_delegation(&input).await {
            Ok(t) => t,
            Err(e) => {
                return Some(format!("Error planning task delegation: {e}"));
            }
        };

        let elapsed = start_time.elapsed();

        // Generate execution plan
        let mut report = self.generate_plan_summary(
            &tasks,
            &architectures,
            &strategies,
            &fm_caps,
            &model_scale,
            &dataset_scale,
        );
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
    use crate::multi_agent::foundation_model_specialist::presets;

    #[test]
    fn test_infer_clip() {
        let config = FoundationModelCoordinatorConfig::default();
        let coordinator = FoundationModelCoordinator::new(config);

        let archs = coordinator.infer_architectures("Implement CLIP model");
        assert!(archs.contains(&FoundationModelArchitecture::CLIP));
    }

    #[test]
    fn test_infer_scaling_law() {
        let config = FoundationModelCoordinatorConfig::default();
        let coordinator = FoundationModelCoordinator::new(config);

        let caps = coordinator.infer_fm_capabilities("Analyze scaling law for compute-optimal training");
        assert!(caps.contains(&FoundationModelCapability::ScalingLawAnalysis));
        assert!(caps.contains(&FoundationModelCapability::ComputeOptimalScaling));
    }

    #[test]
    fn test_infer_model_scale() {
        let config = FoundationModelCoordinatorConfig::default();
        let coordinator = FoundationModelCoordinator::new(config);

        let scale = coordinator.infer_model_scale("Train a large model");
        assert_eq!(scale, Some(ModelScale::Large));
    }

    #[tokio::test]
    async fn test_coordinator_initialization() {
        let config = FoundationModelCoordinatorConfig {
            fm_experts: vec![presets::clip_expert()],
            ..Default::default()
        };

        let coordinator = FoundationModelCoordinator::new(config);
        let pool = coordinator.initialize_agents().await.unwrap();
        let stats = pool.get_stats().await;

        assert_eq!(stats.total_agents, 1);
    }
}
