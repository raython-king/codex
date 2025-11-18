//! Tests for multi-agent collaborative system

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;

    use codex_protocol::user_input::UserInput;

    use super::super::config::{
        ConflictResolution, MultiAgentConfig, TaskDecompositionStrategy,
    };
    use super::super::task_decomposer::{DecompositionContext, TaskDecomposer};

    #[tokio::test]
    async fn test_task_decomposition_simple() {
        let decomposer = TaskDecomposer::new(TaskDecompositionStrategy::Automatic);

        let input = vec![UserInput::Text("Implement a simple function".to_string())];

        let context = DecompositionContext {
            relevant_files: vec![],
            cwd: PathBuf::from("/tmp"),
            include_tests: false,
            project_metadata: HashMap::new(),
        };

        let result = decomposer.decompose(&input, &context).await;
        assert!(result.is_ok());

        let tasks = result.unwrap();
        // Simple tasks should not be decomposed
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, "main");
    }

    #[tokio::test]
    async fn test_task_decomposition_complex() {
        let decomposer = TaskDecomposer::new(TaskDecompositionStrategy::Automatic);

        let input = vec![UserInput::Text(
            "Implement a new feature with frontend UI components and backend API endpoints, \
             and also write comprehensive tests for all functionality"
                .to_string(),
        )];

        let context = DecompositionContext {
            relevant_files: vec![
                PathBuf::from("src/frontend/components.tsx"),
                PathBuf::from("src/backend/api.rs"),
                PathBuf::from("tests/integration_test.rs"),
            ],
            cwd: PathBuf::from("/tmp"),
            include_tests: true,
            project_metadata: HashMap::new(),
        };

        let result = decomposer.decompose(&input, &context).await;
        assert!(result.is_ok());

        let tasks = result.unwrap();
        // Complex task should be decomposed
        assert!(tasks.len() > 1);

        // Should have frontend, backend, and tests
        let task_ids: Vec<&str> = tasks.iter().map(|t| t.id.as_str()).collect();
        assert!(task_ids.contains(&"frontend"));
        assert!(task_ids.contains(&"backend"));
        assert!(task_ids.contains(&"tests"));

        // Tests should depend on implementation
        let test_task = tasks.iter().find(|t| t.id == "tests").unwrap();
        assert!(!test_task.dependencies.is_empty());
    }

    #[tokio::test]
    async fn test_task_decomposition_by_file() {
        let decomposer = TaskDecomposer::new(TaskDecompositionStrategy::ByFile);

        let input = vec![UserInput::Text(
            "Refactor the authentication module".to_string(),
        )];

        let context = DecompositionContext {
            relevant_files: vec![
                PathBuf::from("src/auth/login.rs"),
                PathBuf::from("src/auth/signup.rs"),
                PathBuf::from("src/api/routes.rs"),
            ],
            cwd: PathBuf::from("/tmp"),
            include_tests: false,
            project_metadata: HashMap::new(),
        };

        let result = decomposer.decompose(&input, &context).await;
        assert!(result.is_ok());

        let tasks = result.unwrap();
        // Should create tasks grouped by directory
        assert!(tasks.len() >= 2); // auth and api groups
    }

    #[test]
    fn test_multi_agent_config_default() {
        let config = MultiAgentConfig::default();
        assert_eq!(config.max_concurrent_agents, 3);
        assert_eq!(
            config.task_decomposition_strategy,
            TaskDecompositionStrategy::Automatic
        );
        assert_eq!(
            config.conflict_resolution,
            ConflictResolution::MergeWithReview
        );
        assert!(config.enable_agent_communication);
        assert!(config.enable_shared_context);
    }

    #[test]
    fn test_conflict_resolution_strategies() {
        // Test different conflict resolution strategies
        let strategies = vec![
            ConflictResolution::MergeWithReview,
            ConflictResolution::MergeAgent,
            ConflictResolution::Sequential,
            ConflictResolution::FailOnConflict,
            ConflictResolution::LastWriteWins,
        ];

        for strategy in strategies {
            let config = MultiAgentConfig {
                conflict_resolution: strategy,
                ..Default::default()
            };
            assert_eq!(config.conflict_resolution, strategy);
        }
    }
}
