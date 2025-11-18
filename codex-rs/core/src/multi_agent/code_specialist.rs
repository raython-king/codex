//! Software development specialized multi-agent system
//!
//! This module provides a comprehensive multi-agent system for software
//! development, inspired by Claude Code's capabilities. It includes
//! specialized agents for different aspects of software engineering.

use super::types::{AgentCapability, AgentRole, AgentSpec};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Programming languages and their ecosystems
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProgrammingLanguage {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Go,
    Java,
    Cpp,
    C,
    CSharp,
    Ruby,
    PHP,
    Swift,
    Kotlin,
    Scala,
    Haskell,
    Elixir,
    Clojure,
    R,
    Lua,
    Dart,
    SQL,
    Shell,
    Custom(String),
}

impl std::fmt::Display for ProgrammingLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgrammingLanguage::Rust => write!(f, "Rust"),
            ProgrammingLanguage::Python => write!(f, "Python"),
            ProgrammingLanguage::JavaScript => write!(f, "JavaScript"),
            ProgrammingLanguage::TypeScript => write!(f, "TypeScript"),
            ProgrammingLanguage::Go => write!(f, "Go"),
            ProgrammingLanguage::Java => write!(f, "Java"),
            ProgrammingLanguage::Cpp => write!(f, "C++"),
            ProgrammingLanguage::C => write!(f, "C"),
            ProgrammingLanguage::CSharp => write!(f, "C#"),
            ProgrammingLanguage::Ruby => write!(f, "Ruby"),
            ProgrammingLanguage::PHP => write!(f, "PHP"),
            ProgrammingLanguage::Swift => write!(f, "Swift"),
            ProgrammingLanguage::Kotlin => write!(f, "Kotlin"),
            ProgrammingLanguage::Scala => write!(f, "Scala"),
            ProgrammingLanguage::Haskell => write!(f, "Haskell"),
            ProgrammingLanguage::Elixir => write!(f, "Elixir"),
            ProgrammingLanguage::Clojure => write!(f, "Clojure"),
            ProgrammingLanguage::R => write!(f, "R"),
            ProgrammingLanguage::Lua => write!(f, "Lua"),
            ProgrammingLanguage::Dart => write!(f, "Dart"),
            ProgrammingLanguage::SQL => write!(f, "SQL"),
            ProgrammingLanguage::Shell => write!(f, "Shell"),
            ProgrammingLanguage::Custom(name) => write!(f, "{}", name),
        }
    }
}

/// Development domain specialization
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DevelopmentDomain {
    /// Frontend development
    Frontend,
    /// Backend development
    Backend,
    /// Full-stack development
    FullStack,
    /// Mobile development
    Mobile,
    /// Desktop applications
    Desktop,
    /// DevOps and infrastructure
    DevOps,
    /// Database design and optimization
    Database,
    /// Cloud architecture
    Cloud,
    /// Security
    Security,
    /// Performance optimization
    Performance,
    /// API design
    API,
    /// Embedded systems
    Embedded,
    /// Game development
    GameDev,
    /// Web3/Blockchain
    Web3,
    /// Systems programming
    Systems,
}

/// Software development capabilities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DevCapability {
    /// Code implementation
    Implementation,
    /// Code refactoring
    Refactoring,
    /// Bug fixing and debugging
    BugFixing,
    /// Code review
    CodeReview,
    /// Unit testing
    UnitTesting,
    /// Integration testing
    IntegrationTesting,
    /// End-to-end testing
    E2ETesting,
    /// Performance profiling
    PerformanceProfiling,
    /// Security audit
    SecurityAudit,
    /// API design
    APIDesign,
    /// Database design
    DatabaseDesign,
    /// System architecture
    SystemArchitecture,
    /// Microservices
    Microservices,
    /// CI/CD pipeline
    CICD,
    /// Infrastructure as code
    InfrastructureAsCode,
    /// Container orchestration
    ContainerOrchestration,
    /// Monitoring and logging
    MonitoringLogging,
    /// Documentation
    Documentation,
    /// Migration
    Migration,
    /// Legacy code modernization
    LegacyModernization,
    /// Code generation
    CodeGeneration,
    /// Dependency management
    DependencyManagement,
    /// Build optimization
    BuildOptimization,
    /// Custom capability
    Custom(String),
}

impl std::fmt::Display for DevCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DevCapability::Implementation => write!(f, "implementation"),
            DevCapability::Refactoring => write!(f, "refactoring"),
            DevCapability::BugFixing => write!(f, "bug-fixing"),
            DevCapability::CodeReview => write!(f, "code-review"),
            DevCapability::UnitTesting => write!(f, "unit-testing"),
            DevCapability::IntegrationTesting => write!(f, "integration-testing"),
            DevCapability::E2ETesting => write!(f, "e2e-testing"),
            DevCapability::PerformanceProfiling => write!(f, "performance-profiling"),
            DevCapability::SecurityAudit => write!(f, "security-audit"),
            DevCapability::APIDesign => write!(f, "api-design"),
            DevCapability::DatabaseDesign => write!(f, "database-design"),
            DevCapability::SystemArchitecture => write!(f, "system-architecture"),
            DevCapability::Microservices => write!(f, "microservices"),
            DevCapability::CICD => write!(f, "ci-cd"),
            DevCapability::InfrastructureAsCode => write!(f, "infrastructure-as-code"),
            DevCapability::ContainerOrchestration => write!(f, "container-orchestration"),
            DevCapability::MonitoringLogging => write!(f, "monitoring-logging"),
            DevCapability::Documentation => write!(f, "documentation"),
            DevCapability::Migration => write!(f, "migration"),
            DevCapability::LegacyModernization => write!(f, "legacy-modernization"),
            DevCapability::CodeGeneration => write!(f, "code-generation"),
            DevCapability::DependencyManagement => write!(f, "dependency-management"),
            DevCapability::BuildOptimization => write!(f, "build-optimization"),
            DevCapability::Custom(name) => write!(f, "custom:{}", name),
        }
    }
}

/// Development agent with code-specific capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentAgent {
    /// Base agent specification
    pub spec: AgentSpec,
    /// Programming languages expertise
    pub languages: HashSet<ProgrammingLanguage>,
    /// Development domains
    pub domains: HashSet<DevelopmentDomain>,
    /// Development capabilities
    pub dev_capabilities: HashSet<DevCapability>,
    /// Frameworks and libraries
    pub frameworks: Vec<String>,
    /// Tools expertise
    pub tools: Vec<String>,
}

impl DevelopmentAgent {
    /// Create a new development agent
    pub fn new(id: impl Into<String>, role: AgentRole) -> Self {
        let id_string = id.into();
        Self {
            spec: AgentSpec::new(id_string, role),
            languages: HashSet::new(),
            domains: HashSet::new(),
            dev_capabilities: HashSet::new(),
            frameworks: Vec::new(),
            tools: Vec::new(),
        }
    }

    /// Add programming language
    pub fn with_language(mut self, lang: ProgrammingLanguage) -> Self {
        self.languages.insert(lang);
        self
    }

    /// Add multiple languages
    pub fn with_languages(mut self, langs: Vec<ProgrammingLanguage>) -> Self {
        self.languages.extend(langs);
        self
    }

    /// Add development domain
    pub fn with_domain(mut self, domain: DevelopmentDomain) -> Self {
        self.domains.insert(domain);
        self
    }

    /// Add development capability
    pub fn with_dev_capability(mut self, cap: DevCapability) -> Self {
        self.dev_capabilities.insert(cap);
        self
    }

    /// Add multiple development capabilities
    pub fn with_dev_capabilities(mut self, caps: Vec<DevCapability>) -> Self {
        self.dev_capabilities.extend(caps);
        self
    }

    /// Add framework
    pub fn with_framework(mut self, framework: impl Into<String>) -> Self {
        self.frameworks.push(framework.into());
        self
    }

    /// Add tool
    pub fn with_tool(mut self, tool: impl Into<String>) -> Self {
        self.tools.push(tool.into());
        self
    }

    /// Forward to base agent spec
    pub fn with_capability(mut self, cap: AgentCapability) -> Self {
        self.spec = self.spec.with_capability(cap);
        self
    }

    /// Set instructions
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

    /// Check if has language expertise
    pub fn has_language(&self, lang: &ProgrammingLanguage) -> bool {
        self.languages.contains(lang)
    }

    /// Check if has domain expertise
    pub fn has_domain(&self, domain: &DevelopmentDomain) -> bool {
        self.domains.contains(domain)
    }

    /// Check if has dev capability
    pub fn has_dev_capability(&self, cap: &DevCapability) -> bool {
        self.dev_capabilities.contains(cap)
    }

    /// Convert to AgentSpec
    pub fn into_spec(self) -> AgentSpec {
        self.spec
    }
}

/// Presets for common software development agents
pub mod presets {
    use super::*;

    /// Rust expert
    pub fn rust_expert() -> DevelopmentAgent {
        DevelopmentAgent::new(
            "rust-expert",
            AgentRole::Specialist {
                domain: "rust".to_string(),
            },
        )
        .with_language(ProgrammingLanguage::Rust)
        .with_domain(DevelopmentDomain::Backend)
        .with_domain(DevelopmentDomain::Systems)
        .with_domain(DevelopmentDomain::Performance)
        .with_dev_capabilities(vec![
            DevCapability::Implementation,
            DevCapability::Refactoring,
            DevCapability::PerformanceProfiling,
            DevCapability::SystemArchitecture,
        ])
        .with_capability(AgentCapability::CodeGeneration)
        .with_capability(AgentCapability::CodeReview)
        .with_framework("Tokio")
        .with_framework("Actix")
        .with_framework("Axum")
        .with_tool("Cargo")
        .with_tool("Clippy")
        .with_tool("rustfmt")
        .with_instructions(
            "You are a Rust expert specializing in systems programming, async runtime, \
             and high-performance applications. You write safe, idiomatic Rust code.",
        )
        .with_max_concurrent_tasks(3)
        .with_priority(10)
    }

    /// Frontend expert
    pub fn frontend_expert() -> DevelopmentAgent {
        DevelopmentAgent::new(
            "frontend-expert",
            AgentRole::Specialist {
                domain: "frontend".to_string(),
            },
        )
        .with_languages(vec![
            ProgrammingLanguage::JavaScript,
            ProgrammingLanguage::TypeScript,
        ])
        .with_domain(DevelopmentDomain::Frontend)
        .with_dev_capabilities(vec![
            DevCapability::Implementation,
            DevCapability::UnitTesting,
            DevCapability::E2ETesting,
        ])
        .with_capability(AgentCapability::CodeGeneration)
        .with_framework("React")
        .with_framework("Vue")
        .with_framework("Svelte")
        .with_framework("Next.js")
        .with_tool("Webpack")
        .with_tool("Vite")
        .with_tool("ESLint")
        .with_instructions(
            "You are a frontend expert specializing in modern web applications, \
             React ecosystem, and responsive UI/UX implementation.",
        )
        .with_max_concurrent_tasks(3)
        .with_priority(9)
    }

    /// Backend expert
    pub fn backend_expert() -> DevelopmentAgent {
        DevelopmentAgent::new(
            "backend-expert",
            AgentRole::Specialist {
                domain: "backend".to_string(),
            },
        )
        .with_languages(vec![
            ProgrammingLanguage::Python,
            ProgrammingLanguage::Go,
            ProgrammingLanguage::Java,
        ])
        .with_domain(DevelopmentDomain::Backend)
        .with_dev_capabilities(vec![
            DevCapability::Implementation,
            DevCapability::APIDesign,
            DevCapability::DatabaseDesign,
            DevCapability::Microservices,
        ])
        .with_capability(AgentCapability::CodeGeneration)
        .with_capability(AgentCapability::Architecture)
        .with_framework("FastAPI")
        .with_framework("Django")
        .with_framework("Express")
        .with_framework("Spring Boot")
        .with_tool("PostgreSQL")
        .with_tool("Redis")
        .with_tool("MongoDB")
        .with_instructions(
            "You are a backend expert specializing in API development, microservices, \
             and scalable backend architectures.",
        )
        .with_max_concurrent_tasks(3)
        .with_priority(10)
    }

    /// DevOps engineer
    pub fn devops_engineer() -> DevelopmentAgent {
        DevelopmentAgent::new(
            "devops-engineer",
            AgentRole::Specialist {
                domain: "devops".to_string(),
            },
        )
        .with_language(ProgrammingLanguage::Shell)
        .with_language(ProgrammingLanguage::Python)
        .with_language(ProgrammingLanguage::Go)
        .with_domain(DevelopmentDomain::DevOps)
        .with_domain(DevelopmentDomain::Cloud)
        .with_dev_capabilities(vec![
            DevCapability::CICD,
            DevCapability::InfrastructureAsCode,
            DevCapability::ContainerOrchestration,
            DevCapability::MonitoringLogging,
        ])
        .with_capability(AgentCapability::ShellExecution)
        .with_framework("Kubernetes")
        .with_framework("Docker")
        .with_framework("Terraform")
        .with_framework("Ansible")
        .with_tool("GitHub Actions")
        .with_tool("Jenkins")
        .with_tool("Prometheus")
        .with_tool("Grafana")
        .with_instructions(
            "You are a DevOps expert specializing in CI/CD, container orchestration, \
             infrastructure as code, and cloud platforms.",
        )
        .with_max_concurrent_tasks(2)
        .with_priority(9)
    }

    /// Database specialist
    pub fn database_specialist() -> DevelopmentAgent {
        DevelopmentAgent::new(
            "db-specialist",
            AgentRole::Specialist {
                domain: "database".to_string(),
            },
        )
        .with_languages(vec![ProgrammingLanguage::SQL, ProgrammingLanguage::Python])
        .with_domain(DevelopmentDomain::Database)
        .with_dev_capabilities(vec![
            DevCapability::DatabaseDesign,
            DevCapability::PerformanceProfiling,
            DevCapability::Migration,
        ])
        .with_capability(AgentCapability::Architecture)
        .with_tool("PostgreSQL")
        .with_tool("MySQL")
        .with_tool("MongoDB")
        .with_tool("Redis")
        .with_tool("Elasticsearch")
        .with_instructions(
            "You are a database expert specializing in schema design, query optimization, \
             and database performance tuning.",
        )
        .with_max_concurrent_tasks(2)
        .with_priority(8)
    }

    /// Testing specialist
    pub fn testing_specialist() -> DevelopmentAgent {
        DevelopmentAgent::new(
            "test-specialist",
            AgentRole::Specialist {
                domain: "testing".to_string(),
            },
        )
        .with_languages(vec![
            ProgrammingLanguage::Python,
            ProgrammingLanguage::JavaScript,
            ProgrammingLanguage::TypeScript,
        ])
        .with_dev_capabilities(vec![
            DevCapability::UnitTesting,
            DevCapability::IntegrationTesting,
            DevCapability::E2ETesting,
        ])
        .with_capability(AgentCapability::Testing)
        .with_framework("pytest")
        .with_framework("Jest")
        .with_framework("Playwright")
        .with_framework("Cypress")
        .with_tool("Selenium")
        .with_instructions(
            "You are a testing expert specializing in comprehensive test strategies, \
             test automation, and quality assurance.",
        )
        .with_max_concurrent_tasks(3)
        .with_priority(8)
    }

    /// Security specialist
    pub fn security_specialist() -> DevelopmentAgent {
        DevelopmentAgent::new(
            "security-specialist",
            AgentRole::Specialist {
                domain: "security".to_string(),
            },
        )
        .with_domain(DevelopmentDomain::Security)
        .with_dev_capabilities(vec![
            DevCapability::SecurityAudit,
            DevCapability::CodeReview,
        ])
        .with_capability(AgentCapability::CodeReview)
        .with_tool("OWASP ZAP")
        .with_tool("SonarQube")
        .with_tool("Snyk")
        .with_instructions(
            "You are a security expert focusing on vulnerability detection, \
             secure coding practices, and security best practices.",
        )
        .with_max_concurrent_tasks(2)
        .with_priority(10)
    }

    /// Code reviewer
    pub fn code_reviewer() -> DevelopmentAgent {
        DevelopmentAgent::new(
            "code-reviewer",
            AgentRole::Specialist {
                domain: "code-review".to_string(),
            },
        )
        .with_dev_capabilities(vec![DevCapability::CodeReview, DevCapability::Refactoring])
        .with_capability(AgentCapability::CodeReview)
        .with_instructions(
            "You are a code review expert focusing on code quality, maintainability, \
             and best practices across multiple languages.",
        )
        .with_max_concurrent_tasks(4)
        .with_priority(8)
    }

    /// Performance expert
    pub fn performance_expert() -> DevelopmentAgent {
        DevelopmentAgent::new(
            "performance-expert",
            AgentRole::Specialist {
                domain: "performance".to_string(),
            },
        )
        .with_domain(DevelopmentDomain::Performance)
        .with_dev_capabilities(vec![
            DevCapability::PerformanceProfiling,
            DevCapability::BuildOptimization,
        ])
        .with_capability(AgentCapability::Debugging)
        .with_tool("perf")
        .with_tool("flame")
        .with_tool("Valgrind")
        .with_instructions(
            "You are a performance expert specializing in profiling, optimization, \
             and performance analysis across different platforms.",
        )
        .with_max_concurrent_tasks(2)
        .with_priority(8)
    }

    /// Documentation specialist
    pub fn documentation_specialist() -> DevelopmentAgent {
        DevelopmentAgent::new(
            "doc-specialist",
            AgentRole::Specialist {
                domain: "documentation".to_string(),
            },
        )
        .with_dev_capability(DevCapability::Documentation)
        .with_capability(AgentCapability::Documentation)
        .with_tool("Markdown")
        .with_tool("Sphinx")
        .with_tool("JSDoc")
        .with_tool("Rustdoc")
        .with_instructions(
            "You are a documentation expert specializing in clear, comprehensive \
             technical documentation and API references.",
        )
        .with_max_concurrent_tasks(3)
        .with_priority(7)
    }

    /// Get all development experts
    pub fn all_dev_experts() -> Vec<DevelopmentAgent> {
        vec![
            rust_expert(),
            frontend_expert(),
            backend_expert(),
            devops_engineer(),
            database_specialist(),
            testing_specialist(),
            security_specialist(),
            code_reviewer(),
            performance_expert(),
            documentation_specialist(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_development_agent() {
        let agent = DevelopmentAgent::new("test", AgentRole::GeneralPurpose)
            .with_language(ProgrammingLanguage::Rust)
            .with_domain(DevelopmentDomain::Backend);

        assert!(agent.has_language(&ProgrammingLanguage::Rust));
        assert!(agent.has_domain(&DevelopmentDomain::Backend));
    }

    #[test]
    fn test_rust_expert_preset() {
        let agent = presets::rust_expert();
        assert!(agent.has_language(&ProgrammingLanguage::Rust));
        assert!(agent.has_domain(&DevelopmentDomain::Backend));
        assert!(agent.has_dev_capability(&DevCapability::Implementation));
    }

    #[test]
    fn test_all_dev_experts() {
        let experts = presets::all_dev_experts();
        assert_eq!(experts.len(), 10);
    }
}
