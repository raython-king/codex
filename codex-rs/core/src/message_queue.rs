/// Agent-to-agent message queue implementation.

use codex_protocol::{AgentId, QueuedMessage};
use std::collections::{BinaryHeap, HashMap};

/// Priority queue wrapper for message ordering.
#[derive(Debug)]
struct PriorityMessage {
    message: QueuedMessage,
    sequence: u64,
}

impl PartialEq for PriorityMessage {
    fn eq(&self, other: &Self) -> bool {
        self.message.priority == other.message.priority && self.sequence == other.sequence
    }
}

impl Eq for PriorityMessage {}

impl PartialOrd for PriorityMessage {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PriorityMessage {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Higher priority first, then FIFO within same priority
        // Note: BinaryHeap is a max-heap, so we compare self.priority > other.priority
        // to get higher priorities popped first
        match self.message.priority.cmp(&other.message.priority) {
            std::cmp::Ordering::Equal => other.sequence.cmp(&self.sequence),
            ordering => ordering,
        }
    }
}

/// Message queue for a specific agent.
#[derive(Debug)]
struct AgentQueue {
    messages: BinaryHeap<PriorityMessage>,
    sequence_counter: u64,
}

impl AgentQueue {
    fn new() -> Self {
        Self {
            messages: BinaryHeap::new(),
            sequence_counter: 0,
        }
    }

    fn push(&mut self, message: QueuedMessage) {
        self.sequence_counter += 1;
        self.messages.push(PriorityMessage {
            message,
            sequence: self.sequence_counter,
        });
    }

    fn pop(&mut self) -> Option<QueuedMessage> {
        self.messages.pop().map(|pm| pm.message)
    }

    fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    fn len(&self) -> usize {
        self.messages.len()
    }
}

/// Central message queue for all agents in a session.
#[derive(Debug)]
pub struct MessageQueue {
    /// Per-agent message queues
    queues: HashMap<AgentId, AgentQueue>,

    /// Pending responses (message_id -> waiting agent)
    pending_responses: HashMap<String, AgentId>,
}

impl MessageQueue {
    pub fn new() -> Self {
        Self {
            queues: HashMap::new(),
            pending_responses: HashMap::new(),
        }
    }

    /// Enqueues a message for the target agent.
    pub fn enqueue(&mut self, message: QueuedMessage) {
        let target = message.message.to.clone();

        // Track pending responses
        if message.requires_response {
            self.pending_responses.insert(
                message.message.id.clone(),
                message.message.from.clone(),
            );
        }

        // Add to target agent's queue
        self.queues
            .entry(target)
            .or_insert_with(AgentQueue::new)
            .push(message);
    }

    /// Dequeues the next message for the specified agent.
    pub fn dequeue(&mut self, agent_id: &AgentId) -> Option<QueuedMessage> {
        self.queues
            .get_mut(agent_id)
            .and_then(|queue| queue.pop())
    }

    /// Returns the number of pending messages for an agent.
    pub fn pending_count(&self, agent_id: &AgentId) -> usize {
        self.queues
            .get(agent_id)
            .map(|q| q.len())
            .unwrap_or(0)
    }

    /// Checks if there are any pending messages for an agent.
    pub fn has_messages(&self, agent_id: &AgentId) -> bool {
        self.queues
            .get(agent_id)
            .map(|q| !q.is_empty())
            .unwrap_or(false)
    }

    /// Clears all messages for a specific agent (used when agent is unregistered).
    pub fn clear_agent_messages(&mut self, agent_id: &AgentId) {
        self.queues.remove(agent_id);

        // Remove from pending responses
        self.pending_responses.retain(|_, from| from != agent_id);
    }
}

impl Default for MessageQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex_protocol::{AgentMessage, MessagePriority, MessageType};
    use serde_json::json;

    fn create_test_message(from: &str, to: &str, priority: MessagePriority) -> QueuedMessage {
        let msg = AgentMessage::new(
            AgentId::new(from),
            AgentId::new(to),
            MessageType::Request,
            json!({}),
        );
        QueuedMessage::new(msg).with_priority(priority)
    }

    #[test]
    fn test_message_queue_enqueue_dequeue() {
        let mut queue = MessageQueue::new();
        let msg = create_test_message("agent-a", "agent-b", MessagePriority::Normal);

        queue.enqueue(msg);

        assert_eq!(queue.pending_count(&AgentId::new("agent-b")), 1);

        let dequeued = queue.dequeue(&AgentId::new("agent-b")).unwrap();
        assert_eq!(dequeued.message.from, AgentId::new("agent-a"));
    }

    #[test]
    fn test_priority_ordering() {
        let mut queue = MessageQueue::new();

        queue.enqueue(create_test_message("a", "target", MessagePriority::Low));
        queue.enqueue(create_test_message("b", "target", MessagePriority::High));
        queue.enqueue(create_test_message("c", "target", MessagePriority::Normal));
        queue.enqueue(create_test_message("d", "target", MessagePriority::Critical));

        let target = AgentId::new("target");

        // Should dequeue in priority order
        assert_eq!(queue.dequeue(&target).unwrap().priority, MessagePriority::Critical);
        assert_eq!(queue.dequeue(&target).unwrap().priority, MessagePriority::High);
        assert_eq!(queue.dequeue(&target).unwrap().priority, MessagePriority::Normal);
        assert_eq!(queue.dequeue(&target).unwrap().priority, MessagePriority::Low);
    }

    #[test]
    fn test_clear_agent_messages() {
        let mut queue = MessageQueue::new();

        queue.enqueue(create_test_message("a", "target", MessagePriority::Normal));
        queue.enqueue(create_test_message("b", "target", MessagePriority::Normal));

        let target = AgentId::new("target");
        assert_eq!(queue.pending_count(&target), 2);

        queue.clear_agent_messages(&target);
        assert_eq!(queue.pending_count(&target), 0);
    }
}
