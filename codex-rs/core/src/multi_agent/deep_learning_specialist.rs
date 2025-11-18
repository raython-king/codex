//! Deep learning specialist agents
//!
//! This module provides specialized agents for deep learning development,
//! with fine-grained capabilities for model architecture, training, optimization,
//! and deployment.

use super::types::{AgentCapability, AgentRole, AgentSpec};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;

/// Deep learning frameworks
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeepLearningFramework {
    PyTorch,
    TensorFlow,
    JAX,
    Keras,
    MXNet,
    Caffe,
    ONNX,
    TensorRT,
    CoreML,
    TFLite,
    Custom(String),
}

impl fmt::Display for DeepLearningFramework {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeepLearningFramework::PyTorch => write!(f, "PyTorch"),
            DeepLearningFramework::TensorFlow => write!(f, "TensorFlow"),
            DeepLearningFramework::JAX => write!(f, "JAX"),
            DeepLearningFramework::Keras => write!(f, "Keras"),
            DeepLearningFramework::MXNet => write!(f, "MXNet"),
            DeepLearningFramework::Caffe => write!(f, "Caffe"),
            DeepLearningFramework::ONNX => write!(f, "ONNX"),
            DeepLearningFramework::TensorRT => write!(f, "TensorRT"),
            DeepLearningFramework::CoreML => write!(f, "CoreML"),
            DeepLearningFramework::TFLite => write!(f, "TFLite"),
            DeepLearningFramework::Custom(name) => write!(f, "{}", name),
        }
    }
}

/// Model architectures
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModelArchitecture {
    // Convolutional Networks
    CNN,
    ResNet,
    VGG,
    Inception,
    EfficientNet,
    MobileNet,

    // Recurrent Networks
    RNN,
    LSTM,
    GRU,
    BiLSTM,

    // Transformer-based
    Transformer,
    BERT,
    GPT,
    T5,
    ViT,
    CLIP,
    DallE,

    // Generative Models
    GAN,
    VAE,
    Diffusion,
    AutoEncoder,

    // Object Detection
    YOLO,
    RCNN,
    SSD,
    RetinaNet,

    // Segmentation
    UNet,
    MaskRCNN,
    DeepLab,

    // Custom
    Custom(String),
}

impl fmt::Display for ModelArchitecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Deep learning capabilities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeepLearningCapability {
    // Model Development
    ArchitectureDesign,
    LayerImplementation,
    CustomOperators,
    ModelPrototyping,

    // Training
    TrainingPipeline,
    DistributedTraining,
    MixedPrecisionTraining,
    GradientAccumulation,
    CurriculumLearning,
    TransferLearning,
    FewShotLearning,
    MetaLearning,

    // Optimization
    HyperparameterTuning,
    LearningRateScheduling,
    RegularizationTechniques,
    DataAugmentation,
    BatchNormalization,
    DropoutStrategies,

    // Model Compression
    Pruning,
    Quantization,
    KnowledgeDistillation,
    NeuralArchitectureSearch,

    // Evaluation
    ModelEvaluation,
    BenchmarkTesting,
    AblationStudy,
    ErrorAnalysis,

    // Deployment
    ModelExport,
    ONNXConversion,
    TensorRTOptimization,
    EdgeDeployment,
    MobileDeployment,
    ServerDeployment,

    // Experiment Management
    ExperimentTracking,
    MetricsLogging,
    Visualization,
    CheckpointManagement,
    ReproducibilityEnsurance,

    // Data Processing
    DataPipeline,
    PreprocessingPipeline,
    DataLoading,
    DataParallelism,

    // Debugging
    GradientAnalysis,
    ActivationVisualization,
    ModelProfiling,
    MemoryOptimization,

    Custom(String),
}

impl fmt::Display for DeepLearningCapability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeepLearningCapability::ArchitectureDesign => write!(f, "architecture-design"),
            DeepLearningCapability::LayerImplementation => write!(f, "layer-implementation"),
            DeepLearningCapability::CustomOperators => write!(f, "custom-operators"),
            DeepLearningCapability::ModelPrototyping => write!(f, "model-prototyping"),
            DeepLearningCapability::TrainingPipeline => write!(f, "training-pipeline"),
            DeepLearningCapability::DistributedTraining => write!(f, "distributed-training"),
            DeepLearningCapability::MixedPrecisionTraining => write!(f, "mixed-precision-training"),
            DeepLearningCapability::GradientAccumulation => write!(f, "gradient-accumulation"),
            DeepLearningCapability::CurriculumLearning => write!(f, "curriculum-learning"),
            DeepLearningCapability::TransferLearning => write!(f, "transfer-learning"),
            DeepLearningCapability::FewShotLearning => write!(f, "few-shot-learning"),
            DeepLearningCapability::MetaLearning => write!(f, "meta-learning"),
            DeepLearningCapability::HyperparameterTuning => write!(f, "hyperparameter-tuning"),
            DeepLearningCapability::LearningRateScheduling => write!(f, "lr-scheduling"),
            DeepLearningCapability::RegularizationTechniques => write!(f, "regularization"),
            DeepLearningCapability::DataAugmentation => write!(f, "data-augmentation"),
            DeepLearningCapability::BatchNormalization => write!(f, "batch-normalization"),
            DeepLearningCapability::DropoutStrategies => write!(f, "dropout-strategies"),
            DeepLearningCapability::Pruning => write!(f, "pruning"),
            DeepLearningCapability::Quantization => write!(f, "quantization"),
            DeepLearningCapability::KnowledgeDistillation => write!(f, "knowledge-distillation"),
            DeepLearningCapability::NeuralArchitectureSearch => write!(f, "nas"),
            DeepLearningCapability::ModelEvaluation => write!(f, "model-evaluation"),
            DeepLearningCapability::BenchmarkTesting => write!(f, "benchmark-testing"),
            DeepLearningCapability::AblationStudy => write!(f, "ablation-study"),
            DeepLearningCapability::ErrorAnalysis => write!(f, "error-analysis"),
            DeepLearningCapability::ModelExport => write!(f, "model-export"),
            DeepLearningCapability::ONNXConversion => write!(f, "onnx-conversion"),
            DeepLearningCapability::TensorRTOptimization => write!(f, "tensorrt-optimization"),
            DeepLearningCapability::EdgeDeployment => write!(f, "edge-deployment"),
            DeepLearningCapability::MobileDeployment => write!(f, "mobile-deployment"),
            DeepLearningCapability::ServerDeployment => write!(f, "server-deployment"),
            DeepLearningCapability::ExperimentTracking => write!(f, "experiment-tracking"),
            DeepLearningCapability::MetricsLogging => write!(f, "metrics-logging"),
            DeepLearningCapability::Visualization => write!(f, "visualization"),
            DeepLearningCapability::CheckpointManagement => write!(f, "checkpoint-management"),
            DeepLearningCapability::ReproducibilityEnsurance => {
                write!(f, "reproducibility-ensurance")
            }
            DeepLearningCapability::DataPipeline => write!(f, "data-pipeline"),
            DeepLearningCapability::PreprocessingPipeline => write!(f, "preprocessing-pipeline"),
            DeepLearningCapability::DataLoading => write!(f, "data-loading"),
            DeepLearningCapability::DataParallelism => write!(f, "data-parallelism"),
            DeepLearningCapability::GradientAnalysis => write!(f, "gradient-analysis"),
            DeepLearningCapability::ActivationVisualization => {
                write!(f, "activation-visualization")
            }
            DeepLearningCapability::ModelProfiling => write!(f, "model-profiling"),
            DeepLearningCapability::MemoryOptimization => write!(f, "memory-optimization"),
            DeepLearningCapability::Custom(name) => write!(f, "custom:{}", name),
        }
    }
}

/// Training strategies
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrainingStrategy {
    StandardTraining,
    DistributedDataParallel,
    DistributedModelParallel,
    PipelineParallelism,
    MixedPrecision,
    GradientCheckpointing,
    OnlineLearning,
    IncrementalLearning,
    ActiveLearning,
    Custom(String),
}

/// Application domain
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ApplicationDomain {
    ComputerVision,
    NaturalLanguageProcessing,
    SpeechRecognition,
    RecommendationSystems,
    TimeSeriesForecasting,
    AnomalyDetection,
    MedicalImaging,
    AutonomousDriving,
    Robotics,
    GameAI,
    Custom(String),
}

/// Deep learning specialist agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepLearningAgent {
    /// Base agent specification
    pub spec: AgentSpec,
    /// Deep learning capabilities
    pub dl_capabilities: HashSet<DeepLearningCapability>,
    /// Frameworks expertise
    pub frameworks: HashSet<DeepLearningFramework>,
    /// Model architectures expertise
    pub architectures: HashSet<ModelArchitecture>,
    /// Training strategies
    pub training_strategies: HashSet<TrainingStrategy>,
    /// Application domains
    pub application_domains: HashSet<ApplicationDomain>,
    /// Libraries and tools
    pub libraries: Vec<String>,
}

impl DeepLearningAgent {
    /// Create a new deep learning agent
    pub fn new(id: impl Into<String>, role: AgentRole) -> Self {
        let id_string = id.into();
        Self {
            spec: AgentSpec::new(id_string, role),
            dl_capabilities: HashSet::new(),
            frameworks: HashSet::new(),
            architectures: HashSet::new(),
            training_strategies: HashSet::new(),
            application_domains: HashSet::new(),
            libraries: Vec::new(),
        }
    }

    /// Add deep learning capability
    pub fn with_dl_capability(mut self, capability: DeepLearningCapability) -> Self {
        self.dl_capabilities.insert(capability);
        self
    }

    /// Add multiple deep learning capabilities
    pub fn with_dl_capabilities(mut self, capabilities: Vec<DeepLearningCapability>) -> Self {
        self.dl_capabilities.extend(capabilities);
        self
    }

    /// Add framework
    pub fn with_framework(mut self, framework: DeepLearningFramework) -> Self {
        self.frameworks.insert(framework);
        self
    }

    /// Add architecture
    pub fn with_architecture(mut self, architecture: ModelArchitecture) -> Self {
        self.architectures.insert(architecture);
        self
    }

    /// Add multiple architectures
    pub fn with_architectures(mut self, architectures: Vec<ModelArchitecture>) -> Self {
        self.architectures.extend(architectures);
        self
    }

    /// Add training strategy
    pub fn with_training_strategy(mut self, strategy: TrainingStrategy) -> Self {
        self.training_strategies.insert(strategy);
        self
    }

    /// Add application domain
    pub fn with_application_domain(mut self, domain: ApplicationDomain) -> Self {
        self.application_domains.insert(domain);
        self
    }

    /// Add library
    pub fn with_library(mut self, library: impl Into<String>) -> Self {
        self.libraries.push(library.into());
        self
    }

    /// Add general agent capability
    pub fn with_capability(mut self, capability: AgentCapability) -> Self {
        self.spec.capabilities.insert(capability);
        self
    }

    /// Set instructions
    pub fn with_instructions(mut self, instructions: impl Into<String>) -> Self {
        self.spec.instructions = Some(instructions.into());
        self
    }

    /// Set priority
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.spec.priority = priority;
        self
    }

    /// Set max concurrent tasks
    pub fn with_max_concurrent_tasks(mut self, max: usize) -> Self {
        self.spec.max_concurrent_tasks = max;
        self
    }

    /// Convert to generic agent spec
    pub fn into_spec(self) -> AgentSpec {
        self.spec
    }
}

/// Pre-configured deep learning expert presets
pub mod presets {
    use super::*;

    /// PyTorch architecture expert
    pub fn pytorch_architect() -> DeepLearningAgent {
        DeepLearningAgent::new(
            "pytorch-architect",
            AgentRole::Specialist {
                domain: "pytorch-architecture".to_string(),
            },
        )
        .with_framework(DeepLearningFramework::PyTorch)
        .with_dl_capabilities(vec![
            DeepLearningCapability::ArchitectureDesign,
            DeepLearningCapability::LayerImplementation,
            DeepLearningCapability::CustomOperators,
            DeepLearningCapability::ModelPrototyping,
        ])
        .with_architectures(vec![
            ModelArchitecture::Transformer,
            ModelArchitecture::ResNet,
            ModelArchitecture::CNN,
            ModelArchitecture::GAN,
        ])
        .with_capability(AgentCapability::CodeGeneration)
        .with_library("torch")
        .with_library("torch.nn")
        .with_library("torchvision")
        .with_instructions(
            "PyTorch architecture expert specializing in model design and custom layer implementation",
        )
        .with_priority(9)
    }

    /// Training specialist
    pub fn training_specialist() -> DeepLearningAgent {
        DeepLearningAgent::new(
            "training-specialist",
            AgentRole::Specialist {
                domain: "training".to_string(),
            },
        )
        .with_framework(DeepLearningFramework::PyTorch)
        .with_framework(DeepLearningFramework::TensorFlow)
        .with_dl_capabilities(vec![
            DeepLearningCapability::TrainingPipeline,
            DeepLearningCapability::DistributedTraining,
            DeepLearningCapability::MixedPrecisionTraining,
            DeepLearningCapability::GradientAccumulation,
            DeepLearningCapability::LearningRateScheduling,
        ])
        .with_training_strategy(TrainingStrategy::DistributedDataParallel)
        .with_training_strategy(TrainingStrategy::MixedPrecision)
        .with_capability(AgentCapability::CodeGeneration)
        .with_library("torch.distributed")
        .with_library("torch.cuda.amp")
        .with_library("tf.distribute")
        .with_instructions("Training expert for distributed and optimized model training")
        .with_priority(10)
    }

    /// Computer vision expert
    pub fn vision_expert() -> DeepLearningAgent {
        DeepLearningAgent::new(
            "vision-expert",
            AgentRole::Specialist {
                domain: "computer-vision".to_string(),
            },
        )
        .with_framework(DeepLearningFramework::PyTorch)
        .with_application_domain(ApplicationDomain::ComputerVision)
        .with_architectures(vec![
            ModelArchitecture::CNN,
            ModelArchitecture::ResNet,
            ModelArchitecture::YOLO,
            ModelArchitecture::UNet,
            ModelArchitecture::ViT,
        ])
        .with_dl_capabilities(vec![
            DeepLearningCapability::ArchitectureDesign,
            DeepLearningCapability::DataAugmentation,
            DeepLearningCapability::TransferLearning,
        ])
        .with_capability(AgentCapability::CodeGeneration)
        .with_library("torchvision")
        .with_library("opencv-python")
        .with_library("albumentations")
        .with_instructions("Computer vision expert specializing in image-based deep learning")
        .with_priority(9)
    }

    /// Transformer/NLP expert
    pub fn transformer_expert() -> DeepLearningAgent {
        DeepLearningAgent::new(
            "transformer-expert",
            AgentRole::Specialist {
                domain: "transformers".to_string(),
            },
        )
        .with_framework(DeepLearningFramework::PyTorch)
        .with_application_domain(ApplicationDomain::NaturalLanguageProcessing)
        .with_architectures(vec![
            ModelArchitecture::Transformer,
            ModelArchitecture::BERT,
            ModelArchitecture::GPT,
            ModelArchitecture::T5,
        ])
        .with_dl_capabilities(vec![
            DeepLearningCapability::ArchitectureDesign,
            DeepLearningCapability::TransferLearning,
            DeepLearningCapability::FewShotLearning,
        ])
        .with_capability(AgentCapability::CodeGeneration)
        .with_library("transformers")
        .with_library("tokenizers")
        .with_library("datasets")
        .with_instructions("Transformer architecture expert for NLP and multimodal tasks")
        .with_priority(10)
    }

    /// Model optimization expert
    pub fn optimization_expert() -> DeepLearningAgent {
        DeepLearningAgent::new(
            "optimization-expert",
            AgentRole::Specialist {
                domain: "optimization".to_string(),
            },
        )
        .with_framework(DeepLearningFramework::PyTorch)
        .with_framework(DeepLearningFramework::ONNX)
        .with_framework(DeepLearningFramework::TensorRT)
        .with_dl_capabilities(vec![
            DeepLearningCapability::Pruning,
            DeepLearningCapability::Quantization,
            DeepLearningCapability::KnowledgeDistillation,
            DeepLearningCapability::ModelProfiling,
            DeepLearningCapability::MemoryOptimization,
        ])
        .with_capability(AgentCapability::CodeGeneration)
        .with_library("torch.quantization")
        .with_library("torch.onnx")
        .with_library("onnxruntime")
        .with_instructions("Model optimization expert for compression and efficiency")
        .with_priority(8)
    }

    /// Deployment specialist
    pub fn deployment_specialist() -> DeepLearningAgent {
        DeepLearningAgent::new(
            "deployment-specialist",
            AgentRole::Specialist {
                domain: "deployment".to_string(),
            },
        )
        .with_framework(DeepLearningFramework::ONNX)
        .with_framework(DeepLearningFramework::TensorRT)
        .with_framework(DeepLearningFramework::TFLite)
        .with_framework(DeepLearningFramework::CoreML)
        .with_dl_capabilities(vec![
            DeepLearningCapability::ModelExport,
            DeepLearningCapability::ONNXConversion,
            DeepLearningCapability::TensorRTOptimization,
            DeepLearningCapability::EdgeDeployment,
            DeepLearningCapability::MobileDeployment,
        ])
        .with_capability(AgentCapability::CodeGeneration)
        .with_library("onnx")
        .with_library("tensorrt")
        .with_library("tensorflow-lite")
        .with_instructions("Deployment expert for production and edge inference")
        .with_priority(8)
    }

    /// Experiment tracking specialist
    pub fn experiment_specialist() -> DeepLearningAgent {
        DeepLearningAgent::new(
            "experiment-specialist",
            AgentRole::Specialist {
                domain: "experiments".to_string(),
            },
        )
        .with_framework(DeepLearningFramework::PyTorch)
        .with_framework(DeepLearningFramework::TensorFlow)
        .with_dl_capabilities(vec![
            DeepLearningCapability::ExperimentTracking,
            DeepLearningCapability::MetricsLogging,
            DeepLearningCapability::Visualization,
            DeepLearningCapability::CheckpointManagement,
            DeepLearningCapability::ReproducibilityEnsurance,
        ])
        .with_capability(AgentCapability::CodeGeneration)
        .with_library("wandb")
        .with_library("tensorboard")
        .with_library("mlflow")
        .with_instructions("Experiment management and reproducibility expert")
        .with_priority(7)
    }

    /// Data pipeline expert
    pub fn data_pipeline_expert() -> DeepLearningAgent {
        DeepLearningAgent::new(
            "data-pipeline-expert",
            AgentRole::Specialist {
                domain: "data-pipeline".to_string(),
            },
        )
        .with_framework(DeepLearningFramework::PyTorch)
        .with_framework(DeepLearningFramework::TensorFlow)
        .with_dl_capabilities(vec![
            DeepLearningCapability::DataPipeline,
            DeepLearningCapability::PreprocessingPipeline,
            DeepLearningCapability::DataLoading,
            DeepLearningCapability::DataAugmentation,
            DeepLearningCapability::DataParallelism,
        ])
        .with_capability(AgentCapability::CodeGeneration)
        .with_library("torch.utils.data")
        .with_library("tf.data")
        .with_library("albumentations")
        .with_library("torchdata")
        .with_instructions("Data pipeline optimization and preprocessing expert")
        .with_priority(8)
    }

    /// Hyperparameter tuning expert
    pub fn tuning_expert() -> DeepLearningAgent {
        DeepLearningAgent::new(
            "tuning-expert",
            AgentRole::Specialist {
                domain: "hyperparameter-tuning".to_string(),
            },
        )
        .with_dl_capabilities(vec![
            DeepLearningCapability::HyperparameterTuning,
            DeepLearningCapability::NeuralArchitectureSearch,
            DeepLearningCapability::ModelEvaluation,
            DeepLearningCapability::AblationStudy,
        ])
        .with_capability(AgentCapability::CodeGeneration)
        .with_library("optuna")
        .with_library("ray[tune]")
        .with_library("hyperopt")
        .with_instructions("Hyperparameter optimization and AutoML expert")
        .with_priority(7)
    }

    /// Model debugging expert
    pub fn debugging_expert() -> DeepLearningAgent {
        DeepLearningAgent::new(
            "debugging-expert",
            AgentRole::Specialist {
                domain: "debugging".to_string(),
            },
        )
        .with_framework(DeepLearningFramework::PyTorch)
        .with_dl_capabilities(vec![
            DeepLearningCapability::GradientAnalysis,
            DeepLearningCapability::ActivationVisualization,
            DeepLearningCapability::ModelProfiling,
            DeepLearningCapability::ErrorAnalysis,
        ])
        .with_capability(AgentCapability::Debugging)
        .with_library("torch.autograd")
        .with_library("tensorwatch")
        .with_library("captum")
        .with_instructions("Model debugging and interpretability expert")
        .with_priority(7)
    }

    /// Get all deep learning experts
    pub fn all_dl_experts() -> Vec<DeepLearningAgent> {
        vec![
            pytorch_architect(),
            training_specialist(),
            vision_expert(),
            transformer_expert(),
            optimization_expert(),
            deployment_specialist(),
            experiment_specialist(),
            data_pipeline_expert(),
            tuning_expert(),
            debugging_expert(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_pytorch_architect() {
        let agent = presets::pytorch_architect();
        assert_eq!(agent.spec.id.0, "pytorch-architect");
        assert!(agent.frameworks.contains(&DeepLearningFramework::PyTorch));
        assert!(agent
            .dl_capabilities
            .contains(&DeepLearningCapability::ArchitectureDesign));
    }

    #[test]
    fn test_create_training_specialist() {
        let agent = presets::training_specialist();
        assert!(agent
            .dl_capabilities
            .contains(&DeepLearningCapability::DistributedTraining));
        assert!(agent
            .training_strategies
            .contains(&TrainingStrategy::DistributedDataParallel));
    }

    #[test]
    fn test_all_experts_count() {
        let experts = presets::all_dl_experts();
        assert_eq!(experts.len(), 10);
    }

    #[test]
    fn test_custom_agent() {
        let agent = DeepLearningAgent::new(
            "custom-expert",
            AgentRole::Specialist {
                domain: "custom".to_string(),
            },
        )
        .with_framework(DeepLearningFramework::JAX)
        .with_dl_capability(DeepLearningCapability::ArchitectureDesign)
        .with_architecture(ModelArchitecture::Transformer)
        .with_library("jax")
        .with_priority(10);

        assert_eq!(agent.spec.id.0, "custom-expert");
        assert!(agent.frameworks.contains(&DeepLearningFramework::JAX));
        assert_eq!(agent.spec.priority, 10);
    }
}
