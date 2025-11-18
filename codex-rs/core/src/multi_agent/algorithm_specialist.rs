//! Algorithm engineer specialized multi-agent system
//!
//! This module provides specialized agents for algorithm engineers,
//! including various perception experts and domain specialists.

use super::types::{AgentCapability, AgentRole, AgentSpec};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Specialized capabilities for algorithm engineering
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlgorithmCapability {
    /// Machine learning model development
    MachineLearning,
    /// Deep learning and neural networks
    DeepLearning,
    /// Computer vision algorithms
    ComputerVision,
    /// Natural language processing
    NaturalLanguageProcessing,
    /// Signal processing
    SignalProcessing,
    /// Optimization algorithms
    Optimization,
    /// Statistical analysis
    StatisticalAnalysis,
    /// Data preprocessing and feature engineering
    DataEngineering,
    /// Model evaluation and validation
    ModelEvaluation,
    /// Hyperparameter tuning
    HyperparameterTuning,
    /// Algorithm complexity analysis
    ComplexityAnalysis,
    /// Reinforcement learning
    ReinforcementLearning,
    /// Time series analysis
    TimeSeriesAnalysis,
    /// Graph algorithms
    GraphAlgorithms,
    /// Numerical methods
    NumericalMethods,
    /// Probabilistic modeling
    ProbabilisticModeling,
    /// Pattern recognition
    PatternRecognition,
    /// Image segmentation
    ImageSegmentation,
    /// Object detection
    ObjectDetection,
    /// Speech recognition
    SpeechRecognition,
    /// Recommendation systems
    RecommendationSystems,
    /// Anomaly detection
    AnomalyDetection,
    /// Clustering algorithms
    Clustering,
    /// Dimensionality reduction
    DimensionalityReduction,
    /// Transfer learning
    TransferLearning,
    /// Model compression
    ModelCompression,
    /// Edge computing optimization
    EdgeComputing,
    /// Distributed training
    DistributedTraining,
    /// AutoML and neural architecture search
    AutoML,
    /// Explainable AI
    ExplainableAI,
    /// Custom algorithm capability
    Custom(String),
}

impl std::fmt::Display for AlgorithmCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlgorithmCapability::MachineLearning => write!(f, "machine-learning"),
            AlgorithmCapability::DeepLearning => write!(f, "deep-learning"),
            AlgorithmCapability::ComputerVision => write!(f, "computer-vision"),
            AlgorithmCapability::NaturalLanguageProcessing => write!(f, "nlp"),
            AlgorithmCapability::SignalProcessing => write!(f, "signal-processing"),
            AlgorithmCapability::Optimization => write!(f, "optimization"),
            AlgorithmCapability::StatisticalAnalysis => write!(f, "statistical-analysis"),
            AlgorithmCapability::DataEngineering => write!(f, "data-engineering"),
            AlgorithmCapability::ModelEvaluation => write!(f, "model-evaluation"),
            AlgorithmCapability::HyperparameterTuning => write!(f, "hyperparameter-tuning"),
            AlgorithmCapability::ComplexityAnalysis => write!(f, "complexity-analysis"),
            AlgorithmCapability::ReinforcementLearning => write!(f, "reinforcement-learning"),
            AlgorithmCapability::TimeSeriesAnalysis => write!(f, "time-series"),
            AlgorithmCapability::GraphAlgorithms => write!(f, "graph-algorithms"),
            AlgorithmCapability::NumericalMethods => write!(f, "numerical-methods"),
            AlgorithmCapability::ProbabilisticModeling => write!(f, "probabilistic-modeling"),
            AlgorithmCapability::PatternRecognition => write!(f, "pattern-recognition"),
            AlgorithmCapability::ImageSegmentation => write!(f, "image-segmentation"),
            AlgorithmCapability::ObjectDetection => write!(f, "object-detection"),
            AlgorithmCapability::SpeechRecognition => write!(f, "speech-recognition"),
            AlgorithmCapability::RecommendationSystems => write!(f, "recommendation-systems"),
            AlgorithmCapability::AnomalyDetection => write!(f, "anomaly-detection"),
            AlgorithmCapability::Clustering => write!(f, "clustering"),
            AlgorithmCapability::DimensionalityReduction => write!(f, "dimensionality-reduction"),
            AlgorithmCapability::TransferLearning => write!(f, "transfer-learning"),
            AlgorithmCapability::ModelCompression => write!(f, "model-compression"),
            AlgorithmCapability::EdgeComputing => write!(f, "edge-computing"),
            AlgorithmCapability::DistributedTraining => write!(f, "distributed-training"),
            AlgorithmCapability::AutoML => write!(f, "automl"),
            AlgorithmCapability::ExplainableAI => write!(f, "explainable-ai"),
            AlgorithmCapability::Custom(name) => write!(f, "custom:{name}"),
        }
    }
}

/// Perception expert specialization
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PerceptionDomain {
    /// Visual perception (computer vision)
    Visual,
    /// Audio perception (speech, sound)
    Audio,
    /// Text perception (NLP)
    Text,
    /// Sensor fusion
    SensorFusion,
    /// 3D perception
    ThreeDimensional,
    /// Multimodal perception
    Multimodal,
}

/// Algorithm engineer specialized agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmAgent {
    /// Base agent specification
    pub spec: AgentSpec,
    /// Algorithm-specific capabilities
    pub algorithm_capabilities: HashSet<AlgorithmCapability>,
    /// Perception domain (if applicable)
    pub perception_domain: Option<PerceptionDomain>,
    /// Preferred frameworks
    pub frameworks: Vec<String>,
    /// Model expertise
    pub model_expertise: Vec<String>,
}

impl AlgorithmAgent {
    /// Create a new algorithm agent
    pub fn new(id: impl Into<String>, role: AgentRole) -> Self {
        let id_string = id.into();
        Self {
            spec: AgentSpec::new(id_string, role),
            algorithm_capabilities: HashSet::new(),
            perception_domain: None,
            frameworks: Vec::new(),
            model_expertise: Vec::new(),
        }
    }

    /// Add algorithm capability
    pub fn with_algorithm_capability(mut self, cap: AlgorithmCapability) -> Self {
        self.algorithm_capabilities.insert(cap);
        self
    }

    /// Add multiple algorithm capabilities
    pub fn with_algorithm_capabilities(mut self, caps: Vec<AlgorithmCapability>) -> Self {
        self.algorithm_capabilities.extend(caps);
        self
    }

    /// Set perception domain
    pub fn with_perception_domain(mut self, domain: PerceptionDomain) -> Self {
        self.perception_domain = Some(domain);
        self
    }

    /// Add framework expertise
    pub fn with_framework(mut self, framework: impl Into<String>) -> Self {
        self.frameworks.push(framework.into());
        self
    }

    /// Add model expertise
    pub fn with_model_expertise(mut self, model: impl Into<String>) -> Self {
        self.model_expertise.push(model.into());
        self
    }

    /// Forward method calls to underlying spec
    pub fn with_capability(mut self, cap: AgentCapability) -> Self {
        self.spec = self.spec.with_capability(cap);
        self
    }

    /// Set custom instructions
    pub fn with_instructions(mut self, instructions: impl Into<String>) -> Self {
        self.spec = self.spec.with_instructions(instructions);
        self
    }

    /// Set max concurrent tasks
    pub fn with_max_concurrent_tasks(mut self, max: usize) -> Self {
        self.spec = self.spec.with_max_concurrent_tasks(max);
        self
    }

    /// Set priority
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.spec = self.spec.with_priority(priority);
        self
    }

    /// Check if has algorithm capability
    pub fn has_algorithm_capability(&self, cap: &AlgorithmCapability) -> bool {
        self.algorithm_capabilities.contains(cap)
    }

    /// Get the underlying agent spec
    pub fn into_spec(self) -> AgentSpec {
        self.spec
    }
}

/// Presets for common algorithm engineer agents
pub mod presets {
    use super::*;

    /// Computer vision expert
    pub fn computer_vision_expert() -> AlgorithmAgent {
        AlgorithmAgent::new(
            "cv-expert",
            AgentRole::Specialist {
                domain: "computer-vision".to_string(),
            },
        )
        .with_perception_domain(PerceptionDomain::Visual)
        .with_algorithm_capabilities(vec![
            AlgorithmCapability::ComputerVision,
            AlgorithmCapability::DeepLearning,
            AlgorithmCapability::ImageSegmentation,
            AlgorithmCapability::ObjectDetection,
            AlgorithmCapability::PatternRecognition,
        ])
        .with_capability(AgentCapability::CodeGeneration)
        .with_capability(AgentCapability::Research)
        .with_framework("PyTorch")
        .with_framework("TensorFlow")
        .with_framework("OpenCV")
        .with_model_expertise("CNN")
        .with_model_expertise("YOLO")
        .with_model_expertise("ResNet")
        .with_model_expertise("Vision Transformer")
        .with_instructions(
            "You are a computer vision expert specializing in image processing, \
             object detection, and visual perception. You excel at implementing \
             state-of-the-art CV algorithms and optimizing model performance.",
        )
        .with_max_concurrent_tasks(2)
        .with_priority(10)
    }

    /// NLP expert
    pub fn nlp_expert() -> AlgorithmAgent {
        AlgorithmAgent::new(
            "nlp-expert",
            AgentRole::Specialist {
                domain: "nlp".to_string(),
            },
        )
        .with_perception_domain(PerceptionDomain::Text)
        .with_algorithm_capabilities(vec![
            AlgorithmCapability::NaturalLanguageProcessing,
            AlgorithmCapability::DeepLearning,
            AlgorithmCapability::PatternRecognition,
        ])
        .with_capability(AgentCapability::CodeGeneration)
        .with_capability(AgentCapability::Research)
        .with_framework("Transformers")
        .with_framework("spaCy")
        .with_framework("NLTK")
        .with_model_expertise("BERT")
        .with_model_expertise("GPT")
        .with_model_expertise("T5")
        .with_instructions(
            "You are an NLP expert specializing in natural language understanding, \
             text generation, and language models. You're proficient in transformer \
             architectures and modern NLP techniques.",
        )
        .with_max_concurrent_tasks(2)
        .with_priority(10)
    }

    /// Machine learning expert
    pub fn ml_expert() -> AlgorithmAgent {
        AlgorithmAgent::new(
            "ml-expert",
            AgentRole::Specialist {
                domain: "machine-learning".to_string(),
            },
        )
        .with_algorithm_capabilities(vec![
            AlgorithmCapability::MachineLearning,
            AlgorithmCapability::StatisticalAnalysis,
            AlgorithmCapability::ModelEvaluation,
            AlgorithmCapability::HyperparameterTuning,
        ])
        .with_capability(AgentCapability::CodeGeneration)
        .with_capability(AgentCapability::Testing)
        .with_framework("scikit-learn")
        .with_framework("XGBoost")
        .with_framework("LightGBM")
        .with_instructions(
            "You are a machine learning expert with deep knowledge of classical ML \
             algorithms, statistical methods, and model evaluation techniques.",
        )
        .with_max_concurrent_tasks(3)
        .with_priority(9)
    }

    /// Deep learning expert
    pub fn deep_learning_expert() -> AlgorithmAgent {
        AlgorithmAgent::new(
            "dl-expert",
            AgentRole::Specialist {
                domain: "deep-learning".to_string(),
            },
        )
        .with_algorithm_capabilities(vec![
            AlgorithmCapability::DeepLearning,
            AlgorithmCapability::Optimization,
            AlgorithmCapability::DistributedTraining,
            AlgorithmCapability::ModelCompression,
        ])
        .with_capability(AgentCapability::CodeGeneration)
        .with_capability(AgentCapability::Architecture)
        .with_framework("PyTorch")
        .with_framework("TensorFlow")
        .with_framework("JAX")
        .with_model_expertise("Neural Networks")
        .with_model_expertise("Attention Mechanisms")
        .with_instructions(
            "You are a deep learning expert specializing in neural network \
             architectures, training optimization, and model deployment.",
        )
        .with_max_concurrent_tasks(2)
        .with_priority(10)
    }

    /// Reinforcement learning expert
    pub fn rl_expert() -> AlgorithmAgent {
        AlgorithmAgent::new(
            "rl-expert",
            AgentRole::Specialist {
                domain: "reinforcement-learning".to_string(),
            },
        )
        .with_algorithm_capabilities(vec![
            AlgorithmCapability::ReinforcementLearning,
            AlgorithmCapability::Optimization,
            AlgorithmCapability::ProbabilisticModeling,
        ])
        .with_capability(AgentCapability::CodeGeneration)
        .with_capability(AgentCapability::Research)
        .with_framework("OpenAI Gym")
        .with_framework("Stable Baselines")
        .with_framework("Ray RLlib")
        .with_model_expertise("DQN")
        .with_model_expertise("PPO")
        .with_model_expertise("A3C")
        .with_instructions(
            "You are a reinforcement learning expert with expertise in policy \
             optimization, value-based methods, and multi-agent RL.",
        )
        .with_max_concurrent_tasks(2)
        .with_priority(9)
    }

    /// Data engineering expert
    pub fn data_engineer() -> AlgorithmAgent {
        AlgorithmAgent::new(
            "data-engineer",
            AgentRole::Specialist {
                domain: "data-engineering".to_string(),
            },
        )
        .with_algorithm_capabilities(vec![
            AlgorithmCapability::DataEngineering,
            AlgorithmCapability::StatisticalAnalysis,
        ])
        .with_capability(AgentCapability::CodeGeneration)
        .with_capability(AgentCapability::Debugging)
        .with_framework("pandas")
        .with_framework("NumPy")
        .with_framework("Dask")
        .with_instructions(
            "You are a data engineering expert specializing in data preprocessing, \
             feature engineering, and pipeline development.",
        )
        .with_max_concurrent_tasks(3)
        .with_priority(8)
    }

    /// Optimization expert
    pub fn optimization_expert() -> AlgorithmAgent {
        AlgorithmAgent::new(
            "optimization-expert",
            AgentRole::Specialist {
                domain: "optimization".to_string(),
            },
        )
        .with_algorithm_capabilities(vec![
            AlgorithmCapability::Optimization,
            AlgorithmCapability::NumericalMethods,
            AlgorithmCapability::ComplexityAnalysis,
        ])
        .with_capability(AgentCapability::CodeGeneration)
        .with_capability(AgentCapability::Architecture)
        .with_framework("SciPy")
        .with_framework("CVXPY")
        .with_instructions(
            "You are an optimization expert with deep knowledge of mathematical \
             optimization, algorithm efficiency, and performance tuning.",
        )
        .with_max_concurrent_tasks(2)
        .with_priority(8)
    }

    /// Time series expert
    pub fn time_series_expert() -> AlgorithmAgent {
        AlgorithmAgent::new(
            "ts-expert",
            AgentRole::Specialist {
                domain: "time-series".to_string(),
            },
        )
        .with_algorithm_capabilities(vec![
            AlgorithmCapability::TimeSeriesAnalysis,
            AlgorithmCapability::StatisticalAnalysis,
            AlgorithmCapability::DeepLearning,
        ])
        .with_capability(AgentCapability::CodeGeneration)
        .with_capability(AgentCapability::Research)
        .with_framework("statsmodels")
        .with_framework("Prophet")
        .with_framework("LSTM")
        .with_instructions(
            "You are a time series expert specializing in forecasting, anomaly \
             detection, and temporal pattern recognition.",
        )
        .with_max_concurrent_tasks(2)
        .with_priority(8)
    }

    /// Model evaluation expert
    pub fn model_evaluator() -> AlgorithmAgent {
        AlgorithmAgent::new(
            "evaluator",
            AgentRole::Specialist {
                domain: "model-evaluation".to_string(),
            },
        )
        .with_algorithm_capabilities(vec![
            AlgorithmCapability::ModelEvaluation,
            AlgorithmCapability::StatisticalAnalysis,
            AlgorithmCapability::ExplainableAI,
        ])
        .with_capability(AgentCapability::CodeReview)
        .with_capability(AgentCapability::Testing)
        .with_framework("scikit-learn")
        .with_framework("SHAP")
        .with_framework("LIME")
        .with_instructions(
            "You are a model evaluation expert focusing on metrics, validation, \
             and interpretability of ML models.",
        )
        .with_max_concurrent_tasks(3)
        .with_priority(9)
    }

    /// Get all perception expert presets
    pub fn all_perception_experts() -> Vec<AlgorithmAgent> {
        vec![
            computer_vision_expert(),
            nlp_expert(),
            ml_expert(),
            deep_learning_expert(),
            rl_expert(),
            data_engineer(),
            optimization_expert(),
            time_series_expert(),
            model_evaluator(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_algorithm_agent() {
        let agent = AlgorithmAgent::new("test", AgentRole::GeneralPurpose)
            .with_algorithm_capability(AlgorithmCapability::MachineLearning);

        assert!(agent.has_algorithm_capability(&AlgorithmCapability::MachineLearning));
    }

    #[test]
    fn test_cv_expert_preset() {
        let agent = presets::computer_vision_expert();
        assert!(agent.has_algorithm_capability(&AlgorithmCapability::ComputerVision));
        assert_eq!(agent.perception_domain, Some(PerceptionDomain::Visual));
    }

    #[test]
    fn test_all_perception_experts() {
        let experts = presets::all_perception_experts();
        assert_eq!(experts.len(), 9);
    }
}
