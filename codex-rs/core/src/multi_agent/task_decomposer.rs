//! Task decomposition logic for breaking complex tasks into sub-tasks

use std::collections::HashMap;
use std::path::PathBuf;

use codex_protocol::user_input::UserInput;
use serde::{Deserialize, Serialize};

use super::config::TaskDecompositionStrategy;

/// Represents a decomposed sub-task that can be assigned to an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubTask {
    /// Unique identifier for this sub-task
    pub id: String,

    /// Human-readable description of the task
    pub description: String,

    /// The actual prompt/instructions for the agent
    pub prompt: String,

    /// Files that this sub-task is expected to modify
    pub target_files: Vec<PathBuf>,

    /// Dependencies on other sub-tasks (must complete before this one starts)
    pub dependencies: Vec<TaskDependency>,

    /// Priority (higher values = higher priority)
    pub priority: i32,

    /// Estimated complexity (1-10 scale)
    pub estimated_complexity: u8,

    /// Whether this task can run in parallel with others
    pub can_parallelize: bool,
}

/// Dependency relationship between tasks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDependency {
    /// ID of the task this depends on
    pub task_id: String,

    /// Type of dependency
    pub dependency_type: DependencyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DependencyType {
    /// Must complete before this task starts
    Blocking,

    /// Should complete before, but not required
    Soft,

    /// Needs output from the dependent task
    DataFlow,
}

/// Responsible for decomposing complex tasks into manageable sub-tasks
pub struct TaskDecomposer {
    strategy: TaskDecompositionStrategy,
}

impl TaskDecomposer {
    pub fn new(strategy: TaskDecompositionStrategy) -> Self {
        Self { strategy }
    }

    /// Decompose user input into a set of sub-tasks
    pub async fn decompose(
        &self,
        input: &[UserInput],
        context: &DecompositionContext,
    ) -> Result<Vec<SubTask>, DecompositionError> {
        match self.strategy {
            TaskDecompositionStrategy::Automatic => {
                self.decompose_automatic(input, context).await
            }
            TaskDecompositionStrategy::ByFile => self.decompose_by_file(input, context).await,
            TaskDecompositionStrategy::ByFunction => {
                self.decompose_by_function(input, context).await
            }
            TaskDecompositionStrategy::ByDependency => {
                self.decompose_by_dependency(input, context).await
            }
            TaskDecompositionStrategy::PlannerBased => {
                self.decompose_planner_based(input, context).await
            }
        }
    }

    /// Automatically analyze the task and determine optimal decomposition
    async fn decompose_automatic(
        &self,
        input: &[UserInput],
        context: &DecompositionContext,
    ) -> Result<Vec<SubTask>, DecompositionError> {
        // Parse the user input to understand the request
        let task_description = self.extract_task_description(input);

        // Analyze complexity and determine if decomposition is beneficial
        if !self.should_decompose(&task_description, context) {
            // Task is simple enough for a single agent
            return Ok(vec![SubTask {
                id: "main".to_string(),
                description: task_description.clone(),
                prompt: task_description,
                target_files: vec![],
                dependencies: vec![],
                priority: 0,
                estimated_complexity: 5,
                can_parallelize: false,
            }]);
        }

        // For automatic decomposition, use a hybrid approach:
        // 1. Identify major components (frontend, backend, tests, docs)
        // 2. Check for file patterns in the request
        // 3. Analyze dependencies

        let mut sub_tasks = Vec::new();

        // Example decomposition patterns
        if task_description.to_lowercase().contains("frontend")
            || task_description.to_lowercase().contains("ui")
        {
            sub_tasks.push(SubTask {
                id: "frontend".to_string(),
                description: "Implement frontend/UI components".to_string(),
                prompt: format!("Focus on frontend/UI aspects of: {}", task_description),
                target_files: context
                    .relevant_files
                    .iter()
                    .filter(|p| {
                        p.to_string_lossy().contains("ui")
                            || p.to_string_lossy().contains("frontend")
                            || p.extension().map_or(false, |e| {
                                e == "tsx" || e == "jsx" || e == "vue" || e == "svelte"
                            })
                    })
                    .cloned()
                    .collect(),
                dependencies: vec![],
                priority: 5,
                estimated_complexity: 6,
                can_parallelize: true,
            });
        }

        if task_description.to_lowercase().contains("backend")
            || task_description.to_lowercase().contains("api")
        {
            sub_tasks.push(SubTask {
                id: "backend".to_string(),
                description: "Implement backend/API logic".to_string(),
                prompt: format!("Focus on backend/API aspects of: {}", task_description),
                target_files: context
                    .relevant_files
                    .iter()
                    .filter(|p| {
                        p.to_string_lossy().contains("api")
                            || p.to_string_lossy().contains("backend")
                            || p.to_string_lossy().contains("server")
                    })
                    .cloned()
                    .collect(),
                dependencies: vec![],
                priority: 10,
                estimated_complexity: 7,
                can_parallelize: true,
            });
        }

        if task_description.to_lowercase().contains("test") || context.include_tests {
            sub_tasks.push(SubTask {
                id: "tests".to_string(),
                description: "Write tests for the implementation".to_string(),
                prompt: format!("Write comprehensive tests for: {}", task_description),
                target_files: context
                    .relevant_files
                    .iter()
                    .filter(|p| {
                        p.to_string_lossy().contains("test")
                            || p.to_string_lossy().contains("spec")
                    })
                    .cloned()
                    .collect(),
                dependencies: if sub_tasks.is_empty() {
                    vec![]
                } else {
                    // Tests depend on implementation
                    sub_tasks
                        .iter()
                        .map(|t| TaskDependency {
                            task_id: t.id.clone(),
                            dependency_type: DependencyType::Blocking,
                        })
                        .collect()
                },
                priority: 3,
                estimated_complexity: 5,
                can_parallelize: false, // Tests should run after implementation
            });
        }

        // If no specific decomposition found, create a single comprehensive task
        if sub_tasks.is_empty() {
            sub_tasks.push(SubTask {
                id: "main".to_string(),
                description: task_description.clone(),
                prompt: task_description,
                target_files: context.relevant_files.clone(),
                dependencies: vec![],
                priority: 0,
                estimated_complexity: 7,
                can_parallelize: false,
            });
        }

        Ok(sub_tasks)
    }

    /// Decompose by file or module boundaries
    async fn decompose_by_file(
        &self,
        input: &[UserInput],
        context: &DecompositionContext,
    ) -> Result<Vec<SubTask>, DecompositionError> {
        let task_description = self.extract_task_description(input);
        let mut sub_tasks = Vec::new();

        // Group files by directory or module
        let mut file_groups: HashMap<String, Vec<PathBuf>> = HashMap::new();

        for file in &context.relevant_files {
            let group_key = file
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("root")
                .to_string();

            file_groups
                .entry(group_key)
                .or_insert_with(Vec::new)
                .push(file.clone());
        }

        // Create a sub-task for each file group
        for (idx, (group, files)) in file_groups.into_iter().enumerate() {
            sub_tasks.push(SubTask {
                id: format!("file_group_{}", idx),
                description: format!("Work on {} module", group),
                prompt: format!(
                    "Apply the following task to files in {}: {}",
                    group, task_description
                ),
                target_files: files,
                dependencies: vec![],
                priority: 5,
                estimated_complexity: 5,
                can_parallelize: true,
            });
        }

        Ok(sub_tasks)
    }

    /// Decompose by functionality
    async fn decompose_by_function(
        &self,
        input: &[UserInput],
        context: &DecompositionContext,
    ) -> Result<Vec<SubTask>, DecompositionError> {
        // Similar to automatic but more explicit about functional boundaries
        self.decompose_automatic(input, context).await
    }

    /// Decompose by dependency graph
    async fn decompose_by_dependency(
        &self,
        input: &[UserInput],
        context: &DecompositionContext,
    ) -> Result<Vec<SubTask>, DecompositionError> {
        // This would analyze the codebase dependency graph
        // and create tasks that respect those dependencies
        // For now, fall back to automatic
        self.decompose_automatic(input, context).await
    }

    /// Use a planning agent to create decomposition
    async fn decompose_planner_based(
        &self,
        input: &[UserInput],
        _context: &DecompositionContext,
    ) -> Result<Vec<SubTask>, DecompositionError> {
        // This would spawn a dedicated planning agent
        // that analyzes the task and creates a decomposition plan
        // For now, return a simple decomposition
        let task_description = self.extract_task_description(input);

        Ok(vec![SubTask {
            id: "planner_task".to_string(),
            description: task_description.clone(),
            prompt: task_description,
            target_files: vec![],
            dependencies: vec![],
            priority: 0,
            estimated_complexity: 5,
            can_parallelize: false,
        }])
    }

    /// Extract task description from user input
    fn extract_task_description(&self, input: &[UserInput]) -> String {
        input
            .iter()
            .filter_map(|i| match i {
                UserInput::Text { text } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Determine if a task should be decomposed
    fn should_decompose(&self, description: &str, context: &DecompositionContext) -> bool {
        // Simple heuristics for now:
        // - Task mentions multiple components
        // - Multiple files involved
        // - Task is long and complex

        let word_count = description.split_whitespace().count();
        let file_count = context.relevant_files.len();

        let mentions_multiple = description.contains("and")
            || description.contains("also")
            || description.contains("additionally");

        word_count > 50 || file_count > 3 || mentions_multiple
    }
}

/// Context information for task decomposition
#[derive(Debug, Clone)]
pub struct DecompositionContext {
    /// Files relevant to this task
    pub relevant_files: Vec<PathBuf>,

    /// Current working directory
    pub cwd: PathBuf,

    /// Whether to include test generation
    pub include_tests: bool,

    /// Project metadata (language, framework, etc.)
    pub project_metadata: HashMap<String, String>,
}

#[derive(Debug, thiserror::Error)]
pub enum DecompositionError {
    #[error("Failed to parse task description")]
    ParseError,

    #[error("Task is too complex to decompose: {0}")]
    TooComplex(String),

    #[error("Decomposition strategy not supported: {0}")]
    UnsupportedStrategy(String),
}
