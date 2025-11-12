/// Result aggregation for coordinated tasks.
///
/// This module implements different strategies for collecting and merging
/// results from multiple agents executing parallel or broadcast tasks.

use crate::coordinator::task::TaskResult;
use std::collections::HashMap;

/// Result aggregator - collects and merges task results
pub struct ResultAggregator {
    /// Pending results indexed by task ID
    pending_results: HashMap<String, Vec<TaskResult>>,

    /// Aggregation strategy
    strategy: AggregationStrategy,
}

impl ResultAggregator {
    /// Creates a new result aggregator
    pub fn new(strategy: AggregationStrategy) -> Self {
        Self {
            pending_results: HashMap::new(),
            strategy,
        }
    }

    /// Creates an aggregator with "All" strategy (default)
    pub fn new_all() -> Self {
        Self::new(AggregationStrategy::All)
    }

    /// Adds a result for a task
    pub fn add_result(&mut self, task_id: &str, result: TaskResult) {
        self.pending_results
            .entry(task_id.to_string())
            .or_insert_with(Vec::new)
            .push(result);
    }

    /// Checks if enough results have been collected for a task
    pub fn is_complete(&self, task_id: &str, expected_count: usize) -> bool {
        match &self.strategy {
            AggregationStrategy::All => self
                .pending_results
                .get(task_id)
                .map(|results| results.len() >= expected_count)
                .unwrap_or(false),

            AggregationStrategy::Any => self
                .pending_results
                .get(task_id)
                .map(|results| !results.is_empty())
                .unwrap_or(false),

            AggregationStrategy::AtLeast(n) => self
                .pending_results
                .get(task_id)
                .map(|results| results.len() >= *n)
                .unwrap_or(false),

            AggregationStrategy::Majority => {
                let total = expected_count;
                self.pending_results
                    .get(task_id)
                    .map(|results| results.len() > total / 2)
                    .unwrap_or(false)
            }

            AggregationStrategy::Custom(_) => {
                // For custom strategies, we check if we have at least one result
                self.pending_results
                    .get(task_id)
                    .map(|results| !results.is_empty())
                    .unwrap_or(false)
            }
        }
    }

    /// Gets the aggregated result for a task
    pub fn get_aggregated_result(&self, task_id: &str) -> Option<serde_json::Value> {
        let results = self.pending_results.get(task_id)?;
        if results.is_empty() {
            return None;
        }

        match &self.strategy {
            AggregationStrategy::All => {
                // Merge all results into an array
                let merged = results
                    .iter()
                    .map(|r| r.result.clone())
                    .collect::<Vec<_>>();
                Some(serde_json::json!({
                    "results": merged,
                    "count": merged.len(),
                }))
            }

            AggregationStrategy::Any => {
                // Return the first result
                results.first().map(|r| r.result.clone())
            }

            AggregationStrategy::AtLeast(n) => {
                if results.len() >= *n {
                    let merged = results
                        .iter()
                        .take(*n)
                        .map(|r| r.result.clone())
                        .collect::<Vec<_>>();
                    Some(serde_json::json!({
                        "results": merged,
                        "count": merged.len(),
                    }))
                } else {
                    None
                }
            }

            AggregationStrategy::Majority => {
                // Use majority voting on results
                Self::majority_vote(results)
            }

            AggregationStrategy::Custom(func) => {
                // Use custom aggregation function
                Some(func(results.clone()))
            }
        }
    }

    /// Performs majority voting on results
    fn majority_vote(results: &[TaskResult]) -> Option<serde_json::Value> {
        if results.is_empty() {
            return None;
        }

        // Count occurrences of each result
        let mut counts: HashMap<String, usize> = HashMap::new();
        for result in results {
            let key = result.result.to_string();
            *counts.entry(key).or_insert(0) += 1;
        }

        // Find the most common result
        let max_count = counts.values().max()?;
        let winner = counts
            .iter()
            .find(|(_, count)| *count == max_count)?
            .0;

        // Parse back to JSON
        serde_json::from_str(winner).ok()
    }

    /// Clears results for a completed task
    pub fn cleanup(&mut self, task_id: &str) {
        self.pending_results.remove(task_id);
    }

    /// Gets the number of pending results for a task
    pub fn pending_count(&self, task_id: &str) -> usize {
        self.pending_results
            .get(task_id)
            .map(|r| r.len())
            .unwrap_or(0)
    }

    /// Clears all pending results
    pub fn clear_all(&mut self) {
        self.pending_results.clear();
    }
}

/// Aggregation strategy - determines how to combine multiple results
pub enum AggregationStrategy {
    /// Wait for all expected results
    All,

    /// Return as soon as any result is available
    Any,

    /// Wait for at least N results
    AtLeast(usize),

    /// Use majority voting (most common result wins)
    Majority,

    /// Custom aggregation function
    Custom(Box<dyn Fn(Vec<TaskResult>) -> serde_json::Value + Send + Sync>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::AgentId;
    use serde_json::json;

    fn create_test_result(task_id: &str, agent_id: &str, value: i32) -> TaskResult {
        TaskResult::new(
            task_id.to_string(),
            AgentId::new(agent_id),
            json!({"value": value}),
            100,
        )
    }

    #[test]
    fn test_aggregator_all_strategy() {
        let mut aggregator = ResultAggregator::new_all();

        aggregator.add_result("task-1", create_test_result("task-1", "agent-a", 1));
        aggregator.add_result("task-1", create_test_result("task-1", "agent-b", 2));
        aggregator.add_result("task-1", create_test_result("task-1", "agent-c", 3));

        assert!(aggregator.is_complete("task-1", 3));
        assert!(!aggregator.is_complete("task-1", 4));

        let result = aggregator.get_aggregated_result("task-1").unwrap();
        assert!(result.is_object());
        assert_eq!(result["count"], 3);
    }

    #[test]
    fn test_aggregator_any_strategy() {
        let mut aggregator = ResultAggregator::new(AggregationStrategy::Any);

        aggregator.add_result("task-1", create_test_result("task-1", "agent-a", 1));

        // Should be complete with just one result
        assert!(aggregator.is_complete("task-1", 3));

        let result = aggregator.get_aggregated_result("task-1").unwrap();
        assert_eq!(result["value"], 1);
    }

    #[test]
    fn test_aggregator_at_least_strategy() {
        let mut aggregator = ResultAggregator::new(AggregationStrategy::AtLeast(2));

        aggregator.add_result("task-1", create_test_result("task-1", "agent-a", 1));

        // Not complete with 1 result
        assert!(!aggregator.is_complete("task-1", 3));

        aggregator.add_result("task-1", create_test_result("task-1", "agent-b", 2));

        // Complete with 2 results
        assert!(aggregator.is_complete("task-1", 3));

        let result = aggregator.get_aggregated_result("task-1").unwrap();
        assert_eq!(result["count"], 2);
    }

    #[test]
    fn test_aggregator_majority_strategy() {
        let mut aggregator = ResultAggregator::new(AggregationStrategy::Majority);

        aggregator.add_result("task-1", create_test_result("task-1", "agent-a", 1));
        aggregator.add_result("task-1", create_test_result("task-1", "agent-b", 1));
        aggregator.add_result("task-1", create_test_result("task-1", "agent-c", 2));

        // Complete when we have majority (> 1.5 out of 3)
        assert!(aggregator.is_complete("task-1", 3));

        let result = aggregator.get_aggregated_result("task-1").unwrap();
        // Majority voted for value 1
        assert_eq!(result["value"], 1);
    }

    #[test]
    fn test_aggregator_custom_strategy() {
        let custom_func = |results: Vec<TaskResult>| {
            let sum: i64 = results
                .iter()
                .filter_map(|r| r.result["value"].as_i64())
                .sum();
            json!({"sum": sum, "count": results.len()})
        };

        let mut aggregator = ResultAggregator::new(AggregationStrategy::Custom(Box::new(custom_func)));

        aggregator.add_result("task-1", create_test_result("task-1", "agent-a", 10));
        aggregator.add_result("task-1", create_test_result("task-1", "agent-b", 20));
        aggregator.add_result("task-1", create_test_result("task-1", "agent-c", 30));

        let result = aggregator.get_aggregated_result("task-1").unwrap();
        assert_eq!(result["sum"], 60);
        assert_eq!(result["count"], 3);
    }

    #[test]
    fn test_aggregator_cleanup() {
        let mut aggregator = ResultAggregator::new_all();

        aggregator.add_result("task-1", create_test_result("task-1", "agent-a", 1));
        aggregator.add_result("task-2", create_test_result("task-2", "agent-b", 2));

        assert_eq!(aggregator.pending_count("task-1"), 1);
        assert_eq!(aggregator.pending_count("task-2"), 1);

        aggregator.cleanup("task-1");

        assert_eq!(aggregator.pending_count("task-1"), 0);
        assert_eq!(aggregator.pending_count("task-2"), 1);
    }

    #[test]
    fn test_aggregator_clear_all() {
        let mut aggregator = ResultAggregator::new_all();

        aggregator.add_result("task-1", create_test_result("task-1", "agent-a", 1));
        aggregator.add_result("task-2", create_test_result("task-2", "agent-b", 2));

        aggregator.clear_all();

        assert_eq!(aggregator.pending_count("task-1"), 0);
        assert_eq!(aggregator.pending_count("task-2"), 0);
    }
}
