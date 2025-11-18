//! Multi-agent collaboration example
//!
//! This example demonstrates how to set up and use multiple agents
//! to collaborate on a complex software development task.

use codex_core::multi_agent::{
    AgentCapability, AgentPool, AgentPoolConfig, AgentRole, AgentSpec, CoordinatorConfig,
    CoordinatorTask, TaskDistributionStrategy,
};

/// Example: Setting up a multi-agent development team
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Multi-Agent Collaboration Example");
    println!("==================================\n");

    // Define specialized agents for different tasks
    let agents = create_development_team();

    // Create coordinator configuration
    let config = CoordinatorConfig {
        pool_config: AgentPoolConfig {
            max_agents: 10,
            max_concurrent_tasks: 20,
            enable_event_aggregation: true,
        },
        distribution_strategy: TaskDistributionStrategy::BestMatch,
        agent_specs: agents,
        verbose: true,
    };

    println!("Coordinator configured with:");
    println!("  - Max agents: {}", config.pool_config.max_agents);
    println!("  - Max tasks: {}", config.pool_config.max_concurrent_tasks);
    println!("  - Strategy: {:?}\n", config.distribution_strategy);

    // Create the coordinator
    let _coordinator = CoordinatorTask::new(config);

    println!("Multi-agent system ready!");
    println!("\nExample tasks that can be delegated:");
    println!("  1. 'Implement a REST API for user authentication'");
    println!("     -> Routed to: backend-developer");
    println!("  2. 'Review the authentication code for security issues'");
    println!("     -> Routed to: security-reviewer");
    println!("  3. 'Write integration tests for the API'");
    println!("     -> Routed to: test-engineer");
    println!("  4. 'Document the API endpoints'");
    println!("     -> Routed to: technical-writer");

    Ok(())
}

/// Create a team of specialized agents
fn create_development_team() -> Vec<AgentSpec> {
    vec![
        // Backend developer agent
        AgentSpec::new(
            "backend-developer",
            AgentRole::Specialist {
                domain: "backend".to_string(),
            },
        )
        .with_capabilities(vec![
            AgentCapability::CodeGeneration,
            AgentCapability::Debugging,
            AgentCapability::Architecture,
        ])
        .with_instructions("You are a backend developer specializing in Rust and API development. Focus on creating efficient, secure, and well-structured code.")
        .with_max_concurrent_tasks(3)
        .with_priority(10),
        // Frontend developer agent
        AgentSpec::new(
            "frontend-developer",
            AgentRole::Specialist {
                domain: "frontend".to_string(),
            },
        )
        .with_capabilities(vec![
            AgentCapability::CodeGeneration,
            AgentCapability::Debugging,
        ])
        .with_instructions("You are a frontend developer specializing in TypeScript and React. Create beautiful, responsive user interfaces.")
        .with_max_concurrent_tasks(3)
        .with_priority(10),
        // Code reviewer agent
        AgentSpec::new(
            "security-reviewer",
            AgentRole::Specialist {
                domain: "security".to_string(),
            },
        )
        .with_capabilities(vec![
            AgentCapability::CodeReview,
            AgentCapability::Research,
        ])
        .with_instructions("You are a security-focused code reviewer. Look for vulnerabilities, security issues, and best practices.")
        .with_max_concurrent_tasks(2)
        .with_priority(8),
        // Test engineer agent
        AgentSpec::new(
            "test-engineer",
            AgentRole::Specialist {
                domain: "testing".to_string(),
            },
        )
        .with_capabilities(vec![
            AgentCapability::Testing,
            AgentCapability::CodeGeneration,
            AgentCapability::Debugging,
        ])
        .with_instructions("You are a test engineer. Write comprehensive tests including unit, integration, and end-to-end tests.")
        .with_max_concurrent_tasks(3)
        .with_priority(9),
        // Documentation specialist agent
        AgentSpec::new(
            "technical-writer",
            AgentRole::Specialist {
                domain: "documentation".to_string(),
            },
        )
        .with_capabilities(vec![
            AgentCapability::Documentation,
            AgentCapability::Research,
        ])
        .with_instructions("You are a technical writer. Create clear, comprehensive documentation for developers and users.")
        .with_max_concurrent_tasks(2)
        .with_priority(7),
        // Architecture/Design agent
        AgentSpec::new(
            "architect",
            AgentRole::Specialist {
                domain: "architecture".to_string(),
            },
        )
        .with_capabilities(vec![
            AgentCapability::Architecture,
            AgentCapability::Research,
            AgentCapability::CodeReview,
        ])
        .with_instructions("You are a software architect. Design scalable, maintainable system architectures and review architectural decisions.")
        .with_max_concurrent_tasks(1)
        .with_priority(10),
        // DevOps specialist agent
        AgentSpec::new(
            "devops-specialist",
            AgentRole::Specialist {
                domain: "devops".to_string(),
            },
        )
        .with_capabilities(vec![
            AgentCapability::ShellExecution,
            AgentCapability::CodeGeneration,
            AgentCapability::Debugging,
        ])
        .with_instructions("You are a DevOps specialist. Handle deployment, CI/CD, infrastructure, and operational tasks.")
        .with_max_concurrent_tasks(2)
        .with_priority(8),
        // General purpose helper agent
        AgentSpec::new("general-helper", AgentRole::GeneralPurpose)
            .with_capabilities(vec![
                AgentCapability::CodeGeneration,
                AgentCapability::Research,
                AgentCapability::Documentation,
            ])
            .with_instructions("You are a general-purpose assistant. Help with various tasks that don't require specialized knowledge.")
            .with_max_concurrent_tasks(5)
            .with_priority(5),
    ]
}

/// Example: Demonstrating different distribution strategies
#[allow(dead_code)]
fn demonstrate_distribution_strategies() {
    println!("\nDistribution Strategies:\n");

    println!("1. RoundRobin:");
    println!("   Tasks are distributed evenly across all capable agents");
    println!("   Use when: All agents are equally capable and you want even load\n");

    println!("2. LeastLoaded:");
    println!("   Tasks go to the agent with the fewest active tasks");
    println!("   Use when: Minimizing wait time is important\n");

    println!("3. BestMatch:");
    println!("   Tasks go to agents with the most matching capabilities");
    println!("   Use when: Task-agent specialization is important\n");

    println!("4. Priority:");
    println!("   Tasks go to the highest priority agent");
    println!("   Use when: You want explicit control over agent selection\n");

    println!("5. Random:");
    println!("   Tasks are randomly distributed to capable agents");
    println!("   Use when: Simple load balancing without determinism\n");
}

/// Example: Complex multi-agent workflow
#[allow(dead_code)]
async fn complex_workflow_example() {
    println!("\nComplex Workflow Example:");
    println!("Task: Build a new microservice\n");

    println!("Phase 1: Design (Architect)");
    println!("  - Define service boundaries");
    println!("  - Design API contracts");
    println!("  - Plan data models\n");

    println!("Phase 2: Parallel Implementation");
    println!("  - Backend Developer: Implement API");
    println!("  - Frontend Developer: Build UI");
    println!("  - Test Engineer: Prepare test framework\n");

    println!("Phase 3: Quality Assurance");
    println!("  - Security Reviewer: Security audit");
    println!("  - Test Engineer: Run test suite");
    println!("  - Technical Writer: API documentation\n");

    println!("Phase 4: Deployment (DevOps)");
    println!("  - Set up CI/CD pipeline");
    println!("  - Configure infrastructure");
    println!("  - Deploy to staging\n");

    println!("Coordinator manages the entire workflow!");
}
