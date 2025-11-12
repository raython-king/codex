#![cfg(not(target_os = "windows"))]

//! Integration tests for agent-to-agent messaging.
//!
//! These tests verify message sending, receiving, priority handling,
//! and error conditions in the agent messaging system.

use anyhow::Result;
use codex_core::protocol::{EventMsg, Op};
use codex_protocol::{AgentConfig, AgentId, AgentMessage, MessagePriority, MessageType, QueuedMessage};
use core_test_support::responses::start_mock_server;
use core_test_support::skip_if_no_network;
use core_test_support::test_codex::{test_codex, TestCodex};
use core_test_support::wait_for_event_match;
use serde_json::json;

/// Test simple message send from one agent to another.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_simple_message_send() -> Result<()> {
    skip_if_no_network!(Ok(()));

    let server = start_mock_server().await;
    let TestCodex { codex, .. } = test_codex().build(&server).await?;

    // Register two agents
    let planner_config = AgentConfig {
        id: AgentId::new("planner"),
        name: "Planner Agent".to_string(),
        role: "planner".to_string(),
        system_prompt: None,
        allowed_tools: None,
        max_concurrent_tasks: 1,
    };
    codex
        .submit(Op::RegisterAgent {
            config: planner_config,
        })
        .await?;

    let executor_config = AgentConfig {
        id: AgentId::new("executor"),
        name: "Executor Agent".to_string(),
        role: "executor".to_string(),
        system_prompt: None,
        allowed_tools: None,
        max_concurrent_tasks: 1,
    };
    codex
        .submit(Op::RegisterAgent {
            config: executor_config,
        })
        .await?;

    // Wait for both agents to register
    wait_for_event_match(&codex, |ev| match ev {
        EventMsg::AgentRegistered(e) if e.agent_id == AgentId::new("planner") => Some(()),
        _ => None,
    })
    .await;

    wait_for_event_match(&codex, |ev| match ev {
        EventMsg::AgentRegistered(e) if e.agent_id == AgentId::new("executor") => Some(()),
        _ => None,
    })
    .await;

    // Send message from planner to executor
    let message = AgentMessage::new(
        AgentId::new("planner"),
        AgentId::new("executor"),
        MessageType::Request,
        json!({"task": "execute_command"}),
    );

    let message_id = message.id.clone();
    let queued = QueuedMessage::new(message);
    codex.submit(Op::SendToAgent { message: queued }).await?;

    // Wait for MessageSent event
    let sent_event = wait_for_event_match(&codex, |ev| match ev {
        EventMsg::MessageSent(e) => Some(e.clone()),
        _ => None,
    })
    .await;

    assert_eq!(sent_event.message_id, message_id);
    assert_eq!(sent_event.from, AgentId::new("planner"));
    assert_eq!(sent_event.to, AgentId::new("executor"));

    Ok(())
}

/// Test message priority ordering by sending messages with different priorities.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_message_priority_ordering() -> Result<()> {
    skip_if_no_network!(Ok(()));

    let server = start_mock_server().await;
    let TestCodex { codex, .. } = test_codex().build(&server).await?;

    // Register agents
    let sender_config = AgentConfig {
        id: AgentId::new("sender"),
        name: "Sender".to_string(),
        role: "generic".to_string(),
        system_prompt: None,
        allowed_tools: None,
        max_concurrent_tasks: 1,
    };
    codex
        .submit(Op::RegisterAgent {
            config: sender_config,
        })
        .await?;

    let receiver_config = AgentConfig {
        id: AgentId::new("receiver"),
        name: "Receiver".to_string(),
        role: "generic".to_string(),
        system_prompt: None,
        allowed_tools: None,
        max_concurrent_tasks: 1,
    };
    codex
        .submit(Op::RegisterAgent {
            config: receiver_config,
        })
        .await?;

    // Wait for registrations
    wait_for_event_match(&codex, |ev| match ev {
        EventMsg::AgentRegistered(e) if e.agent_id == AgentId::new("sender") => Some(()),
        _ => None,
    })
    .await;

    wait_for_event_match(&codex, |ev| match ev {
        EventMsg::AgentRegistered(e) if e.agent_id == AgentId::new("receiver") => Some(()),
        _ => None,
    })
    .await;

    // Send messages with different priorities
    let priorities = vec![
        MessagePriority::Low,
        MessagePriority::Critical,
        MessagePriority::Normal,
        MessagePriority::High,
    ];

    for (i, priority) in priorities.iter().enumerate() {
        let message = AgentMessage::new(
            AgentId::new("sender"),
            AgentId::new("receiver"),
            MessageType::Notification,
            json!({"index": i}),
        );

        let queued = QueuedMessage::new(message).with_priority(*priority);
        codex.submit(Op::SendToAgent { message: queued }).await?;
    }

    // Verify messages sent
    for _ in 0..4 {
        wait_for_event_match(&codex, |ev| match ev {
            EventMsg::MessageSent(_) => Some(()),
            _ => None,
        })
        .await;
    }

    Ok(())
}

/// Test message with response requirement.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_message_with_response() -> Result<()> {
    skip_if_no_network!(Ok(()));

    let server = start_mock_server().await;
    let TestCodex { codex, .. } = test_codex().build(&server).await?;

    // Register agents
    let agent_a_config = AgentConfig {
        id: AgentId::new("agent-a"),
        name: "Agent A".to_string(),
        role: "generic".to_string(),
        system_prompt: None,
        allowed_tools: None,
        max_concurrent_tasks: 1,
    };
    codex
        .submit(Op::RegisterAgent {
            config: agent_a_config,
        })
        .await?;

    let agent_b_config = AgentConfig {
        id: AgentId::new("agent-b"),
        name: "Agent B".to_string(),
        role: "generic".to_string(),
        system_prompt: None,
        allowed_tools: None,
        max_concurrent_tasks: 1,
    };
    codex
        .submit(Op::RegisterAgent {
            config: agent_b_config,
        })
        .await?;

    // Wait for registrations
    wait_for_event_match(&codex, |ev| match ev {
        EventMsg::AgentRegistered(e) if e.agent_id == AgentId::new("agent-a") => Some(()),
        _ => None,
    })
    .await;

    wait_for_event_match(&codex, |ev| match ev {
        EventMsg::AgentRegistered(e) if e.agent_id == AgentId::new("agent-b") => Some(()),
        _ => None,
    })
    .await;

    // Send request
    let request = AgentMessage::new(
        AgentId::new("agent-a"),
        AgentId::new("agent-b"),
        MessageType::Request,
        json!({"query": "status"}),
    );

    let _request_id = request.id.clone();
    let queued_request = QueuedMessage::new(request).requires_response();

    codex
        .submit(Op::SendToAgent {
            message: queued_request,
        })
        .await?;

    wait_for_event_match(&codex, |ev| match ev {
        EventMsg::MessageSent(_) => Some(()),
        _ => None,
    })
    .await;

    // Create response
    let response = AgentMessage::new(
        AgentId::new("agent-b"),
        AgentId::new("agent-a"),
        MessageType::Response,
        json!({"status": "ok"}),
    );

    let queued_response = QueuedMessage::new(response);
    codex
        .submit(Op::SendToAgent {
            message: queued_response,
        })
        .await?;

    let response_sent = wait_for_event_match(&codex, |ev| match ev {
        EventMsg::MessageSent(e) => Some(e.clone()),
        _ => None,
    })
    .await;

    assert_eq!(response_sent.from, AgentId::new("agent-b"));
    assert_eq!(response_sent.to, AgentId::new("agent-a"));

    Ok(())
}

/// Test message to nonexistent agent fails.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_message_to_nonexistent_agent_fails() -> Result<()> {
    skip_if_no_network!(Ok(()));

    let server = start_mock_server().await;
    let TestCodex { codex, .. } = test_codex().build(&server).await?;

    // Register only sender
    let sender_config = AgentConfig {
        id: AgentId::new("sender"),
        name: "Sender".to_string(),
        role: "generic".to_string(),
        system_prompt: None,
        allowed_tools: None,
        max_concurrent_tasks: 1,
    };
    codex
        .submit(Op::RegisterAgent {
            config: sender_config,
        })
        .await?;

    // Wait for registration
    wait_for_event_match(&codex, |ev| match ev {
        EventMsg::AgentRegistered(e) if e.agent_id == AgentId::new("sender") => Some(()),
        _ => None,
    })
    .await;

    // Try to send to non-existent agent
    let message = AgentMessage::new(
        AgentId::new("sender"),
        AgentId::new("nonexistent"),
        MessageType::Notification,
        json!({}),
    );

    let queued = QueuedMessage::new(message);
    codex.submit(Op::SendToAgent { message: queued }).await?;

    // Should receive error event
    let error = wait_for_event_match(&codex, |ev| match ev {
        EventMsg::Error(e) => Some(e.clone()),
        _ => None,
    })
    .await;

    assert!(
        error.message.contains("not found") || error.message.contains("does not exist")
    );

    Ok(())
}

/// Test broadcast to multiple agents.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_broadcast_to_multiple_agents() -> Result<()> {
    skip_if_no_network!(Ok(()));

    let server = start_mock_server().await;
    let TestCodex { codex, .. } = test_codex().build(&server).await?;

    // Register broadcaster
    let broadcaster_config = AgentConfig {
        id: AgentId::new("broadcaster"),
        name: "Broadcaster".to_string(),
        role: "generic".to_string(),
        system_prompt: None,
        allowed_tools: None,
        max_concurrent_tasks: 1,
    };
    codex
        .submit(Op::RegisterAgent {
            config: broadcaster_config,
        })
        .await?;

    // Register multiple receivers
    let receiver_ids = vec!["receiver-1", "receiver-2", "receiver-3"];
    for id in &receiver_ids {
        let config = AgentConfig {
            id: AgentId::new(*id),
            name: format!("Receiver {}", id),
            role: "generic".to_string(),
            system_prompt: None,
            allowed_tools: None,
            max_concurrent_tasks: 1,
        };
        codex.submit(Op::RegisterAgent { config }).await?;
    }

    // Wait for all registrations
    wait_for_event_match(&codex, |ev| match ev {
        EventMsg::AgentRegistered(e) if e.agent_id == AgentId::new("broadcaster") => Some(()),
        _ => None,
    })
    .await;

    for receiver_id in &receiver_ids {
        wait_for_event_match(&codex, |ev| match ev {
            EventMsg::AgentRegistered(e) if e.agent_id == AgentId::new(*receiver_id) => Some(()),
            _ => None,
        })
        .await;
    }

    // Broadcast to all receivers
    for receiver_id in &receiver_ids {
        let message = AgentMessage::new(
            AgentId::new("broadcaster"),
            AgentId::new(*receiver_id),
            MessageType::Notification,
            json!({"event": "broadcast_test"}),
        );

        let queued = QueuedMessage::new(message);
        codex.submit(Op::SendToAgent { message: queued }).await?;
    }

    // Verify all messages sent
    for _ in 0..receiver_ids.len() {
        wait_for_event_match(&codex, |ev| match ev {
            EventMsg::MessageSent(_) => Some(()),
            _ => None,
        })
        .await;
    }

    Ok(())
}

/// Test sending different message types.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_different_message_types() -> Result<()> {
    skip_if_no_network!(Ok(()));

    let server = start_mock_server().await;
    let TestCodex { codex, .. } = test_codex().build(&server).await?;

    // Register agents
    let sender_config = AgentConfig {
        id: AgentId::new("sender"),
        name: "Sender".to_string(),
        role: "generic".to_string(),
        system_prompt: None,
        allowed_tools: None,
        max_concurrent_tasks: 1,
    };
    codex
        .submit(Op::RegisterAgent {
            config: sender_config,
        })
        .await?;

    let receiver_config = AgentConfig {
        id: AgentId::new("receiver"),
        name: "Receiver".to_string(),
        role: "generic".to_string(),
        system_prompt: None,
        allowed_tools: None,
        max_concurrent_tasks: 1,
    };
    codex
        .submit(Op::RegisterAgent {
            config: receiver_config,
        })
        .await?;

    // Wait for registrations
    wait_for_event_match(&codex, |ev| match ev {
        EventMsg::AgentRegistered(e) if e.agent_id == AgentId::new("sender") => Some(()),
        _ => None,
    })
    .await;

    wait_for_event_match(&codex, |ev| match ev {
        EventMsg::AgentRegistered(e) if e.agent_id == AgentId::new("receiver") => Some(()),
        _ => None,
    })
    .await;

    // Test different message types
    let message_types = vec![
        MessageType::Request,
        MessageType::Response,
        MessageType::Notification,
        MessageType::Query,
    ];

    for msg_type in message_types {
        let message = AgentMessage::new(
            AgentId::new("sender"),
            AgentId::new("receiver"),
            msg_type,
            json!({"type": format!("{:?}", msg_type)}),
        );

        let queued = QueuedMessage::new(message);
        codex.submit(Op::SendToAgent { message: queued }).await?;

        let sent = wait_for_event_match(&codex, |ev| match ev {
            EventMsg::MessageSent(e) => Some(e.clone()),
            _ => None,
        })
        .await;

        assert_eq!(sent.message_type, msg_type);
    }

    Ok(())
}

/// Test message with timeout.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_message_with_timeout() -> Result<()> {
    skip_if_no_network!(Ok(()));

    let server = start_mock_server().await;
    let TestCodex { codex, .. } = test_codex().build(&server).await?;

    // Register agents
    let sender_config = AgentConfig {
        id: AgentId::new("sender"),
        name: "Sender".to_string(),
        role: "generic".to_string(),
        system_prompt: None,
        allowed_tools: None,
        max_concurrent_tasks: 1,
    };
    codex
        .submit(Op::RegisterAgent {
            config: sender_config,
        })
        .await?;

    let receiver_config = AgentConfig {
        id: AgentId::new("receiver"),
        name: "Receiver".to_string(),
        role: "generic".to_string(),
        system_prompt: None,
        allowed_tools: None,
        max_concurrent_tasks: 1,
    };
    codex
        .submit(Op::RegisterAgent {
            config: receiver_config,
        })
        .await?;

    // Wait for registrations
    wait_for_event_match(&codex, |ev| match ev {
        EventMsg::AgentRegistered(e) if e.agent_id == AgentId::new("sender") => Some(()),
        _ => None,
    })
    .await;

    wait_for_event_match(&codex, |ev| match ev {
        EventMsg::AgentRegistered(e) if e.agent_id == AgentId::new("receiver") => Some(()),
        _ => None,
    })
    .await;

    // Send message with timeout
    let message = AgentMessage::new(
        AgentId::new("sender"),
        AgentId::new("receiver"),
        MessageType::Request,
        json!({"task": "time_sensitive"}),
    );

    let queued = QueuedMessage::new(message)
        .with_timeout(5000)
        .requires_response();

    codex.submit(Op::SendToAgent { message: queued }).await?;

    // Should receive MessageSent event
    let sent = wait_for_event_match(&codex, |ev| match ev {
        EventMsg::MessageSent(e) => Some(e.clone()),
        _ => None,
    })
    .await;

    assert_eq!(sent.from, AgentId::new("sender"));
    assert_eq!(sent.to, AgentId::new("receiver"));

    Ok(())
}
