//! Deep learning coordinator
//!
//! This coordinator specializes in deep learning workflows,
//! routing tasks to appropriate deep learning experts based on
//! framework, architecture, training strategy, and deployment needs.

use super::agent_pool::{AgentPool, AgentPoolConfig};
use super::deep_learning_specialist::{
    ApplicationDomain, DeepLearningAgent, DeepLearningCapability, DeepLearningFramework,
    ModelArchitecture,
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

/// Configuration for deep learning coordinator
#[derive(Debug, Clone)]
pub struct DeepLearningCoordinatorConfig {
    /// Agent pool configuration
    pub pool_config: AgentPoolConfig,
    /// Task distribution strategy
    pub distribution_strategy: TaskDistributionStrategy,
    /// Deep learning experts to include
    pub dl_experts: Vec<DeepLearningAgent>,
    /// Enable automatic capability inference
    pub auto_infer: bool,
    /// Enable verbose logging
    pub verbose: bool,
}

impl Default for DeepLearningCoordinatorConfig {
    fn default() -> Self {
        Self {
            pool_config: AgentPoolConfig {
                max_agents: 15,
                max_concurrent_tasks: 30,
                enable_event_aggregation: true,
            },
            distribution_strategy: TaskDistributionStrategy::BestMatch,
            dl_experts: Vec::new(),
            auto_infer: true,
            verbose: false,
        }
    }
}

/// Specialized coordinator for deep learning tasks
pub struct DeepLearningCoordinator {
    /// Configuration
    config: DeepLearningCoordinatorConfig,
    /// Agent pool
    agent_pool: Arc<RwLock<Option<Arc<AgentPool>>>>,
    /// Task distributor
    task_distributor: Arc<RwLock<Option<Arc<TaskDistributor>>>>,
    /// Next task ID
    next_task_id: Arc<RwLock<usize>>,
}

impl DeepLearningCoordinator {
    /// Create a new deep learning coordinator
    pub fn new(config: DeepLearningCoordinatorConfig) -> Self {
        Self {
            config,
            agent_pool: Arc::new(RwLock::new(None)),
            task_distributor: Arc::new(RwLock::new(None)),
            next_task_id: Arc::new(RwLock::new(0)),
        }
    }

    /// Initialize the agent pool with deep learning experts
    async fn initialize_agents(&self) -> Result<Arc<AgentPool>, CodexErr> {
        let pool = Arc::new(AgentPool::new(self.config.pool_config.clone()));

        // Add all deep learning experts
        for expert in &self.config.dl_experts {
            pool.add_agent(expert.clone().into_spec()).await?;
        }

        Ok(pool)
    }

    /// Infer frameworks from task description
    fn infer_frameworks(&self, description: &str) -> HashSet<DeepLearningFramework> {
        let mut frameworks = HashSet::new();
        let desc_lower = description.to_lowercase();

        if desc_lower.contains("pytorch") || desc_lower.contains("torch") {
            frameworks.insert(DeepLearningFramework::PyTorch);
        }

        if desc_lower.contains("tensorflow") || desc_lower.contains("tf.") {
            frameworks.insert(DeepLearningFramework::TensorFlow);
        }

        if desc_lower.contains("jax") || desc_lower.contains("flax") {
            frameworks.insert(DeepLearningFramework::JAX);
        }

        if desc_lower.contains("keras") {
            frameworks.insert(DeepLearningFramework::Keras);
        }

        if desc_lower.contains("onnx") {
            frameworks.insert(DeepLearningFramework::ONNX);
        }

        if desc_lower.contains("tensorrt") {
            frameworks.insert(DeepLearningFramework::TensorRT);
        }

        if desc_lower.contains("tflite") {
            frameworks.insert(DeepLearningFramework::TFLite);
        }

        if desc_lower.contains("coreml") {
            frameworks.insert(DeepLearningFramework::CoreML);
        }

        frameworks
    }

    /// Infer model architectures from task description
    fn infer_architectures(&self, description: &str) -> HashSet<ModelArchitecture> {
        let mut architectures = HashSet::new();
        let desc_lower = description.to_lowercase();

        // Transformers
        if desc_lower.contains("transformer") {
            architectures.insert(ModelArchitecture::Transformer);
        }
        if desc_lower.contains("bert") {
            architectures.insert(ModelArchitecture::BERT);
        }
        if desc_lower.contains("gpt") {
            architectures.insert(ModelArchitecture::GPT);
        }
        if desc_lower.contains("t5") {
            architectures.insert(ModelArchitecture::T5);
        }
        if desc_lower.contains("vit") || desc_lower.contains("vision transformer") {
            architectures.insert(ModelArchitecture::ViT);
        }
        if desc_lower.contains("clip") {
            architectures.insert(ModelArchitecture::CLIP);
        }

        // CNNs
        if desc_lower.contains("cnn") || desc_lower.contains("convolutional") {
            architectures.insert(ModelArchitecture::CNN);
        }
        if desc_lower.contains("resnet") {
            architectures.insert(ModelArchitecture::ResNet);
        }
        if desc_lower.contains("efficientnet") {
            architectures.insert(ModelArchitecture::EfficientNet);
        }
        if desc_lower.contains("mobilenet") {
            architectures.insert(ModelArchitecture::MobileNet);
        }

        // RNNs
        if desc_lower.contains("lstm") {
            architectures.insert(ModelArchitecture::LSTM);
        }
        if desc_lower.contains("gru") {
            architectures.insert(ModelArchitecture::GRU);
        }
        if desc_lower.contains("rnn") {
            architectures.insert(ModelArchitecture::RNN);
        }

        // Object Detection
        if desc_lower.contains("yolo") {
            architectures.insert(ModelArchitecture::YOLO);
        }
        if desc_lower.contains("rcnn") || desc_lower.contains("r-cnn") {
            architectures.insert(ModelArchitecture::RCNN);
        }

        // Generative
        if desc_lower.contains("gan") {
            architectures.insert(ModelArchitecture::GAN);
        }
        if desc_lower.contains("vae") || desc_lower.contains("variational autoencoder") {
            architectures.insert(ModelArchitecture::VAE);
        }
        if desc_lower.contains("diffusion") {
            architectures.insert(ModelArchitecture::Diffusion);
        }

        // Segmentation
        if desc_lower.contains("unet") || desc_lower.contains("u-net") {
            architectures.insert(ModelArchitecture::UNet);
        }

        architectures
    }

    /// Infer deep learning capabilities from task description
    fn infer_dl_capabilities(&self, description: &str) -> HashSet<DeepLearningCapability> {
        let mut capabilities = HashSet::new();
        let desc_lower = description.to_lowercase();

        // Architecture
        if desc_lower.contains("design") || desc_lower.contains("architecture") {
            capabilities.insert(DeepLearningCapability::ArchitectureDesign);
        }
        if desc_lower.contains("custom layer") || desc_lower.contains("implement layer") {
            capabilities.insert(DeepLearningCapability::LayerImplementation);
        }

        // Training
        if desc_lower.contains("train") || desc_lower.contains("training") {
            capabilities.insert(DeepLearningCapability::TrainingPipeline);
        }
        if desc_lower.contains("distributed") {
            capabilities.insert(DeepLearningCapability::DistributedTraining);
        }
        if desc_lower.contains("mixed precision") || desc_lower.contains("fp16") {
            capabilities.insert(DeepLearningCapability::MixedPrecisionTraining);
        }
        if desc_lower.contains("transfer learning") || desc_lower.contains("fine-tune") {
            capabilities.insert(DeepLearningCapability::TransferLearning);
        }

        // Optimization
        if desc_lower.contains("hyperparameter") || desc_lower.contains("tuning") {
            capabilities.insert(DeepLearningCapability::HyperparameterTuning);
        }
        if desc_lower.contains("learning rate") {
            capabilities.insert(DeepLearningCapability::LearningRateScheduling);
        }
        if desc_lower.contains("augment") || desc_lower.contains("augmentation") {
            capabilities.insert(DeepLearningCapability::DataAugmentation);
        }

        // Compression
        if desc_lower.contains("prune") || desc_lower.contains("pruning") {
            capabilities.insert(DeepLearningCapability::Pruning);
        }
        if desc_lower.contains("quantiz") {
            capabilities.insert(DeepLearningCapability::Quantization);
        }
        if desc_lower.contains("distill") {
            capabilities.insert(DeepLearningCapability::KnowledgeDistillation);
        }

        // Deployment
        if desc_lower.contains("deploy") || desc_lower.contains("deployment") {
            capabilities.insert(DeepLearningCapability::ModelExport);
        }
        if desc_lower.contains("onnx") {
            capabilities.insert(DeepLearningCapability::ONNXConversion);
        }
        if desc_lower.contains("tensorrt") {
            capabilities.insert(DeepLearningCapability::TensorRTOptimization);
        }
        if desc_lower.contains("edge") || desc_lower.contains("mobile") {
            capabilities.insert(DeepLearningCapability::EdgeDeployment);
        }

        // Experiment Management
        if desc_lower.contains("experiment") || desc_lower.contains("tracking") {
            capabilities.insert(DeepLearningCapability::ExperimentTracking);
        }
        if desc_lower.contains("visualiz") {
            capabilities.insert(DeepLearningCapability::Visualization);
        }

        // Data
        if desc_lower.contains("data pipeline") || desc_lower.contains("dataloader") {
            capabilities.insert(DeepLearningCapability::DataPipeline);
        }

        // Debugging
        if desc_lower.contains("debug") || desc_lower.contains("gradient") {
            capabilities.insert(DeepLearningCapability::GradientAnalysis);
        }
        if desc_lower.contains("profil") {
            capabilities.insert(DeepLearningCapability::ModelProfiling);
        }

        capabilities
    }

    /// Infer application domains from task description
    fn infer_application_domains(&self, description: &str) -> HashSet<ApplicationDomain> {
        let mut domains = HashSet::new();
        let desc_lower = description.to_lowercase();

        if desc_lower.contains("vision")
            || desc_lower.contains("image")
            || desc_lower.contains("video")
        {
            domains.insert(ApplicationDomain::ComputerVision);
        }

        if desc_lower.contains("nlp")
            || desc_lower.contains("language")
            || desc_lower.contains("text")
        {
            domains.insert(ApplicationDomain::NaturalLanguageProcessing);
        }

        if desc_lower.contains("speech") || desc_lower.contains("audio") {
            domains.insert(ApplicationDomain::SpeechRecognition);
        }

        if desc_lower.contains("recommend") {
            domains.insert(ApplicationDomain::RecommendationSystems);
        }

        if desc_lower.contains("time series") || desc_lower.contains("forecast") {
            domains.insert(ApplicationDomain::TimeSeriesForecasting);
        }

        if desc_lower.contains("anomaly") {
            domains.insert(ApplicationDomain::AnomalyDetection);
        }

        if desc_lower.contains("medical") || desc_lower.contains("healthcare") {
            domains.insert(ApplicationDomain::MedicalImaging);
        }

        if desc_lower.contains("autonomous") || desc_lower.contains("driving") {
            domains.insert(ApplicationDomain::AutonomousDriving);
        }

        domains
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

        if desc_lower.contains("test") {
            capabilities.insert(AgentCapability::Testing);
        }

        if desc_lower.contains("debug") {
            capabilities.insert(AgentCapability::Debugging);
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

    /// Plan task delegation for deep learning tasks
    async fn plan_dl_delegation(
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
            let id = format!("dl-task-{next_id}");
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
        frameworks: &HashSet<DeepLearningFramework>,
        architectures: &HashSet<ModelArchitecture>,
        dl_caps: &HashSet<DeepLearningCapability>,
        domains: &HashSet<ApplicationDomain>,
    ) -> String {
        let mut report = String::from("Deep Learning Task Plan\n");
        report.push_str(&"=".repeat(50));
        report.push('\n');

        // Frameworks detected
        if !frameworks.is_empty() {
            report.push_str("\nDetected Frameworks:\n");
            for framework in frameworks {
                report.push_str(&format!("  • {}\n", framework));
            }
        }

        // Architectures detected
        if !architectures.is_empty() {
            report.push_str("\nDetected Model Architectures:\n");
            for arch in architectures {
                report.push_str(&format!("  • {}\n", arch));
            }
        }

        // Application domains
        if !domains.is_empty() {
            report.push_str("\nApplication Domains:\n");
            for domain in domains {
                report.push_str(&format!("  • {:?}\n", domain));
            }
        }

        // Deep learning capabilities
        if !dl_caps.is_empty() {
            report.push_str("\nRequired DL Capabilities:\n");
            for cap in dl_caps {
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
impl SessionTask for DeepLearningCoordinator {
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
                return Some(format!("Error initializing deep learning agents: {e}"));
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
        let frameworks = if self.config.auto_infer {
            self.infer_frameworks(&message)
        } else {
            HashSet::new()
        };

        let architectures = if self.config.auto_infer {
            self.infer_architectures(&message)
        } else {
            HashSet::new()
        };

        let dl_caps = if self.config.auto_infer {
            self.infer_dl_capabilities(&message)
        } else {
            HashSet::new()
        };

        let domains = if self.config.auto_infer {
            self.infer_application_domains(&message)
        } else {
            HashSet::new()
        };

        // Plan task delegation
        let tasks = match self.plan_dl_delegation(&input).await {
            Ok(t) => t,
            Err(e) => {
                return Some(format!("Error planning task delegation: {e}"));
            }
        };

        let elapsed = start_time.elapsed();

        // Generate execution plan
        let mut report =
            self.generate_plan_summary(&tasks, &frameworks, &architectures, &dl_caps, &domains);
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
    use crate::multi_agent::deep_learning_specialist::presets;

    #[test]
    fn test_infer_pytorch() {
        let config = DeepLearningCoordinatorConfig::default();
        let coordinator = DeepLearningCoordinator::new(config);

        let frameworks = coordinator.infer_frameworks("Implement PyTorch transformer model");
        assert!(frameworks.contains(&DeepLearningFramework::PyTorch));
    }

    #[test]
    fn test_infer_transformer() {
        let config = DeepLearningCoordinatorConfig::default();
        let coordinator = DeepLearningCoordinator::new(config);

        let archs = coordinator.infer_architectures("Build a BERT model for NLP");
        assert!(archs.contains(&ModelArchitecture::BERT));
    }

    #[test]
    fn test_infer_training_capabilities() {
        let config = DeepLearningCoordinatorConfig::default();
        let coordinator = DeepLearningCoordinator::new(config);

        let caps = coordinator
            .infer_dl_capabilities("Train model with distributed training and mixed precision");
        assert!(caps.contains(&DeepLearningCapability::DistributedTraining));
        assert!(caps.contains(&DeepLearningCapability::MixedPrecisionTraining));
    }

    #[tokio::test]
    async fn test_coordinator_initialization() {
        let config = DeepLearningCoordinatorConfig {
            dl_experts: vec![presets::pytorch_architect()],
            ..Default::default()
        };

        let coordinator = DeepLearningCoordinator::new(config);
        let pool = coordinator.initialize_agents().await.unwrap();
        let stats = pool.get_stats().await;

        assert_eq!(stats.total_agents, 1);
    }
}
