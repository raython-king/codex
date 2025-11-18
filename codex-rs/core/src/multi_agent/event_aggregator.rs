//! Event aggregation for multi-agent systems

use super::types::AgentId;
use crate::protocol::Event;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tokio_util::sync::CancellationToken;

/// Aggregates events from multiple agents
pub struct EventAggregator {
    /// Receiver for events from all agents
    event_rx: Arc<RwLock<mpsc::UnboundedReceiver<(AgentId, Event)>>>,
    /// Sender for aggregated events to the coordinator
    aggregated_tx: mpsc::UnboundedSender<AggregatedEvent>,
    /// Cancellation token
    cancellation_token: CancellationToken,
}

/// An event with its source agent
#[derive(Debug, Clone)]
pub struct AggregatedEvent {
    /// The agent that generated this event
    pub agent_id: AgentId,
    /// The event itself
    pub event: Event,
    /// Timestamp (milliseconds since epoch)
    pub timestamp_ms: u64,
}

impl EventAggregator {
    /// Create a new event aggregator
    pub fn new(
        event_rx: mpsc::UnboundedReceiver<(AgentId, Event)>,
    ) -> (Self, mpsc::UnboundedReceiver<AggregatedEvent>) {
        let (aggregated_tx, aggregated_rx) = mpsc::unbounded_channel();
        let cancellation_token = CancellationToken::new();

        let aggregator = Self {
            event_rx: Arc::new(RwLock::new(event_rx)),
            aggregated_tx,
            cancellation_token,
        };

        (aggregator, aggregated_rx)
    }

    /// Start aggregating events
    pub async fn start(self: Arc<Self>) {
        let cancel_token = self.cancellation_token.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    event = async {
                        let mut rx = self.event_rx.write().await;
                        rx.recv().await
                    } => {
                        match event {
                            Some((agent_id, event)) => {
                                let timestamp_ms = std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap_or_default()
                                    .as_millis() as u64;

                                let aggregated = AggregatedEvent {
                                    agent_id,
                                    event,
                                    timestamp_ms,
                                };

                                if self.aggregated_tx.send(aggregated).is_err() {
                                    break;
                                }
                            }
                            None => break,
                        }
                    }
                    _ = cancel_token.cancelled() => break,
                }
            }
        });
    }

    /// Stop the aggregator
    pub fn stop(&self) {
        self.cancellation_token.cancel();
    }
}

/// Filter events by agent ID
pub struct EventFilter {
    /// Agent IDs to include (None = all)
    include_agents: Option<Vec<AgentId>>,
    /// Agent IDs to exclude
    exclude_agents: Vec<AgentId>,
}

impl EventFilter {
    /// Create a new event filter
    pub fn new() -> Self {
        Self {
            include_agents: None,
            exclude_agents: Vec::new(),
        }
    }

    /// Include only specific agents
    pub fn include_agents(mut self, agents: Vec<AgentId>) -> Self {
        self.include_agents = Some(agents);
        self
    }

    /// Exclude specific agents
    pub fn exclude_agents(mut self, agents: Vec<AgentId>) -> Self {
        self.exclude_agents = agents;
        self
    }

    /// Check if an event should be included
    pub fn should_include(&self, agent_id: &AgentId) -> bool {
        // Check exclusions first
        if self.exclude_agents.contains(agent_id) {
            return false;
        }

        // Check inclusions
        if let Some(ref include) = self.include_agents {
            include.contains(agent_id)
        } else {
            true
        }
    }
}

impl Default for EventFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_filter() {
        let filter = EventFilter::new()
            .include_agents(vec![AgentId::from("agent-1"), AgentId::from("agent-2")]);

        assert!(filter.should_include(&AgentId::from("agent-1")));
        assert!(filter.should_include(&AgentId::from("agent-2")));
        assert!(!filter.should_include(&AgentId::from("agent-3")));
    }

    #[test]
    fn test_event_filter_exclude() {
        let filter = EventFilter::new().exclude_agents(vec![AgentId::from("agent-1")]);

        assert!(!filter.should_include(&AgentId::from("agent-1")));
        assert!(filter.should_include(&AgentId::from("agent-2")));
    }
}
