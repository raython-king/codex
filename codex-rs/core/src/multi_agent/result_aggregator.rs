//! Result aggregation and conflict resolution for multi-agent results

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::config::ConflictResolution;

/// Result from a single agent's execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResult {
    /// ID of the agent that produced this result
    pub agent_id: String,

    /// ID of the sub-task this result corresponds to
    pub task_id: String,

    /// Whether the task completed successfully
    pub success: bool,

    /// Final message from the agent
    pub message: Option<String>,

    /// Files modified by this agent
    pub modified_files: Vec<FileChange>,

    /// Any errors encountered
    pub errors: Vec<String>,

    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

/// Represents a file change made by an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    /// Path to the file
    pub path: PathBuf,

    /// Type of change
    pub change_type: ChangeType,

    /// Content hash before change (if applicable)
    pub before_hash: Option<String>,

    /// Content hash after change
    pub after_hash: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeType {
    Created,
    Modified,
    Deleted,
}

/// Aggregates results from multiple agents and resolves conflicts
pub struct ResultAggregator {
    resolution_strategy: ConflictResolution,
}

impl ResultAggregator {
    pub fn new(resolution_strategy: ConflictResolution) -> Self {
        Self {
            resolution_strategy,
        }
    }

    /// Aggregate results from multiple agents
    pub async fn aggregate(
        &self,
        results: Vec<AgentResult>,
    ) -> Result<AggregatedResult, AggregationError> {
        // Detect conflicts
        let conflicts = self.detect_conflicts(&results);

        // If there are conflicts, apply resolution strategy
        let resolved_changes = if conflicts.is_empty() {
            // No conflicts, merge all changes
            self.merge_all_changes(&results)
        } else {
            // Apply conflict resolution
            self.resolve_conflicts(&results, &conflicts).await?
        };

        // Aggregate messages and errors
        let messages: Vec<String> = results
            .iter()
            .filter_map(|r| r.message.clone())
            .collect();

        let all_errors: Vec<String> = results
            .iter()
            .flat_map(|r| r.errors.clone())
            .collect();

        let total_time_ms: u64 = results.iter().map(|r| r.execution_time_ms).sum();

        let success = results.iter().all(|r| r.success) && all_errors.is_empty();

        Ok(AggregatedResult {
            success,
            final_changes: resolved_changes,
            messages,
            errors: all_errors,
            total_execution_time_ms: total_time_ms,
            conflict_count: conflicts.len(),
            agent_count: results.len(),
        })
    }

    /// Detect conflicts between agent results
    fn detect_conflicts(&self, results: &[AgentResult]) -> Vec<FileConflict> {
        let mut file_changes: HashMap<PathBuf, Vec<(usize, &FileChange)>> = HashMap::new();

        // Group changes by file
        for (idx, result) in results.iter().enumerate() {
            for change in &result.modified_files {
                file_changes
                    .entry(change.path.clone())
                    .or_insert_with(Vec::new)
                    .push((idx, change));
            }
        }

        // Find files modified by multiple agents
        let mut conflicts = Vec::new();
        for (path, changes) in file_changes {
            if changes.len() > 1 {
                // Multiple agents modified the same file
                let agent_ids: Vec<String> = changes
                    .iter()
                    .map(|(idx, _)| results[*idx].agent_id.clone())
                    .collect();

                conflicts.push(FileConflict {
                    path: path.clone(),
                    conflicting_agents: agent_ids,
                    changes: changes.into_iter().map(|(_, c)| c.clone()).collect(),
                });
            }
        }

        conflicts
    }

    /// Merge all changes when there are no conflicts
    fn merge_all_changes(&self, results: &[AgentResult]) -> Vec<FileChange> {
        results
            .iter()
            .flat_map(|r| r.modified_files.clone())
            .collect()
    }

    /// Resolve conflicts according to the configured strategy
    async fn resolve_conflicts(
        &self,
        results: &[AgentResult],
        conflicts: &[FileConflict],
    ) -> Result<Vec<FileChange>, AggregationError> {
        match self.resolution_strategy {
            ConflictResolution::MergeWithReview => {
                self.merge_with_review(results, conflicts).await
            }
            ConflictResolution::MergeAgent => self.merge_agent(results, conflicts).await,
            ConflictResolution::Sequential => {
                // Should not happen if tasks were executed sequentially
                Err(AggregationError::UnexpectedConflict)
            }
            ConflictResolution::FailOnConflict => {
                Err(AggregationError::ConflictDetected(conflicts.len()))
            }
            ConflictResolution::LastWriteWins => self.last_write_wins(results, conflicts),
        }
    }

    /// Merge results and mark conflicts for review
    async fn merge_with_review(
        &self,
        results: &[AgentResult],
        conflicts: &[FileConflict],
    ) -> Result<Vec<FileChange>, AggregationError> {
        // For now, collect all non-conflicting changes
        // and mark conflicting files for manual review
        let mut final_changes = Vec::new();

        let conflicting_paths: Vec<&PathBuf> = conflicts.iter().map(|c| &c.path).collect();

        for result in results {
            for change in &result.modified_files {
                if !conflicting_paths.contains(&&change.path) {
                    final_changes.push(change.clone());
                }
            }
        }

        // TODO: Create review tasks for conflicts
        // For now, just log them
        tracing::warn!(
            "Detected {} file conflicts that require review",
            conflicts.len()
        );

        Ok(final_changes)
    }

    /// Use a dedicated merge agent to resolve conflicts
    async fn merge_agent(
        &self,
        _results: &[AgentResult],
        _conflicts: &[FileConflict],
    ) -> Result<Vec<FileChange>, AggregationError> {
        // TODO: Spawn a dedicated merge agent
        // that analyzes conflicts and produces a merged result
        Err(AggregationError::NotImplemented(
            "Merge agent not yet implemented".to_string(),
        ))
    }

    /// Last write wins (simple but potentially dangerous)
    fn last_write_wins(
        &self,
        results: &[AgentResult],
        _conflicts: &[FileConflict],
    ) -> Result<Vec<FileChange>, AggregationError> {
        // Just take the last change for each file
        let mut file_map: HashMap<PathBuf, FileChange> = HashMap::new();

        for result in results {
            for change in &result.modified_files {
                file_map.insert(change.path.clone(), change.clone());
            }
        }

        Ok(file_map.into_values().collect())
    }
}

/// Detected conflict between agent results
#[derive(Debug, Clone)]
pub struct FileConflict {
    /// Path to the conflicting file
    pub path: PathBuf,

    /// IDs of agents that modified this file
    pub conflicting_agents: Vec<String>,

    /// The conflicting changes
    pub changes: Vec<FileChange>,
}

/// Final aggregated result from all agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedResult {
    /// Whether all agents succeeded
    pub success: bool,

    /// Final set of file changes after conflict resolution
    pub final_changes: Vec<FileChange>,

    /// Messages from all agents
    pub messages: Vec<String>,

    /// All errors encountered
    pub errors: Vec<String>,

    /// Total execution time across all agents
    pub total_execution_time_ms: u64,

    /// Number of conflicts detected
    pub conflict_count: usize,

    /// Number of agents involved
    pub agent_count: usize,
}

#[derive(Debug, thiserror::Error)]
pub enum AggregationError {
    #[error("Conflicts detected: {0} files have conflicting changes")]
    ConflictDetected(usize),

    #[error("Unexpected conflict in sequential execution mode")]
    UnexpectedConflict,

    #[error("Feature not implemented: {0}")]
    NotImplemented(String),

    #[error("Failed to merge results: {0}")]
    MergeError(String),
}
