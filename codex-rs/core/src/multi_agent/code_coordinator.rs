//! Software development coordinator
//!
//! This coordinator specializes in software development workflows,
//! routing tasks to appropriate development experts based on language,
//! domain, and task type.

use super::agent_pool::{AgentPool, AgentPoolConfig};
use super::code_specialist::{
    DevCapability, DevelopmentAgent, DevelopmentDomain, ProgrammingLanguage,
};
use super::task_distributor::{TaskDistributionStrategy, TaskDistributor};
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

/// Configuration for code development coordinator
#[derive(Debug, Clone)]
pub struct CodeCoordinatorConfig {
    /// Agent pool configuration
    pub pool_config: AgentPoolConfig,
    /// Task distribution strategy
    pub distribution_strategy: TaskDistributionStrategy,
    /// Development experts to include
    pub dev_experts: Vec<DevelopmentAgent>,
    /// Enable automatic language/domain inference
    pub auto_infer: bool,
    /// Enable verbose logging
    pub verbose: bool,
}

impl Default for CodeCoordinatorConfig {
    fn default() -> Self {
        Self {
            pool_config: AgentPoolConfig {
                max_agents: 15,
                max_concurrent_tasks: 30,
                enable_event_aggregation: true,
            },
            distribution_strategy: TaskDistributionStrategy::BestMatch,
            dev_experts: Vec::new(),
            auto_infer: true,
            verbose: false,
        }
    }
}

/// Specialized coordinator for software development tasks
pub struct CodeCoordinator {
    /// Configuration
    config: CodeCoordinatorConfig,
    /// Agent pool
    agent_pool: Arc<RwLock<Option<Arc<AgentPool>>>>,
    /// Task distributor
    task_distributor: Arc<RwLock<Option<Arc<TaskDistributor>>>>,
    /// Next task ID
    next_task_id: Arc<RwLock<usize>>,
}

impl CodeCoordinator {
    /// Create a new code coordinator
    pub fn new(config: CodeCoordinatorConfig) -> Self {
        Self {
            config,
            agent_pool: Arc::new(RwLock::new(None)),
            task_distributor: Arc::new(RwLock::new(None)),
            next_task_id: Arc::new(RwLock::new(0)),
        }
    }

    /// Initialize the agent pool with development experts
    async fn initialize_agents(&self) -> Result<Arc<AgentPool>, CodexErr> {
        let pool = Arc::new(AgentPool::new(self.config.pool_config.clone()));

        // Add all development experts
        for expert in &self.config.dev_experts {
            pool.add_agent(expert.clone().into_spec()).await?;
        }

        Ok(pool)
    }

    /// Infer programming languages from task description
    fn infer_languages(&self, description: &str) -> HashSet<ProgrammingLanguage> {
        let mut languages = HashSet::new();
        let desc_lower = description.to_lowercase();

        // Language detection patterns
        if desc_lower.contains("rust") || desc_lower.contains(".rs") {
            languages.insert(ProgrammingLanguage::Rust);
        }

        if desc_lower.contains("python") || desc_lower.contains(".py") {
            languages.insert(ProgrammingLanguage::Python);
        }

        if desc_lower.contains("javascript") || desc_lower.contains(".js") {
            languages.insert(ProgrammingLanguage::JavaScript);
        }

        if desc_lower.contains("typescript") || desc_lower.contains(".ts") {
            languages.insert(ProgrammingLanguage::TypeScript);
        }

        if desc_lower.contains("golang") || desc_lower.contains("go") || desc_lower.contains(".go")
        {
            languages.insert(ProgrammingLanguage::Go);
        }

        if desc_lower.contains("java") && !desc_lower.contains("javascript") {
            languages.insert(ProgrammingLanguage::Java);
        }

        if desc_lower.contains("c++") || desc_lower.contains("cpp") {
            languages.insert(ProgrammingLanguage::Cpp);
        }

        if desc_lower.contains("c#") || desc_lower.contains("csharp") {
            languages.insert(ProgrammingLanguage::CSharp);
        }

        if desc_lower.contains("ruby") || desc_lower.contains(".rb") {
            languages.insert(ProgrammingLanguage::Ruby);
        }

        if desc_lower.contains("php") {
            languages.insert(ProgrammingLanguage::PHP);
        }

        if desc_lower.contains("swift") {
            languages.insert(ProgrammingLanguage::Swift);
        }

        if desc_lower.contains("kotlin") {
            languages.insert(ProgrammingLanguage::Kotlin);
        }

        if desc_lower.contains("sql") {
            languages.insert(ProgrammingLanguage::SQL);
        }

        if desc_lower.contains("shell") || desc_lower.contains("bash") || desc_lower.contains(".sh")
        {
            languages.insert(ProgrammingLanguage::Shell);
        }

        languages
    }

    /// Infer development domains from task description
    fn infer_domains(&self, description: &str) -> HashSet<DevelopmentDomain> {
        let mut domains = HashSet::new();
        let desc_lower = description.to_lowercase();

        if desc_lower.contains("frontend")
            || desc_lower.contains("ui")
            || desc_lower.contains("react")
            || desc_lower.contains("vue")
            || desc_lower.contains("angular")
        {
            domains.insert(DevelopmentDomain::Frontend);
        }

        if desc_lower.contains("backend")
            || desc_lower.contains("api")
            || desc_lower.contains("server")
            || desc_lower.contains("microservice")
        {
            domains.insert(DevelopmentDomain::Backend);
        }

        if desc_lower.contains("devops")
            || desc_lower.contains("ci/cd")
            || desc_lower.contains("kubernetes")
            || desc_lower.contains("docker")
            || desc_lower.contains("terraform")
        {
            domains.insert(DevelopmentDomain::DevOps);
        }

        if desc_lower.contains("database")
            || desc_lower.contains("sql")
            || desc_lower.contains("postgres")
            || desc_lower.contains("mongodb")
        {
            domains.insert(DevelopmentDomain::Database);
        }

        if desc_lower.contains("cloud")
            || desc_lower.contains("aws")
            || desc_lower.contains("azure")
            || desc_lower.contains("gcp")
        {
            domains.insert(DevelopmentDomain::Cloud);
        }

        if desc_lower.contains("security")
            || desc_lower.contains("authentication")
            || desc_lower.contains("encryption")
        {
            domains.insert(DevelopmentDomain::Security);
        }

        if desc_lower.contains("performance")
            || desc_lower.contains("optimization")
            || desc_lower.contains("profiling")
        {
            domains.insert(DevelopmentDomain::Performance);
        }

        if desc_lower.contains("mobile")
            || desc_lower.contains("ios")
            || desc_lower.contains("android")
        {
            domains.insert(DevelopmentDomain::Mobile);
        }

        domains
    }

    /// Infer development capabilities from task description
    fn infer_dev_capabilities(&self, description: &str) -> HashSet<DevCapability> {
        let mut capabilities = HashSet::new();
        let desc_lower = description.to_lowercase();

        if desc_lower.contains("implement")
            || desc_lower.contains("build")
            || desc_lower.contains("create")
            || desc_lower.contains("develop")
        {
            capabilities.insert(DevCapability::Implementation);
        }

        if desc_lower.contains("refactor") || desc_lower.contains("improve") {
            capabilities.insert(DevCapability::Refactoring);
        }

        if desc_lower.contains("bug") || desc_lower.contains("fix") || desc_lower.contains("debug")
        {
            capabilities.insert(DevCapability::BugFixing);
        }

        if desc_lower.contains("review") || desc_lower.contains("audit") {
            capabilities.insert(DevCapability::CodeReview);
        }

        if desc_lower.contains("test") {
            capabilities.insert(DevCapability::UnitTesting);
            capabilities.insert(DevCapability::IntegrationTesting);
        }

        if desc_lower.contains("api") {
            capabilities.insert(DevCapability::APIDesign);
        }

        if desc_lower.contains("database") || desc_lower.contains("schema") {
            capabilities.insert(DevCapability::DatabaseDesign);
        }

        if desc_lower.contains("architecture") || desc_lower.contains("design") {
            capabilities.insert(DevCapability::SystemArchitecture);
        }

        if desc_lower.contains("ci/cd") || desc_lower.contains("pipeline") {
            capabilities.insert(DevCapability::CICD);
        }

        if desc_lower.contains("docker") || desc_lower.contains("kubernetes") {
            capabilities.insert(DevCapability::ContainerOrchestration);
        }

        if desc_lower.contains("document") {
            capabilities.insert(DevCapability::Documentation);
        }

        if desc_lower.contains("migrate") || desc_lower.contains("migration") {
            capabilities.insert(DevCapability::Migration);
        }

        capabilities
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

        if desc_lower.contains("review") {
            capabilities.insert(AgentCapability::CodeReview);
        }

        if desc_lower.contains("test") {
            capabilities.insert(AgentCapability::Testing);
        }

        if desc_lower.contains("debug") || desc_lower.contains("fix") {
            capabilities.insert(AgentCapability::Debugging);
        }

        if desc_lower.contains("architecture") || desc_lower.contains("design") {
            capabilities.insert(AgentCapability::Architecture);
        }

        if desc_lower.contains("document") {
            capabilities.insert(AgentCapability::Documentation);
        }

        // Default to code generation if nothing specific
        if capabilities.is_empty() {
            capabilities.insert(AgentCapability::CodeGeneration);
        }

        capabilities
    }

    /// Plan task delegation for development tasks
    async fn plan_dev_delegation(
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
            let id = format!("dev-task-{next_id}");
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
        languages: &HashSet<ProgrammingLanguage>,
        domains: &HashSet<DevelopmentDomain>,
        dev_caps: &HashSet<DevCapability>,
    ) -> String {
        let mut report = String::from("Software Development Task Plan\n");
        report.push_str(&"=".repeat(50));
        report.push('\n');

        // Languages detected
        if !languages.is_empty() {
            report.push_str("\nDetected Languages:\n");
            for lang in languages {
                report.push_str(&format!("  • {}\n", lang));
            }
        }

        // Domains detected
        if !domains.is_empty() {
            report.push_str("\nDetected Domains:\n");
            for domain in domains {
                report.push_str(&format!("  • {:?}\n", domain));
            }
        }

        // Development capabilities
        if !dev_caps.is_empty() {
            report.push_str("\nRequired Development Capabilities:\n");
            for cap in dev_caps {
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
impl SessionTask for CodeCoordinator {
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
                return Some(format!("Error initializing development agents: {e}"));
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
        let languages = if self.config.auto_infer {
            self.infer_languages(&message)
        } else {
            HashSet::new()
        };

        let domains = if self.config.auto_infer {
            self.infer_domains(&message)
        } else {
            HashSet::new()
        };

        let dev_caps = if self.config.auto_infer {
            self.infer_dev_capabilities(&message)
        } else {
            HashSet::new()
        };

        // Plan task delegation
        let tasks = match self.plan_dev_delegation(&input).await {
            Ok(t) => t,
            Err(e) => {
                return Some(format!("Error planning task delegation: {e}"));
            }
        };

        let elapsed = start_time.elapsed();

        // Generate execution plan
        let mut report = self.generate_plan_summary(&tasks, &languages, &domains, &dev_caps);
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
    use crate::multi_agent::code_specialist::presets;

    #[test]
    fn test_infer_rust() {
        let config = CodeCoordinatorConfig::default();
        let coordinator = CodeCoordinator::new(config);

        let langs = coordinator.infer_languages("Implement a Rust async server");
        assert!(langs.contains(&ProgrammingLanguage::Rust));
    }

    #[test]
    fn test_infer_backend() {
        let config = CodeCoordinatorConfig::default();
        let coordinator = CodeCoordinator::new(config);

        let domains = coordinator.infer_domains("Build a REST API backend");
        assert!(domains.contains(&DevelopmentDomain::Backend));
    }

    #[test]
    fn test_infer_dev_capabilities() {
        let config = CodeCoordinatorConfig::default();
        let coordinator = CodeCoordinator::new(config);

        let caps = coordinator.infer_dev_capabilities("Implement and test API endpoints");
        assert!(caps.contains(&DevCapability::Implementation));
        assert!(caps.contains(&DevCapability::UnitTesting));
    }

    #[tokio::test]
    async fn test_coordinator_initialization() {
        let config = CodeCoordinatorConfig {
            dev_experts: vec![presets::rust_expert()],
            ..Default::default()
        };

        let coordinator = CodeCoordinator::new(config);
        let pool = coordinator.initialize_agents().await.unwrap();
        let stats = pool.get_stats().await;

        assert_eq!(stats.total_agents, 1);
    }
}
