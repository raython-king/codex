//! Algorithm engineer multi-agent collaboration example
//!
//! This example demonstrates how to use specialized perception experts
//! for algorithm engineering tasks using the multi-agent framework.

use codex_core::multi_agent::{
    algorithm_presets, AlgorithmAgent, AlgorithmCapability, AlgorithmCoordinator,
    AlgorithmCoordinatorConfig, PerceptionDomain, TaskDistributionStrategy,
};

/// Example: Setting up an algorithm engineering team
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Algorithm Engineer Multi-Agent System");
    println!("======================================\n");

    // Example 1: Using all perception expert presets
    demonstrate_perception_experts();

    // Example 2: Creating custom algorithm agents
    demonstrate_custom_agents();

    // Example 3: Setting up a complete algorithm team
    demonstrate_algorithm_team().await?;

    // Example 4: Task routing examples
    demonstrate_task_routing();

    Ok(())
}

/// Demonstrate using all perception expert presets
fn demonstrate_perception_experts() {
    println!("=== Perception Expert Presets ===\n");

    let experts = algorithm_presets::all_perception_experts();

    println!("Available Perception Experts: {}\n", experts.len());

    for expert in &experts {
        println!("Expert: {}", expert.spec.id.0);
        println!("  Domain: {:?}", expert.perception_domain);
        println!("  Algorithm Capabilities:");
        for cap in &expert.algorithm_capabilities {
            println!("    - {}", cap);
        }
        println!("  Frameworks: {:?}", expert.frameworks);
        if !expert.model_expertise.is_empty() {
            println!("  Model Expertise: {:?}", expert.model_expertise);
        }
        println!();
    }
}

/// Demonstrate creating custom algorithm agents
fn demonstrate_custom_agents() {
    println!("\n=== Custom Algorithm Agents ===\n");

    // Create a multimodal perception expert
    let multimodal_expert = AlgorithmAgent::new(
        "multimodal-expert",
        codex_core::multi_agent::AgentRole::Specialist {
            domain: "multimodal-perception".to_string(),
        },
    )
    .with_perception_domain(PerceptionDomain::Multimodal)
    .with_algorithm_capabilities(vec![
        AlgorithmCapability::ComputerVision,
        AlgorithmCapability::NaturalLanguageProcessing,
        AlgorithmCapability::DeepLearning,
        AlgorithmCapability::SensorFusion,
    ])
    .with_capability(codex_core::multi_agent::AgentCapability::CodeGeneration)
    .with_capability(codex_core::multi_agent::AgentCapability::Research)
    .with_framework("PyTorch")
    .with_framework("Transformers")
    .with_model_expertise("CLIP")
    .with_model_expertise("DALL-E")
    .with_instructions(
        "You are a multimodal perception expert specializing in combining \
         visual and textual information. You excel at vision-language models.",
    )
    .with_max_concurrent_tasks(2)
    .with_priority(10);

    println!("Custom Multimodal Expert:");
    println!("  ID: {}", multimodal_expert.spec.id.0);
    println!("  Perception Domain: {:?}", multimodal_expert.perception_domain);
    println!("  Algorithm Capabilities: {}", multimodal_expert.algorithm_capabilities.len());
    println!("  Frameworks: {:?}", multimodal_expert.frameworks);
    println!();

    // Create an AutoML specialist
    let automl_expert = AlgorithmAgent::new(
        "automl-expert",
        codex_core::multi_agent::AgentRole::Specialist {
            domain: "automl".to_string(),
        },
    )
    .with_algorithm_capabilities(vec![
        AlgorithmCapability::AutoML,
        AlgorithmCapability::HyperparameterTuning,
        AlgorithmCapability::Optimization,
        AlgorithmCapability::ModelEvaluation,
    ])
    .with_capability(codex_core::multi_agent::AgentCapability::CodeGeneration)
    .with_framework("Optuna")
    .with_framework("Ray Tune")
    .with_framework("AutoKeras")
    .with_instructions(
        "You are an AutoML expert specializing in automated machine learning, \
         hyperparameter optimization, and neural architecture search.",
    );

    println!("Custom AutoML Expert:");
    println!("  ID: {}", automl_expert.spec.id.0);
    println!("  Algorithm Capabilities: {:?}", automl_expert.algorithm_capabilities);
    println!();
}

/// Demonstrate setting up a complete algorithm team
async fn demonstrate_algorithm_team() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Complete Algorithm Engineering Team ===\n");

    // Build a comprehensive team
    let team = vec![
        algorithm_presets::computer_vision_expert(),
        algorithm_presets::nlp_expert(),
        algorithm_presets::ml_expert(),
        algorithm_presets::deep_learning_expert(),
        algorithm_presets::rl_expert(),
        algorithm_presets::data_engineer(),
        algorithm_presets::optimization_expert(),
        algorithm_presets::time_series_expert(),
        algorithm_presets::model_evaluator(),
    ];

    // Create coordinator configuration
    let config = AlgorithmCoordinatorConfig {
        perception_experts: team,
        distribution_strategy: TaskDistributionStrategy::BestMatch,
        auto_infer_capabilities: true,
        verbose: true,
        ..Default::default()
    };

    println!("Team Configuration:");
    println!("  Total Experts: {}", config.perception_experts.len());
    println!("  Distribution Strategy: {:?}", config.distribution_strategy);
    println!("  Auto Capability Inference: {}", config.auto_infer_capabilities);
    println!("  Max Agents: {}", config.pool_config.max_agents);
    println!("  Max Concurrent Tasks: {}", config.pool_config.max_concurrent_tasks);
    println!();

    // Create the coordinator
    let _coordinator = AlgorithmCoordinator::new(config);

    println!("Algorithm Coordinator initialized successfully!");
    println!();

    Ok(())
}

/// Demonstrate task routing examples
fn demonstrate_task_routing() {
    println!("\n=== Task Routing Examples ===\n");

    let examples = vec![
        (
            "Computer Vision Task",
            "Implement a YOLO-based object detection system for real-time video processing",
            vec!["cv-expert"],
            vec![AlgorithmCapability::ComputerVision, AlgorithmCapability::DeepLearning]
        ),
        (
            "NLP Task",
            "Build a BERT-based sentiment analysis model for customer reviews",
            vec!["nlp-expert"],
            vec![AlgorithmCapability::NaturalLanguageProcessing, AlgorithmCapability::DeepLearning]
        ),
        (
            "Time Series Task",
            "Create an LSTM model for stock price forecasting with 30-day predictions",
            vec!["ts-expert", "dl-expert"],
            vec![AlgorithmCapability::TimeSeriesAnalysis, AlgorithmCapability::DeepLearning]
        ),
        (
            "Reinforcement Learning Task",
            "Implement a PPO agent for playing Atari games using OpenAI Gym",
            vec!["rl-expert"],
            vec![AlgorithmCapability::ReinforcementLearning, AlgorithmCapability::DeepLearning]
        ),
        (
            "Data Engineering Task",
            "Build an ETL pipeline for preprocessing large-scale image datasets",
            vec!["data-engineer", "cv-expert"],
            vec![AlgorithmCapability::DataEngineering]
        ),
        (
            "Optimization Task",
            "Optimize hyperparameters for a deep neural network using Bayesian optimization",
            vec!["optimization-expert", "dl-expert"],
            vec![AlgorithmCapability::Optimization, AlgorithmCapability::HyperparameterTuning]
        ),
        (
            "Model Evaluation Task",
            "Evaluate model performance using cross-validation and generate SHAP explanations",
            vec!["evaluator"],
            vec![AlgorithmCapability::ModelEvaluation, AlgorithmCapability::ExplainableAI]
        ),
        (
            "Clustering Task",
            "Implement DBSCAN clustering for anomaly detection in sensor data",
            vec!["ml-expert"],
            vec![AlgorithmCapability::Clustering, AlgorithmCapability::AnomalyDetection]
        ),
    ];

    for (task_name, description, expected_agents, expected_caps) in examples {
        println!("Task: {}", task_name);
        println!("  Description: {}", description);
        println!("  Would route to: {:?}", expected_agents);
        println!("  Required capabilities:");
        for cap in expected_caps {
            println!("    - {}", cap);
        }
        println!();
    }
}

/// Example: Complex workflow - End-to-end ML pipeline
#[allow(dead_code)]
async fn complex_workflow_example() {
    println!("\n=== Complex Workflow: End-to-End ML Pipeline ===\n");

    println!("Task: Build a complete computer vision pipeline for product defect detection\n");

    println!("Phase 1: Data Engineering (data-engineer)");
    println!("  - Collect and preprocess manufacturing images");
    println!("  - Implement data augmentation");
    println!("  - Create train/validation/test splits\n");

    println!("Phase 2: Model Development (cv-expert, dl-expert)");
    println!("  - Design CNN architecture for defect detection");
    println!("  - Implement transfer learning with pre-trained models");
    println!("  - Train model with appropriate loss functions\n");

    println!("Phase 3: Optimization (optimization-expert)");
    println!("  - Hyperparameter tuning using grid search");
    println!("  - Model compression for edge deployment");
    println!("  - Performance profiling\n");

    println!("Phase 4: Evaluation (evaluator)");
    println!("  - Compute precision, recall, F1 scores");
    println!("  - Generate confusion matrix");
    println!("  - Create SHAP explanations for predictions\n");

    println!("Phase 5: Deployment Preparation (optimization-expert)");
    println!("  - Convert to ONNX format");
    println!("  - Optimize for edge devices");
    println!("  - Create deployment documentation\n");

    println!("Coordinator manages the entire pipeline across multiple experts!");
}

/// Example: Research collaboration
#[allow(dead_code)]
async fn research_collaboration_example() {
    println!("\n=== Research Collaboration Example ===\n");

    println!("Research Goal: Improve state-of-the-art on ImageNet classification\n");

    println!("Phase 1: Literature Review (cv-expert, dl-expert)");
    println!("  - Survey recent papers on vision transformers");
    println!("  - Analyze architectural innovations");
    println!("  - Identify promising research directions\n");

    println!("Phase 2: Experimentation (dl-expert, optimization-expert)");
    println!("  - Implement novel architecture variants");
    println!("  - Design efficient training strategies");
    println!("  - Conduct ablation studies\n");

    println!("Phase 3: Analysis (evaluator, ml-expert)");
    println!("  - Statistical significance testing");
    println!("  - Complexity analysis");
    println!("  - Interpretability studies\n");

    println!("Multiple experts collaborate to advance the research!");
}
