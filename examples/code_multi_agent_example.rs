//! Software Development Multi-Agent System Example
//!
//! This example demonstrates the code development multi-agent system with:
//! - Pre-configured development experts (Rust, Frontend, Backend, DevOps, etc.)
//! - Custom development agents
//! - Smart task routing based on language and domain
//! - Different development workflows

use codex_core::multi_agent::{
    code_presets, AgentCapability, AgentPoolConfig, AgentRole, CodeCoordinator,
    CodeCoordinatorConfig, DevCapability, DevelopmentAgent, DevelopmentDomain,
    ProgrammingLanguage, TaskDistributionStrategy,
};

fn main() {
    println!("========================================");
    println!("Software Development Multi-Agent System");
    println!("========================================\n");

    // Example 1: Full development team
    println!("Example 1: Full Development Team");
    println!("---------------------------------");
    full_development_team();

    println!("\n");

    // Example 2: Frontend-focused team
    println!("Example 2: Frontend-Focused Team");
    println!("----------------------------------");
    frontend_team();

    println!("\n");

    // Example 3: Backend and DevOps team
    println!("Example 3: Backend + DevOps Team");
    println!("----------------------------------");
    backend_devops_team();

    println!("\n");

    // Example 4: Security-focused team
    println!("Example 4: Security-Focused Team");
    println!("----------------------------------");
    security_team();

    println!("\n");

    // Example 5: Custom development agent
    println!("Example 5: Custom Development Agent");
    println!("------------------------------------");
    custom_agent();

    println!("\n");

    // Example 6: Language detection
    println!("Example 6: Language Detection");
    println!("------------------------------");
    language_detection();

    println!("\n");

    // Example 7: Development workflow simulation
    println!("Example 7: Development Workflow");
    println!("--------------------------------");
    development_workflow();
}

/// Example 1: Full development team with all experts
fn full_development_team() {
    let experts = code_presets::all_dev_experts();

    println!("Development Team Size: {} experts", experts.len());
    println!("\nTeam Members:");
    for expert in &experts {
        println!("  • {} - {:?}", expert.spec.id.0, expert.spec.role);
        if !expert.languages.is_empty() {
            println!(
                "    Languages: {}",
                expert
                    .languages
                    .iter()
                    .map(|l| format!("{}", l))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        if !expert.domains.is_empty() {
            println!(
                "    Domains: {}",
                expert
                    .domains
                    .iter()
                    .map(|d| format!("{:?}", d))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }

    let config = CodeCoordinatorConfig {
        dev_experts: experts,
        distribution_strategy: TaskDistributionStrategy::BestMatch,
        auto_infer: true,
        verbose: true,
        ..Default::default()
    };

    let _coordinator = CodeCoordinator::new(config);
    println!("\n✓ Full development team coordinator created");
}

/// Example 2: Frontend-focused team
fn frontend_team() {
    let team = vec![
        code_presets::frontend_expert(),
        code_presets::testing_specialist(),
        code_presets::code_reviewer(),
    ];

    println!("Frontend Team:");
    for expert in &team {
        println!("  • {} - {:?}", expert.spec.id.0, expert.spec.role);
        if !expert.frameworks.is_empty() {
            println!("    Frameworks: {}", expert.frameworks.join(", "));
        }
    }

    let config = CodeCoordinatorConfig {
        dev_experts: team,
        ..Default::default()
    };

    let _coordinator = CodeCoordinator::new(config);
    println!("\n✓ Frontend team coordinator created");
}

/// Example 3: Backend and DevOps team
fn backend_devops_team() {
    let team = vec![
        code_presets::backend_expert(),
        code_presets::database_specialist(),
        code_presets::devops_engineer(),
        code_presets::performance_expert(),
    ];

    println!("Backend + DevOps Team:");
    for expert in &team {
        println!("  • {} - {:?}", expert.spec.id.0, expert.spec.role);
        if !expert.tools.is_empty() {
            println!("    Tools: {}", expert.tools.join(", "));
        }
    }

    let config = CodeCoordinatorConfig {
        dev_experts: team,
        distribution_strategy: TaskDistributionStrategy::LeastLoaded,
        ..Default::default()
    };

    let _coordinator = CodeCoordinator::new(config);
    println!("\n✓ Backend + DevOps coordinator created");
}

/// Example 4: Security-focused team
fn security_team() {
    let team = vec![
        code_presets::security_specialist(),
        code_presets::code_reviewer(),
        code_presets::testing_specialist(),
    ];

    println!("Security Team:");
    for expert in &team {
        println!("  • {} - {:?}", expert.spec.id.0, expert.spec.role);
        println!(
            "    Capabilities: {}",
            expert
                .dev_capabilities
                .iter()
                .map(|c| format!("{}", c))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    let config = CodeCoordinatorConfig {
        dev_experts: team,
        ..Default::default()
    };

    let _coordinator = CodeCoordinator::new(config);
    println!("\n✓ Security team coordinator created");
}

/// Example 5: Custom development agent
fn custom_agent() {
    let custom_expert = DevelopmentAgent::new(
        "web3-expert",
        AgentRole::Specialist {
            domain: "web3".to_string(),
        },
    )
    .with_language(ProgrammingLanguage::JavaScript)
    .with_language(ProgrammingLanguage::TypeScript)
    .with_language(ProgrammingLanguage::Rust)
    .with_domain(DevelopmentDomain::Web3)
    .with_domain(DevelopmentDomain::Backend)
    .with_dev_capability(DevCapability::Implementation)
    .with_dev_capability(DevCapability::SecurityAudit)
    .with_dev_capability(DevCapability::CodeReview)
    .with_capability(AgentCapability::CodeGeneration)
    .with_capability(AgentCapability::CodeReview)
    .with_framework("ethers.js")
    .with_framework("Solana SDK")
    .with_framework("Anchor")
    .with_tool("Hardhat")
    .with_tool("Foundry")
    .with_tool("Remix")
    .with_instructions("Web3 and blockchain development expert specializing in smart contracts, DeFi, and dApp development")
    .with_priority(10);

    println!("Custom Web3 Expert:");
    println!("  ID: {}", custom_expert.spec.id.0);
    println!("  Role: {:?}", custom_expert.spec.role);
    println!(
        "  Languages: {}",
        custom_expert
            .languages
            .iter()
            .map(|l| format!("{}", l))
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!(
        "  Frameworks: {}",
        custom_expert.frameworks.join(", ")
    );
    println!("  Tools: {}", custom_expert.tools.join(", "));
    println!("  Instructions: {}", custom_expert.spec.instructions.as_ref().unwrap_or(&"".to_string()));

    let team = vec![
        custom_expert,
        code_presets::security_specialist(),
        code_presets::testing_specialist(),
    ];

    let config = CodeCoordinatorConfig {
        dev_experts: team,
        ..Default::default()
    };

    let _coordinator = CodeCoordinator::new(config);
    println!("\n✓ Custom Web3 team coordinator created");
}

/// Example 6: Language detection demonstration
fn language_detection() {
    let test_cases = vec![
        ("Implement a Rust async web server", vec![ProgrammingLanguage::Rust]),
        (
            "Build a React frontend with TypeScript",
            vec![ProgrammingLanguage::TypeScript],
        ),
        (
            "Create Python API using FastAPI",
            vec![ProgrammingLanguage::Python],
        ),
        (
            "Write a Go microservice",
            vec![ProgrammingLanguage::Go],
        ),
        (
            "Implement .js and .ts files",
            vec![ProgrammingLanguage::JavaScript, ProgrammingLanguage::TypeScript],
        ),
    ];

    println!("Language Detection Examples:");
    for (description, expected) in test_cases {
        println!("\n  Task: \"{}\"", description);
        print!("  Detected: ");
        for lang in expected {
            print!("{} ", lang);
        }
        println!();
    }

    println!("\n✓ Language detection patterns demonstrated");
}

/// Example 7: Development workflow simulation
fn development_workflow() {
    println!("Simulating Full-Stack Development Workflow:");
    println!("\nPhase 1: Planning & Architecture");
    println!("  Assigned to: Backend Expert");
    println!("  Tasks:");
    println!("    - Design system architecture");
    println!("    - Define API contracts");
    println!("    - Plan database schema");

    println!("\nPhase 2: Backend Development");
    println!("  Assigned to: Backend Expert, Database Specialist");
    println!("  Tasks:");
    println!("    - Implement REST API endpoints");
    println!("    - Set up PostgreSQL database");
    println!("    - Create authentication system");

    println!("\nPhase 3: Frontend Development");
    println!("  Assigned to: Frontend Expert");
    println!("  Tasks:");
    println!("    - Build React components");
    println!("    - Implement state management");
    println!("    - Create responsive UI");

    println!("\nPhase 4: Testing");
    println!("  Assigned to: Testing Specialist");
    println!("  Tasks:");
    println!("    - Write unit tests");
    println!("    - Implement integration tests");
    println!("    - Run E2E tests");

    println!("\nPhase 5: Security Review");
    println!("  Assigned to: Security Specialist");
    println!("  Tasks:");
    println!("    - Security audit");
    println!("    - Vulnerability scanning");
    println!("    - OWASP compliance check");

    println!("\nPhase 6: Code Review");
    println!("  Assigned to: Code Reviewer");
    println!("  Tasks:");
    println!("    - Review code quality");
    println!("    - Check best practices");
    println!("    - Ensure documentation");

    println!("\nPhase 7: Performance Optimization");
    println!("  Assigned to: Performance Expert");
    println!("  Tasks:");
    println!("    - Profile application");
    println!("    - Optimize bottlenecks");
    println!("    - Load testing");

    println!("\nPhase 8: Deployment");
    println!("  Assigned to: DevOps Engineer");
    println!("  Tasks:");
    println!("    - Set up CI/CD pipeline");
    println!("    - Configure Kubernetes deployment");
    println!("    - Set up monitoring and logging");

    println!("\n✓ Development workflow completed");
}

/// Example 8: Task distribution strategies
#[allow(dead_code)]
fn task_distribution_strategies() {
    println!("Task Distribution Strategies:");

    println!("\n1. BestMatch Strategy:");
    println!("   - Matches task requirements to agent capabilities");
    println!("   - Best for heterogeneous tasks");
    println!("   - Recommended for most use cases");

    let config1 = CodeCoordinatorConfig {
        dev_experts: code_presets::all_dev_experts(),
        distribution_strategy: TaskDistributionStrategy::BestMatch,
        ..Default::default()
    };
    let _coordinator1 = CodeCoordinator::new(config1);

    println!("\n2. LeastLoaded Strategy:");
    println!("   - Distributes to least busy agent");
    println!("   - Balances workload evenly");
    println!("   - Good for similar difficulty tasks");

    let config2 = CodeCoordinatorConfig {
        dev_experts: code_presets::all_dev_experts(),
        distribution_strategy: TaskDistributionStrategy::LeastLoaded,
        ..Default::default()
    };
    let _coordinator2 = CodeCoordinator::new(config2);

    println!("\n3. Priority Strategy:");
    println!("   - Routes to highest priority agent");
    println!("   - Good for critical path optimization");
    println!("   - Respects expert seniority");

    let config3 = CodeCoordinatorConfig {
        dev_experts: code_presets::all_dev_experts(),
        distribution_strategy: TaskDistributionStrategy::Priority,
        ..Default::default()
    };
    let _coordinator3 = CodeCoordinator::new(config3);

    println!("\n✓ Distribution strategies demonstrated");
}

/// Example 9: Agent pool configuration
#[allow(dead_code)]
fn agent_pool_configuration() {
    println!("Agent Pool Configuration:");

    // Small team configuration
    println!("\n1. Small Team (5 agents, 10 concurrent tasks):");
    let small_config = CodeCoordinatorConfig {
        pool_config: AgentPoolConfig {
            max_agents: 5,
            max_concurrent_tasks: 10,
            enable_event_aggregation: false,
        },
        dev_experts: vec![
            code_presets::rust_expert(),
            code_presets::frontend_expert(),
            code_presets::backend_expert(),
            code_presets::testing_specialist(),
            code_presets::code_reviewer(),
        ],
        ..Default::default()
    };
    let _small_team = CodeCoordinator::new(small_config);
    println!("   ✓ Small team coordinator created");

    // Medium team configuration
    println!("\n2. Medium Team (10 agents, 20 concurrent tasks):");
    let medium_config = CodeCoordinatorConfig {
        pool_config: AgentPoolConfig {
            max_agents: 10,
            max_concurrent_tasks: 20,
            enable_event_aggregation: true,
        },
        dev_experts: code_presets::all_dev_experts(),
        ..Default::default()
    };
    let _medium_team = CodeCoordinator::new(medium_config);
    println!("   ✓ Medium team coordinator created");

    // Large team configuration
    println!("\n3. Large Team (20 agents, 50 concurrent tasks):");
    let large_config = CodeCoordinatorConfig {
        pool_config: AgentPoolConfig {
            max_agents: 20,
            max_concurrent_tasks: 50,
            enable_event_aggregation: true,
        },
        dev_experts: code_presets::all_dev_experts(),
        verbose: true,
        ..Default::default()
    };
    let _large_team = CodeCoordinator::new(large_config);
    println!("   ✓ Large team coordinator created");

    println!("\n✓ Pool configurations demonstrated");
}
