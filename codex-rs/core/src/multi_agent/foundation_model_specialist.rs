//! Foundation model and scaling law specialists
//!
//! This module provides specialized agents for foundation model development,
//! scaling law research, and large-scale model training for computer vision
//! and multimodal tasks.

use super::types::{AgentCapability, AgentRole, AgentSpec};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;

/// Foundation model architectures
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FoundationModelArchitecture {
    // Vision Foundation Models
    CLIP,
    ALIGN,
    Florence,
    SAM,              // Segment Anything Model
    SAM2,             // Segment Anything Model 2 (video + image)
    DINOv2,
    GroundingDINO,    // Grounding DINO for open-vocabulary detection
    MAE,              // Masked Autoencoder
    BEiT,
    SimCLR,
    MoCo,
    DINO,
    DEIT,
    SwinTransformer,

    // Depth Estimation Models
    DepthAnything,    // Depth Anything V1
    DepthAnythingV2,  // Depth Anything V2
    DepthAnythingV3,  // Depth Anything V3 (latest)
    ZoeDepth,
    MiDaS,

    // Multimodal Foundation Models
    Flamingo,
    BLIP,
    BLIP2,
    GPT4Vision,
    LLaVA,
    Kosmos,
    PaLmE,

    // Video Foundation Models
    VideoMAE,
    Omnivore,
    VideoSwin,

    // 3D Vision
    PointBERT,
    PointMae,

    Custom(String),
}

impl fmt::Display for FoundationModelArchitecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Pretraining strategies for foundation models
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PretrainingStrategy {
    // Self-supervised
    MaskedImageModeling,      // MAE, BEiT
    ContrastiveLearning,      // CLIP, SimCLR, MoCo
    DenoisingAutoencoder,
    RotationPrediction,
    JigsawPuzzle,

    // Multimodal
    VisionLanguageAlignment,  // CLIP, ALIGN
    ImageTextMatching,
    MaskedLanguageModeling,

    // Semi-supervised
    PseudoLabeling,
    ConsistencyRegularization,

    // Supervised
    LargeScaleSupervised,

    Custom(String),
}

impl fmt::Display for PretrainingStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PretrainingStrategy::MaskedImageModeling => write!(f, "masked-image-modeling"),
            PretrainingStrategy::ContrastiveLearning => write!(f, "contrastive-learning"),
            PretrainingStrategy::DenoisingAutoencoder => write!(f, "denoising-autoencoder"),
            PretrainingStrategy::RotationPrediction => write!(f, "rotation-prediction"),
            PretrainingStrategy::JigsawPuzzle => write!(f, "jigsaw-puzzle"),
            PretrainingStrategy::VisionLanguageAlignment => write!(f, "vision-language-alignment"),
            PretrainingStrategy::ImageTextMatching => write!(f, "image-text-matching"),
            PretrainingStrategy::MaskedLanguageModeling => write!(f, "masked-language-modeling"),
            PretrainingStrategy::PseudoLabeling => write!(f, "pseudo-labeling"),
            PretrainingStrategy::ConsistencyRegularization => write!(f, "consistency-regularization"),
            PretrainingStrategy::LargeScaleSupervised => write!(f, "large-scale-supervised"),
            PretrainingStrategy::Custom(name) => write!(f, "custom:{}", name),
        }
    }
}

/// Foundation model capabilities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FoundationModelCapability {
    // Model Development
    FoundationModelDesign,
    VisionTransformerDesign,
    MultimodalArchitecture,
    AttentionMechanisms,

    // Pretraining
    SelfSupervisedPretraining,
    ContrastivePretraining,
    MaskedPretraining,
    MultimodalPretraining,
    LargeScalePretraining,

    // Transfer Learning
    ZeroShotTransfer,
    FewShotTransfer,
    LinearProbing,
    FullFineTuning,
    ParameterEfficientFineTuning,  // LoRA, Adapter, Prefix-tuning
    PromptTuning,

    // Data Processing
    WebScaleDataCuration,
    DataFiltering,
    DataDeduplication,
    MultimodalDataAlignment,
    DataAugmentationAtScale,

    // Scaling Laws
    ComputeOptimalScaling,
    DataScaling,
    ModelScaling,
    ScalingLawAnalysis,
    EmergentAbilityDetection,

    // Evaluation
    ZeroShotEvaluation,
    FewShotEvaluation,
    TransferLearningEvaluation,
    RobustnessEvaluation,
    DistributionShiftEvaluation,

    // Efficiency
    EfficientAttention,
    GradientCheckpointing,
    MixedPrecisionTraining,
    ModelParallelism,
    DataParallelism,
    PipelineParallelism,
    ZeROOptimization,

    // Interpretability
    AttentionVisualization,
    FeatureVisualization,
    ConceptDiscovery,
    BiasDetection,

    // Dense Prediction
    MonocularDepthEstimation,
    MetricDepthEstimation,
    RelativeDepthEstimation,
    ZeroShotDepthEstimation,

    // Open-Vocabulary Understanding
    OpenVocabularyDetection,
    OpenVocabularySegmentation,
    GroundingCapability,
    ReferringSegmentation,
    TextGroundedUnderstanding,

    // Video Understanding
    VideoSegmentation,
    VideoObjectTracking,
    TemporalConsistency,

    Custom(String),
}

impl fmt::Display for FoundationModelCapability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FoundationModelCapability::FoundationModelDesign => write!(f, "foundation-model-design"),
            FoundationModelCapability::VisionTransformerDesign => write!(f, "vit-design"),
            FoundationModelCapability::MultimodalArchitecture => write!(f, "multimodal-architecture"),
            FoundationModelCapability::AttentionMechanisms => write!(f, "attention-mechanisms"),
            FoundationModelCapability::SelfSupervisedPretraining => write!(f, "self-supervised-pretraining"),
            FoundationModelCapability::ContrastivePretraining => write!(f, "contrastive-pretraining"),
            FoundationModelCapability::MaskedPretraining => write!(f, "masked-pretraining"),
            FoundationModelCapability::MultimodalPretraining => write!(f, "multimodal-pretraining"),
            FoundationModelCapability::LargeScalePretraining => write!(f, "large-scale-pretraining"),
            FoundationModelCapability::ZeroShotTransfer => write!(f, "zero-shot-transfer"),
            FoundationModelCapability::FewShotTransfer => write!(f, "few-shot-transfer"),
            FoundationModelCapability::LinearProbing => write!(f, "linear-probing"),
            FoundationModelCapability::FullFineTuning => write!(f, "full-fine-tuning"),
            FoundationModelCapability::ParameterEfficientFineTuning => write!(f, "peft"),
            FoundationModelCapability::PromptTuning => write!(f, "prompt-tuning"),
            FoundationModelCapability::WebScaleDataCuration => write!(f, "web-scale-data-curation"),
            FoundationModelCapability::DataFiltering => write!(f, "data-filtering"),
            FoundationModelCapability::DataDeduplication => write!(f, "data-deduplication"),
            FoundationModelCapability::MultimodalDataAlignment => write!(f, "multimodal-data-alignment"),
            FoundationModelCapability::DataAugmentationAtScale => write!(f, "data-augmentation-at-scale"),
            FoundationModelCapability::ComputeOptimalScaling => write!(f, "compute-optimal-scaling"),
            FoundationModelCapability::DataScaling => write!(f, "data-scaling"),
            FoundationModelCapability::ModelScaling => write!(f, "model-scaling"),
            FoundationModelCapability::ScalingLawAnalysis => write!(f, "scaling-law-analysis"),
            FoundationModelCapability::EmergentAbilityDetection => write!(f, "emergent-ability-detection"),
            FoundationModelCapability::ZeroShotEvaluation => write!(f, "zero-shot-evaluation"),
            FoundationModelCapability::FewShotEvaluation => write!(f, "few-shot-evaluation"),
            FoundationModelCapability::TransferLearningEvaluation => write!(f, "transfer-learning-evaluation"),
            FoundationModelCapability::RobustnessEvaluation => write!(f, "robustness-evaluation"),
            FoundationModelCapability::DistributionShiftEvaluation => write!(f, "distribution-shift-evaluation"),
            FoundationModelCapability::EfficientAttention => write!(f, "efficient-attention"),
            FoundationModelCapability::GradientCheckpointing => write!(f, "gradient-checkpointing"),
            FoundationModelCapability::MixedPrecisionTraining => write!(f, "mixed-precision-training"),
            FoundationModelCapability::ModelParallelism => write!(f, "model-parallelism"),
            FoundationModelCapability::DataParallelism => write!(f, "data-parallelism"),
            FoundationModelCapability::PipelineParallelism => write!(f, "pipeline-parallelism"),
            FoundationModelCapability::ZeROOptimization => write!(f, "zero-optimization"),
            FoundationModelCapability::AttentionVisualization => write!(f, "attention-visualization"),
            FoundationModelCapability::FeatureVisualization => write!(f, "feature-visualization"),
            FoundationModelCapability::ConceptDiscovery => write!(f, "concept-discovery"),
            FoundationModelCapability::BiasDetection => write!(f, "bias-detection"),
            FoundationModelCapability::MonocularDepthEstimation => write!(f, "monocular-depth-estimation"),
            FoundationModelCapability::MetricDepthEstimation => write!(f, "metric-depth-estimation"),
            FoundationModelCapability::RelativeDepthEstimation => write!(f, "relative-depth-estimation"),
            FoundationModelCapability::ZeroShotDepthEstimation => write!(f, "zero-shot-depth-estimation"),
            FoundationModelCapability::OpenVocabularyDetection => write!(f, "open-vocabulary-detection"),
            FoundationModelCapability::OpenVocabularySegmentation => write!(f, "open-vocabulary-segmentation"),
            FoundationModelCapability::GroundingCapability => write!(f, "grounding-capability"),
            FoundationModelCapability::ReferringSegmentation => write!(f, "referring-segmentation"),
            FoundationModelCapability::TextGroundedUnderstanding => write!(f, "text-grounded-understanding"),
            FoundationModelCapability::VideoSegmentation => write!(f, "video-segmentation"),
            FoundationModelCapability::VideoObjectTracking => write!(f, "video-object-tracking"),
            FoundationModelCapability::TemporalConsistency => write!(f, "temporal-consistency"),
            FoundationModelCapability::Custom(name) => write!(f, "custom:{}", name),
        }
    }
}

/// Model scale
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModelScale {
    Tiny,          // < 10M parameters
    Small,         // 10M - 100M
    Base,          // 100M - 500M
    Large,         // 500M - 1B
    XLarge,        // 1B - 10B
    XXLarge,       // 10B - 100B
    Gigantic,      // > 100B
    Custom(String),
}

impl fmt::Display for ModelScale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Dataset scale
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DatasetScale {
    Small,         // < 1M samples
    Medium,        // 1M - 10M
    Large,         // 10M - 100M
    WebScale,      // 100M - 1B
    InternetScale, // > 1B
    Custom(String),
}

/// Foundation model specialist agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoundationModelAgent {
    /// Base agent specification
    pub spec: AgentSpec,
    /// Foundation model capabilities
    pub fm_capabilities: HashSet<FoundationModelCapability>,
    /// Model architectures expertise
    pub architectures: HashSet<FoundationModelArchitecture>,
    /// Pretraining strategies
    pub pretraining_strategies: HashSet<PretrainingStrategy>,
    /// Model scales handled
    pub model_scales: HashSet<ModelScale>,
    /// Dataset scales handled
    pub dataset_scales: HashSet<DatasetScale>,
    /// Libraries and tools
    pub libraries: Vec<String>,
}

impl FoundationModelAgent {
    /// Create a new foundation model agent
    pub fn new(id: impl Into<String>, role: AgentRole) -> Self {
        let id_string = id.into();
        Self {
            spec: AgentSpec::new(id_string, role),
            fm_capabilities: HashSet::new(),
            architectures: HashSet::new(),
            pretraining_strategies: HashSet::new(),
            model_scales: HashSet::new(),
            dataset_scales: HashSet::new(),
            libraries: Vec::new(),
        }
    }

    /// Add foundation model capability
    pub fn with_fm_capability(mut self, capability: FoundationModelCapability) -> Self {
        self.fm_capabilities.insert(capability);
        self
    }

    /// Add multiple foundation model capabilities
    pub fn with_fm_capabilities(mut self, capabilities: Vec<FoundationModelCapability>) -> Self {
        self.fm_capabilities.extend(capabilities);
        self
    }

    /// Add architecture
    pub fn with_architecture(mut self, architecture: FoundationModelArchitecture) -> Self {
        self.architectures.insert(architecture);
        self
    }

    /// Add multiple architectures
    pub fn with_architectures(mut self, architectures: Vec<FoundationModelArchitecture>) -> Self {
        self.architectures.extend(architectures);
        self
    }

    /// Add pretraining strategy
    pub fn with_pretraining_strategy(mut self, strategy: PretrainingStrategy) -> Self {
        self.pretraining_strategies.insert(strategy);
        self
    }

    /// Add model scale
    pub fn with_model_scale(mut self, scale: ModelScale) -> Self {
        self.model_scales.insert(scale);
        self
    }

    /// Add dataset scale
    pub fn with_dataset_scale(mut self, scale: DatasetScale) -> Self {
        self.dataset_scales.insert(scale);
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

/// Pre-configured foundation model expert presets
pub mod presets {
    use super::*;

    /// CLIP and vision-language expert
    pub fn clip_expert() -> FoundationModelAgent {
        FoundationModelAgent::new(
            "clip-expert",
            AgentRole::Specialist {
                domain: "vision-language".to_string(),
            },
        )
        .with_architectures(vec![
            FoundationModelArchitecture::CLIP,
            FoundationModelArchitecture::ALIGN,
            FoundationModelArchitecture::BLIP,
        ])
        .with_fm_capabilities(vec![
            FoundationModelCapability::MultimodalArchitecture,
            FoundationModelCapability::ContrastivePretraining,
            FoundationModelCapability::MultimodalPretraining,
            FoundationModelCapability::ZeroShotTransfer,
        ])
        .with_pretraining_strategy(PretrainingStrategy::ContrastiveLearning)
        .with_pretraining_strategy(PretrainingStrategy::VisionLanguageAlignment)
        .with_model_scale(ModelScale::Base)
        .with_model_scale(ModelScale::Large)
        .with_dataset_scale(DatasetScale::WebScale)
        .with_capability(AgentCapability::CodeGeneration)
        .with_library("open_clip")
        .with_library("transformers")
        .with_library("CLIP")
        .with_instructions("Vision-language foundation model expert specializing in CLIP-like models")
        .with_priority(10)
    }

    /// MAE and self-supervised vision expert
    pub fn mae_expert() -> FoundationModelAgent {
        FoundationModelAgent::new(
            "mae-expert",
            AgentRole::Specialist {
                domain: "self-supervised-vision".to_string(),
            },
        )
        .with_architectures(vec![
            FoundationModelArchitecture::MAE,
            FoundationModelArchitecture::BEiT,
            FoundationModelArchitecture::DINOv2,
        ])
        .with_fm_capabilities(vec![
            FoundationModelCapability::VisionTransformerDesign,
            FoundationModelCapability::MaskedPretraining,
            FoundationModelCapability::SelfSupervisedPretraining,
            FoundationModelCapability::LinearProbing,
        ])
        .with_pretraining_strategy(PretrainingStrategy::MaskedImageModeling)
        .with_model_scale(ModelScale::Base)
        .with_model_scale(ModelScale::Large)
        .with_model_scale(ModelScale::XLarge)
        .with_dataset_scale(DatasetScale::Large)
        .with_dataset_scale(DatasetScale::WebScale)
        .with_capability(AgentCapability::CodeGeneration)
        .with_library("timm")
        .with_library("transformers")
        .with_library("mae")
        .with_instructions("Self-supervised vision foundation model expert for MAE and masked pretraining")
        .with_priority(10)
    }

    /// Scaling law researcher
    pub fn scaling_law_expert() -> FoundationModelAgent {
        FoundationModelAgent::new(
            "scaling-law-expert",
            AgentRole::Specialist {
                domain: "scaling-laws".to_string(),
            },
        )
        .with_fm_capabilities(vec![
            FoundationModelCapability::ScalingLawAnalysis,
            FoundationModelCapability::ComputeOptimalScaling,
            FoundationModelCapability::DataScaling,
            FoundationModelCapability::ModelScaling,
            FoundationModelCapability::EmergentAbilityDetection,
        ])
        .with_model_scale(ModelScale::Small)
        .with_model_scale(ModelScale::Base)
        .with_model_scale(ModelScale::Large)
        .with_model_scale(ModelScale::XLarge)
        .with_model_scale(ModelScale::XXLarge)
        .with_dataset_scale(DatasetScale::Large)
        .with_dataset_scale(DatasetScale::WebScale)
        .with_dataset_scale(DatasetScale::InternetScale)
        .with_capability(AgentCapability::Research)
        .with_capability(AgentCapability::CodeGeneration)
        .with_library("numpy")
        .with_library("scipy")
        .with_library("matplotlib")
        .with_instructions("Scaling law expert for compute-optimal model training and emergent abilities")
        .with_priority(9)
    }

    /// Large-scale pretraining expert
    pub fn pretraining_expert() -> FoundationModelAgent {
        FoundationModelAgent::new(
            "pretraining-expert",
            AgentRole::Specialist {
                domain: "large-scale-pretraining".to_string(),
            },
        )
        .with_fm_capabilities(vec![
            FoundationModelCapability::LargeScalePretraining,
            FoundationModelCapability::ModelParallelism,
            FoundationModelCapability::DataParallelism,
            FoundationModelCapability::PipelineParallelism,
            FoundationModelCapability::ZeROOptimization,
            FoundationModelCapability::MixedPrecisionTraining,
        ])
        .with_model_scale(ModelScale::Large)
        .with_model_scale(ModelScale::XLarge)
        .with_model_scale(ModelScale::XXLarge)
        .with_dataset_scale(DatasetScale::WebScale)
        .with_dataset_scale(DatasetScale::InternetScale)
        .with_capability(AgentCapability::CodeGeneration)
        .with_library("DeepSpeed")
        .with_library("Megatron-LM")
        .with_library("fairscale")
        .with_library("torch.distributed")
        .with_instructions("Large-scale pretraining expert for foundation models with billions of parameters")
        .with_priority(10)
    }

    /// Web-scale data curation expert
    pub fn data_curation_expert() -> FoundationModelAgent {
        FoundationModelAgent::new(
            "data-curation-expert",
            AgentRole::Specialist {
                domain: "web-scale-data".to_string(),
            },
        )
        .with_fm_capabilities(vec![
            FoundationModelCapability::WebScaleDataCuration,
            FoundationModelCapability::DataFiltering,
            FoundationModelCapability::DataDeduplication,
            FoundationModelCapability::MultimodalDataAlignment,
        ])
        .with_dataset_scale(DatasetScale::Large)
        .with_dataset_scale(DatasetScale::WebScale)
        .with_dataset_scale(DatasetScale::InternetScale)
        .with_capability(AgentCapability::CodeGeneration)
        .with_library("img2dataset")
        .with_library("clip-retrieval")
        .with_library("datatrove")
        .with_instructions("Web-scale data curation expert for foundation model datasets")
        .with_priority(9)
    }

    /// Transfer learning and fine-tuning expert
    pub fn transfer_learning_expert() -> FoundationModelAgent {
        FoundationModelAgent::new(
            "transfer-expert",
            AgentRole::Specialist {
                domain: "transfer-learning".to_string(),
            },
        )
        .with_fm_capabilities(vec![
            FoundationModelCapability::ZeroShotTransfer,
            FoundationModelCapability::FewShotTransfer,
            FoundationModelCapability::FullFineTuning,
            FoundationModelCapability::ParameterEfficientFineTuning,
            FoundationModelCapability::PromptTuning,
            FoundationModelCapability::LinearProbing,
        ])
        .with_capability(AgentCapability::CodeGeneration)
        .with_library("peft")
        .with_library("adapters")
        .with_library("transformers")
        .with_instructions("Transfer learning expert for foundation model adaptation")
        .with_priority(9)
    }

    /// SAM (Segment Anything) expert
    pub fn sam_expert() -> FoundationModelAgent {
        FoundationModelAgent::new(
            "sam-expert",
            AgentRole::Specialist {
                domain: "segmentation-foundation".to_string(),
            },
        )
        .with_architecture(FoundationModelArchitecture::SAM)
        .with_fm_capabilities(vec![
            FoundationModelCapability::FoundationModelDesign,
            FoundationModelCapability::ZeroShotTransfer,
            FoundationModelCapability::PromptTuning,
        ])
        .with_model_scale(ModelScale::Large)
        .with_dataset_scale(DatasetScale::WebScale)
        .with_capability(AgentCapability::CodeGeneration)
        .with_library("segment-anything")
        .with_library("SAM")
        .with_instructions("Segment Anything Model expert for promptable segmentation")
        .with_priority(9)
    }

    /// Multimodal foundation model expert
    pub fn multimodal_expert() -> FoundationModelAgent {
        FoundationModelAgent::new(
            "multimodal-fm-expert",
            AgentRole::Specialist {
                domain: "multimodal-foundation".to_string(),
            },
        )
        .with_architectures(vec![
            FoundationModelArchitecture::Flamingo,
            FoundationModelArchitecture::GPT4Vision,
            FoundationModelArchitecture::LLaVA,
            FoundationModelArchitecture::BLIP,
        ])
        .with_fm_capabilities(vec![
            FoundationModelCapability::MultimodalArchitecture,
            FoundationModelCapability::MultimodalPretraining,
            FoundationModelCapability::ZeroShotTransfer,
            FoundationModelCapability::FewShotTransfer,
        ])
        .with_pretraining_strategy(PretrainingStrategy::VisionLanguageAlignment)
        .with_pretraining_strategy(PretrainingStrategy::ImageTextMatching)
        .with_model_scale(ModelScale::Large)
        .with_model_scale(ModelScale::XLarge)
        .with_dataset_scale(DatasetScale::WebScale)
        .with_capability(AgentCapability::CodeGeneration)
        .with_library("transformers")
        .with_library("LLaVA")
        .with_library("BLIP")
        .with_instructions("Multimodal foundation model expert for vision-language tasks")
        .with_priority(10)
    }

    /// Evaluation and benchmarking expert
    pub fn evaluation_expert() -> FoundationModelAgent {
        FoundationModelAgent::new(
            "fm-evaluation-expert",
            AgentRole::Specialist {
                domain: "foundation-model-evaluation".to_string(),
            },
        )
        .with_fm_capabilities(vec![
            FoundationModelCapability::ZeroShotEvaluation,
            FoundationModelCapability::FewShotEvaluation,
            FoundationModelCapability::TransferLearningEvaluation,
            FoundationModelCapability::RobustnessEvaluation,
            FoundationModelCapability::DistributionShiftEvaluation,
        ])
        .with_capability(AgentCapability::Testing)
        .with_capability(AgentCapability::CodeGeneration)
        .with_library("ELEVATER")
        .with_library("VTAB")
        .with_library("torchmetrics")
        .with_instructions("Foundation model evaluation expert for comprehensive benchmarking")
        .with_priority(8)
    }

    /// Vision transformer architecture expert
    pub fn vit_expert() -> FoundationModelAgent {
        FoundationModelAgent::new(
            "vit-architect",
            AgentRole::Specialist {
                domain: "vision-transformer".to_string(),
            },
        )
        .with_architectures(vec![
            FoundationModelArchitecture::DEIT,
            FoundationModelArchitecture::SwinTransformer,
            FoundationModelArchitecture::DINOv2,
        ])
        .with_fm_capabilities(vec![
            FoundationModelCapability::VisionTransformerDesign,
            FoundationModelCapability::AttentionMechanisms,
            FoundationModelCapability::EfficientAttention,
        ])
        .with_model_scale(ModelScale::Tiny)
        .with_model_scale(ModelScale::Small)
        .with_model_scale(ModelScale::Base)
        .with_model_scale(ModelScale::Large)
        .with_capability(AgentCapability::Architecture)
        .with_capability(AgentCapability::CodeGeneration)
        .with_library("timm")
        .with_library("transformers")
        .with_instructions("Vision Transformer architecture expert for efficient ViT designs")
        .with_priority(9)
    }

    /// Depth Anything V3 expert
    pub fn depth_anything_expert() -> FoundationModelAgent {
        FoundationModelAgent::new(
            "depth-anything-expert",
            AgentRole::Specialist {
                domain: "depth-estimation".to_string(),
            },
        )
        .with_architectures(vec![
            FoundationModelArchitecture::DepthAnythingV3,
            FoundationModelArchitecture::DepthAnythingV2,
            FoundationModelArchitecture::DepthAnything,
        ])
        .with_fm_capabilities(vec![
            FoundationModelCapability::MonocularDepthEstimation,
            FoundationModelCapability::MetricDepthEstimation,
            FoundationModelCapability::RelativeDepthEstimation,
            FoundationModelCapability::ZeroShotDepthEstimation,
            FoundationModelCapability::ZeroShotTransfer,
        ])
        .with_pretraining_strategy(PretrainingStrategy::DenoisingAutoencoder)
        .with_model_scale(ModelScale::Base)
        .with_model_scale(ModelScale::Large)
        .with_dataset_scale(DatasetScale::WebScale)
        .with_capability(AgentCapability::CodeGeneration)
        .with_library("depth-anything")
        .with_library("transformers")
        .with_library("torch")
        .with_instructions("Depth Anything foundation model expert for zero-shot monocular depth estimation")
        .with_priority(9)
    }

    /// SAM 2 expert (video + image segmentation)
    pub fn sam2_expert() -> FoundationModelAgent {
        FoundationModelAgent::new(
            "sam2-expert",
            AgentRole::Specialist {
                domain: "video-segmentation".to_string(),
            },
        )
        .with_architecture(FoundationModelArchitecture::SAM2)
        .with_fm_capabilities(vec![
            FoundationModelCapability::VideoSegmentation,
            FoundationModelCapability::VideoObjectTracking,
            FoundationModelCapability::TemporalConsistency,
            FoundationModelCapability::ZeroShotTransfer,
            FoundationModelCapability::PromptTuning,
        ])
        .with_model_scale(ModelScale::Large)
        .with_model_scale(ModelScale::XLarge)
        .with_dataset_scale(DatasetScale::WebScale)
        .with_capability(AgentCapability::CodeGeneration)
        .with_library("SAM2")
        .with_library("segment-anything-2")
        .with_library("torch")
        .with_instructions("SAM 2 expert for promptable video and image segmentation with temporal consistency")
        .with_priority(10)
    }

    /// Grounding DINO expert
    pub fn grounding_dino_expert() -> FoundationModelAgent {
        FoundationModelAgent::new(
            "grounding-dino-expert",
            AgentRole::Specialist {
                domain: "open-vocabulary-detection".to_string(),
            },
        )
        .with_architecture(FoundationModelArchitecture::GroundingDINO)
        .with_fm_capabilities(vec![
            FoundationModelCapability::OpenVocabularyDetection,
            FoundationModelCapability::GroundingCapability,
            FoundationModelCapability::TextGroundedUnderstanding,
            FoundationModelCapability::ZeroShotTransfer,
        ])
        .with_pretraining_strategy(PretrainingStrategy::VisionLanguageAlignment)
        .with_model_scale(ModelScale::Base)
        .with_model_scale(ModelScale::Large)
        .with_dataset_scale(DatasetScale::Large)
        .with_capability(AgentCapability::CodeGeneration)
        .with_library("groundingdino")
        .with_library("transformers")
        .with_library("supervision")
        .with_instructions("Grounding DINO expert for open-vocabulary object detection with text grounding")
        .with_priority(9)
    }

    /// DINOv2 enhanced expert
    pub fn dinov2_expert() -> FoundationModelAgent {
        FoundationModelAgent::new(
            "dinov2-expert",
            AgentRole::Specialist {
                domain: "self-distillation".to_string(),
            },
        )
        .with_architecture(FoundationModelArchitecture::DINOv2)
        .with_fm_capabilities(vec![
            FoundationModelCapability::SelfSupervisedPretraining,
            FoundationModelCapability::VisionTransformerDesign,
            FoundationModelCapability::LinearProbing,
            FoundationModelCapability::ZeroShotTransfer,
            FoundationModelCapability::FeatureVisualization,
        ])
        .with_pretraining_strategy(PretrainingStrategy::ContrastiveLearning)
        .with_model_scale(ModelScale::Small)
        .with_model_scale(ModelScale::Base)
        .with_model_scale(ModelScale::Large)
        .with_model_scale(ModelScale::Gigantic)
        .with_dataset_scale(DatasetScale::WebScale)
        .with_capability(AgentCapability::CodeGeneration)
        .with_library("dinov2")
        .with_library("torch")
        .with_library("timm")
        .with_instructions("DINOv2 expert for self-distillation with no labels and strong dense prediction features")
        .with_priority(10)
    }

    /// Get all foundation model experts
    pub fn all_fm_experts() -> Vec<FoundationModelAgent> {
        vec![
            clip_expert(),
            mae_expert(),
            scaling_law_expert(),
            pretraining_expert(),
            data_curation_expert(),
            transfer_learning_expert(),
            sam_expert(),
            sam2_expert(),
            multimodal_expert(),
            evaluation_expert(),
            vit_expert(),
            depth_anything_expert(),
            grounding_dino_expert(),
            dinov2_expert(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_clip_expert() {
        let agent = presets::clip_expert();
        assert_eq!(agent.spec.id.0, "clip-expert");
        assert!(agent.architectures.contains(&FoundationModelArchitecture::CLIP));
        assert!(agent.fm_capabilities.contains(&FoundationModelCapability::ZeroShotTransfer));
    }

    #[test]
    fn test_create_scaling_law_expert() {
        let agent = presets::scaling_law_expert();
        assert!(agent.fm_capabilities.contains(&FoundationModelCapability::ScalingLawAnalysis));
        assert!(agent.model_scales.contains(&ModelScale::Large));
    }

    #[test]
    fn test_all_experts_count() {
        let experts = presets::all_fm_experts();
        assert_eq!(experts.len(), 10);
    }

    #[test]
    fn test_custom_agent() {
        let agent = FoundationModelAgent::new(
            "custom-fm",
            AgentRole::Specialist {
                domain: "custom".to_string(),
            },
        )
        .with_architecture(FoundationModelArchitecture::CLIP)
        .with_fm_capability(FoundationModelCapability::ZeroShotTransfer)
        .with_model_scale(ModelScale::Large)
        .with_priority(10);

        assert_eq!(agent.spec.id.0, "custom-fm");
        assert!(agent.architectures.contains(&FoundationModelArchitecture::CLIP));
        assert_eq!(agent.spec.priority, 10);
    }
}
