//! Deep Learning Multi-Agent System Example
//!
//! This example demonstrates the deep learning multi-agent system with:
//! - Pre-configured deep learning experts (PyTorch, Training, Vision, Transformer, etc.)
//! - Custom deep learning agents
//! - Smart task routing based on framework, architecture, and capabilities
//! - Different deep learning workflows

use codex_core::multi_agent::{
    dl_presets, AgentCapability, AgentRole, ApplicationDomain, DeepLearningAgent,
    DeepLearningCapability, DeepLearningCoordinator, DeepLearningCoordinatorConfig,
    DeepLearningFramework, ModelArchitecture, TaskDistributionStrategy, TrainingStrategy,
};

fn main() {
    println!("======================================");
    println!("Deep Learning Multi-Agent System");
    println!("======================================\n");

    // Example 1: Full deep learning team
    println!("Example 1: Full Deep Learning Team");
    println!("-----------------------------------");
    full_dl_team();

    println!("\n");

    // Example 2: Computer vision pipeline
    println!("Example 2: Computer Vision Pipeline");
    println!("-------------------------------------");
    cv_pipeline();

    println!("\n");

    // Example 3: NLP transformer team
    println!("Example 3: NLP Transformer Team");
    println!("---------------------------------");
    nlp_transformer_team();

    println!("\n");

    // Example 4: Model optimization team
    println!("Example 4: Model Optimization Team");
    println!("------------------------------------");
    optimization_team();

    println!("\n");

    // Example 5: Custom deep learning agent
    println!("Example 5: Custom DL Agent");
    println!("---------------------------");
    custom_dl_agent();

    println!("\n");

    // Example 6: End-to-end training workflow
    println!("Example 6: End-to-End Training Workflow");
    println!("----------------------------------------");
    training_workflow();

    println!("\n");

    // Example 7: Model deployment pipeline
    println!("Example 7: Model Deployment Pipeline");
    println!("--------------------------------------");
    deployment_pipeline();
}

/// Example 1: Full deep learning team with all experts
fn full_dl_team() {
    let experts = dl_presets::all_dl_experts();

    println!("Deep Learning Team Size: {} experts", experts.len());
    println!("\nTeam Members:");
    for expert in &experts {
        println!("  • {} - {:?}", expert.spec.id.0, expert.spec.role);
        if !expert.frameworks.is_empty() {
            println!(
                "    Frameworks: {}",
                expert
                    .frameworks
                    .iter()
                    .map(|f| format!("{}", f))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        if !expert.dl_capabilities.is_empty() {
            let caps: Vec<_> = expert.dl_capabilities.iter().take(3).collect();
            println!(
                "    Capabilities: {} (+{})",
                caps.iter()
                    .map(|c| format!("{}", c))
                    .collect::<Vec<_>>()
                    .join(", "),
                expert.dl_capabilities.len() - caps.len()
            );
        }
    }

    let config = DeepLearningCoordinatorConfig {
        dl_experts: experts,
        distribution_strategy: TaskDistributionStrategy::BestMatch,
        auto_infer: true,
        verbose: true,
        ..Default::default()
    };

    let _coordinator = DeepLearningCoordinator::new(config);
    println!("\n✓ Full deep learning team coordinator created");
}

/// Example 2: Computer vision pipeline team
fn cv_pipeline() {
    let team = vec![
        dl_presets::vision_expert(),
        dl_presets::data_pipeline_expert(),
        dl_presets::training_specialist(),
        dl_presets::optimization_expert(),
    ];

    println!("Computer Vision Pipeline Team:");
    for expert in &team {
        println!("  • {} - {:?}", expert.spec.id.0, expert.spec.role);
        if !expert.architectures.is_empty() {
            println!(
                "    Architectures: {}",
                expert
                    .architectures
                    .iter()
                    .map(|a| format!("{}", a))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }

    let config = DeepLearningCoordinatorConfig {
        dl_experts: team,
        ..Default::default()
    };

    let _coordinator = DeepLearningCoordinator::new(config);
    println!("\n✓ Computer vision pipeline coordinator created");
}

/// Example 3: NLP transformer team
fn nlp_transformer_team() {
    let team = vec![
        dl_presets::transformer_expert(),
        dl_presets::data_pipeline_expert(),
        dl_presets::training_specialist(),
        dl_presets::tuning_expert(),
    ];

    println!("NLP Transformer Team:");
    for expert in &team {
        println!("  • {} - {:?}", expert.spec.id.0, expert.spec.role);
        if !expert.libraries.is_empty() {
            println!("    Libraries: {}", expert.libraries.join(", "));
        }
    }

    let config = DeepLearningCoordinatorConfig {
        dl_experts: team,
        distribution_strategy: TaskDistributionStrategy::BestMatch,
        ..Default::default()
    };

    let _coordinator = DeepLearningCoordinator::new(config);
    println!("\n✓ NLP transformer team coordinator created");
}

/// Example 4: Model optimization team
fn optimization_team() {
    let team = vec![
        dl_presets::optimization_expert(),
        dl_presets::deployment_specialist(),
        dl_presets::debugging_expert(),
    ];

    println!("Model Optimization Team:");
    for expert in &team {
        println!("  • {} - {:?}", expert.spec.id.0, expert.spec.role);
        println!(
            "    Capabilities: {}",
            expert
                .dl_capabilities
                .iter()
                .map(|c| format!("{}", c))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    let config = DeepLearningCoordinatorConfig {
        dl_experts: team,
        ..Default::default()
    };

    let _coordinator = DeepLearningCoordinator::new(config);
    println!("\n✓ Model optimization team coordinator created");
}

/// Example 5: Custom deep learning agent
fn custom_dl_agent() {
    let custom_expert = DeepLearningAgent::new(
        "multimodal-expert",
        AgentRole::Specialist {
            domain: "multimodal".to_string(),
        },
    )
    .with_framework(DeepLearningFramework::PyTorch)
    .with_framework(DeepLearningFramework::JAX)
    .with_architectures(vec![
        ModelArchitecture::CLIP,
        ModelArchitecture::ViT,
        ModelArchitecture::Transformer,
    ])
    .with_dl_capabilities(vec![
        DeepLearningCapability::ArchitectureDesign,
        DeepLearningCapability::TransferLearning,
        DeepLearningCapability::TrainingPipeline,
    ])
    .with_application_domain(ApplicationDomain::ComputerVision)
    .with_application_domain(ApplicationDomain::NaturalLanguageProcessing)
    .with_training_strategy(TrainingStrategy::DistributedDataParallel)
    .with_capability(AgentCapability::CodeGeneration)
    .with_library("transformers")
    .with_library("timm")
    .with_library("open_clip")
    .with_instructions(
        "Multimodal AI expert specializing in vision-language models like CLIP and Flamingo",
    )
    .with_priority(10);

    println!("Custom Multimodal Expert:");
    println!("  ID: {}", custom_expert.spec.id.0);
    println!("  Role: {:?}", custom_expert.spec.role);
    println!(
        "  Frameworks: {}",
        custom_expert
            .frameworks
            .iter()
            .map(|f| format!("{}", f))
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!(
        "  Architectures: {}",
        custom_expert
            .architectures
            .iter()
            .map(|a| format!("{}", a))
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("  Libraries: {}", custom_expert.libraries.join(", "));
    println!(
        "  Instructions: {}",
        custom_expert
            .spec
            .instructions
            .as_ref()
            .unwrap_or(&"".to_string())
    );

    let team = vec![
        custom_expert,
        dl_presets::training_specialist(),
        dl_presets::experiment_specialist(),
    ];

    let config = DeepLearningCoordinatorConfig {
        dl_experts: team,
        ..Default::default()
    };

    let _coordinator = DeepLearningCoordinator::new(config);
    println!("\n✓ Custom multimodal team coordinator created");
}

/// Example 6: End-to-end training workflow
fn training_workflow() {
    println!("Simulating End-to-End Deep Learning Workflow:");
    println!("\nPhase 1: Data Preparation");
    println!("  Assigned to: Data Pipeline Expert");
    println!("  Tasks:");
    println!("    - Design data loading pipeline");
    println!("    - Implement data augmentation");
    println!("    - Set up DataLoader with multi-processing");

    println!("\nPhase 2: Model Architecture");
    println!("  Assigned to: PyTorch Architect");
    println!("  Tasks:");
    println!("    - Design custom model architecture");
    println!("    - Implement custom layers");
    println!("    - Add attention mechanisms");

    println!("\nPhase 3: Training Setup");
    println!("  Assigned to: Training Specialist");
    println!("  Tasks:");
    println!("    - Configure distributed training");
    println!("    - Set up mixed precision training");
    println!("    - Implement learning rate scheduling");

    println!("\nPhase 4: Experiment Tracking");
    println!("  Assigned to: Experiment Specialist");
    println!("  Tasks:");
    println!("    - Set up WandB logging");
    println!("    - Configure TensorBoard");
    println!("    - Implement checkpoint management");

    println!("\nPhase 5: Hyperparameter Tuning");
    println!("  Assigned to: Tuning Expert");
    println!("  Tasks:");
    println!("    - Define search space");
    println!("    - Run Optuna optimization");
    println!("    - Perform ablation studies");

    println!("\nPhase 6: Debugging & Profiling");
    println!("  Assigned to: Debugging Expert");
    println!("  Tasks:");
    println!("    - Analyze gradient flow");
    println!("    - Visualize activations");
    println!("    - Profile memory usage");

    println!("\nPhase 7: Model Optimization");
    println!("  Assigned to: Optimization Expert");
    println!("  Tasks:");
    println!("    - Apply model pruning");
    println!("    - Quantize to INT8");
    println!("    - Knowledge distillation");

    println!("\n✓ Training workflow completed");
}

/// Example 7: Model deployment pipeline
fn deployment_pipeline() {
    println!("Simulating Model Deployment Pipeline:");
    println!("\nPhase 1: Model Export");
    println!("  Assigned to: Deployment Specialist");
    println!("  Tasks:");
    println!("    - Export PyTorch model to ONNX");
    println!("    - Verify ONNX model correctness");
    println!("    - Optimize ONNX graph");

    println!("\nPhase 2: Platform-Specific Optimization");
    println!("  Assigned to: Optimization Expert, Deployment Specialist");
    println!("  Tasks:");
    println!("    - TensorRT optimization for GPU");
    println!("    - TFLite conversion for mobile");
    println!("    - CoreML conversion for iOS");

    println!("\nPhase 3: Performance Testing");
    println!("  Assigned to: Debugging Expert");
    println!("  Tasks:");
    println!("    - Benchmark inference latency");
    println!("    - Measure memory footprint");
    println!("    - Test different batch sizes");

    println!("\nPhase 4: Deployment");
    println!("  Assigned to: Deployment Specialist");
    println!("  Tasks:");
    println!("    - Deploy to edge devices");
    println!("    - Set up model serving");
    println!("    - Configure auto-scaling");

    println!("\n✓ Deployment pipeline completed");
}

/// Example 8: Framework comparison
#[allow(dead_code)]
fn framework_comparison() {
    println!("Framework Expertise Distribution:");

    println!("\n1. PyTorch Ecosystem:");
    let pytorch_team = vec![
        dl_presets::pytorch_architect(),
        dl_presets::vision_expert(),
        dl_presets::transformer_expert(),
        dl_presets::training_specialist(),
    ];
    println!("   Team Size: {} experts", pytorch_team.len());

    println!("\n2. Multi-Framework Team:");
    let multi_team = dl_presets::all_dl_experts();
    println!("   Team Size: {} experts", multi_team.len());
    println!("   Frameworks: PyTorch, TensorFlow, JAX, ONNX, TensorRT");

    println!("\n✓ Framework comparison demonstrated");
}

/// Example 9: Application domain specialization
#[allow(dead_code)]
fn domain_specialization() {
    println!("Application Domain Specialization:");

    println!("\n1. Computer Vision:");
    println!("   - Vision Expert (CNN, YOLO, UNet, ViT)");
    println!("   - Data Pipeline Expert");
    println!("   - Training Specialist");

    println!("\n2. NLP:");
    println!("   - Transformer Expert (BERT, GPT, T5)");
    println!("   - Data Pipeline Expert");
    println!("   - Tuning Expert");

    println!("\n3. Multimodal:");
    println!("   - Custom Multimodal Expert (CLIP, ViT)");
    println!("   - Vision Expert");
    println!("   - Transformer Expert");

    println!("\n✓ Domain specialization demonstrated");
}

/// Example 10: Training strategies
#[allow(dead_code)]
fn training_strategies() {
    println!("Training Strategy Examples:");

    println!("\n1. Standard Training:");
    println!("   - Single GPU");
    println!("   - FP32 precision");
    println!("   - Batch size: 32");

    println!("\n2. Distributed Data Parallel:");
    println!("   - Multi-GPU");
    println!("   - Data parallelism");
    println!("   - Synchronized gradients");

    println!("\n3. Mixed Precision Training:");
    println!("   - FP16 + FP32");
    println!("   - Gradient scaling");
    println!("   - Memory efficient");

    println!("\n4. Gradient Accumulation:");
    println!("   - Large effective batch size");
    println!("   - Limited GPU memory");
    println!("   - Accumulate over N steps");

    println!("\n✓ Training strategies demonstrated");
}
