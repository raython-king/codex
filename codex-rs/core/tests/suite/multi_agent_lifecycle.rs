#![cfg(not(target_os = "windows"))]

//! Integration tests for multi-agent lifecycle management.
//!
//! These tests verify agent registration, unregistration, agent-specific turns,
//! and tool isolation in a multi-agent environment.

use anyhow::Result;
use codex_core::protocol::{EventMsg, Op};
use codex_protocol::user_input::UserInput;
use codex_protocol::{AgentConfig, AgentId};
use core_test_support::responses::{
    ev_assistant_message, ev_completed, ev_response_created, mount_sse_once_match, sse,
    start_mock_server,
};
use core_test_support::skip_if_no_network;
use core_test_support::test_codex::{test_codex, TestCodex};
use core_test_support::wait_for_event_match;
use wiremock::matchers::any;

/// Test basic agent registration and unregistration lifecycle.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_agent_registration_and_unregistration() -> Result<()> {
    skip_if_no_network!(Ok(()));

    let server = start_mock_server().await;
    let TestCodex { codex, .. } = test_codex().build(&server).await?;

    // Register a new agent
    let config = AgentConfig {
        id: AgentId::new("test-agent"),
        name: "Test Agent".to_string(),
        role: "tester".to_string(),
        system_prompt: Some("You are a test agent".to_string()),
        allowed_tools: Some(vec!["Bash".to_string()]),
        max_concurrent_tasks: 1,
    };

    codex.submit(Op::RegisterAgent { config }).await?;

    // Verify AgentRegistered event
    let registered = wait_for_event_match(&codex, |ev| match ev {
        EventMsg::AgentRegistered(e) => Some(e.clone()),
        _ => None,
    })
    .await;

    assert_eq!(registered.agent_id, AgentId::new("test-agent"));
    assert_eq!(registered.name, "Test Agent");

    // Unregister the agent
    codex
        .submit(Op::UnregisterAgent {
            agent_id: AgentId::new("test-agent"),
        })
        .await?;

    // Verify AgentUnregistered event
    let unregistered = wait_for_event_match(&codex, |ev| match ev {
        EventMsg::AgentUnregistered(e) => Some(e.clone()),
        _ => None,
    })
    .await;

    assert_eq!(unregistered.agent_id, AgentId::new("test-agent"));

    Ok(())
}

/// Test that registering an agent with a duplicate ID fails.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_duplicate_agent_registration_fails() -> Result<()> {
    skip_if_no_network!(Ok(()));

    let server = start_mock_server().await;
    let TestCodex { codex, .. } = test_codex().build(&server).await?;

    // Register first agent
    let config = AgentConfig {
        id: AgentId::new("duplicate"),
        name: "First Agent".to_string(),
        role: "generic".to_string(),
        system_prompt: None,
        allowed_tools: None,
        max_concurrent_tasks: 1,
    };

    codex
        .submit(Op::RegisterAgent {
            config: config.clone(),
        })
        .await?;

    // Wait for successful registration
    wait_for_event_match(&codex, |ev| match ev {
        EventMsg::AgentRegistered(e) if e.agent_id == AgentId::new("duplicate") => Some(()),
        _ => None,
    })
    .await;

    // Try to register another agent with same ID
    let config2 = AgentConfig {
        id: AgentId::new("duplicate"),
        name: "Second Agent".to_string(),
        role: "generic".to_string(),
        system_prompt: None,
        allowed_tools: None,
        max_concurrent_tasks: 1,
    };

    codex.submit(Op::RegisterAgent { config: config2 }).await?;

    // Should receive an error event
    let error = wait_for_event_match(&codex, |ev| match ev {
        EventMsg::Error(e) => Some(e.clone()),
        _ => None,
    })
    .await;

    assert!(error.message.contains("already exists"));

    Ok(())
}

/// Test user turn directed to a specific agent.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_user_turn_with_specific_agent() -> Result<()> {
    skip_if_no_network!(Ok(()));

    let server = start_mock_server().await;
    let TestCodex { codex, .. } = test_codex().build(&server).await?;

    // Register a specialized agent
    let config = AgentConfig {
        id: AgentId::new("planner"),
        name: "Planner Agent".to_string(),
        role: "planner".to_string(),
        system_prompt: Some("You plan tasks".to_string()),
        allowed_tools: Some(vec!["Read".to_string()]),
        max_concurrent_tasks: 1,
    };

    codex.submit(Op::RegisterAgent { config }).await?;

    // Wait for registration
    wait_for_event_match(&codex, |ev| match ev {
        EventMsg::AgentRegistered(e) if e.agent_id == AgentId::new("planner") => Some(()),
        _ => None,
    })
    .await;

    // Mock response
    let response = sse(vec![
        ev_response_created("resp-1"),
        ev_assistant_message("msg-1", "Task planned successfully"),
        ev_completed("resp-1"),
    ]);
    mount_sse_once_match(&server, any(), response).await;

    // Submit user turn with specific agent
    codex
        .submit(Op::UserInput {
            items: vec![UserInput::Text {
                text: "Create a plan for this task".to_string(),
            }],
            agent_id: Some(AgentId::new("planner")),
        })
        .await?;

    // Verify events have correct agent_id
    let item_started = wait_for_event_match(&codex, |ev| match ev {
        EventMsg::ItemStarted(e) => Some(e.clone()),
        _ => None,
    })
    .await;

    assert_eq!(item_started.agent_id, Some(AgentId::new("planner")));

    Ok(())
}

/// Test that unregistering the default agent fails.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_cannot_unregister_default_agent() -> Result<()> {
    skip_if_no_network!(Ok(()));

    let server = start_mock_server().await;
    let TestCodex { codex, .. } = test_codex().build(&server).await?;

    // Try to unregister the default agent
    codex
        .submit(Op::UnregisterAgent {
            agent_id: AgentId::default(),
        })
        .await?;

    // Should receive an error event
    let error = wait_for_event_match(&codex, |ev| match ev {
        EventMsg::Error(e) => Some(e.clone()),
        _ => None,
    })
    .await;

    assert!(
        error.message.contains("default agent") || error.message.contains("cannot be unregistered")
    );

    Ok(())
}

/// Test that default agent (no agent_id specified) still works.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_default_agent_backward_compatibility() -> Result<()> {
    skip_if_no_network!(Ok(()));

    let server = start_mock_server().await;
    let TestCodex { codex, .. } = test_codex().build(&server).await?;

    // Mock response
    let response = sse(vec![
        ev_response_created("resp-1"),
        ev_assistant_message("msg-1", "Hello from default agent"),
        ev_completed("resp-1"),
    ]);
    mount_sse_once_match(&server, any(), response).await;

    // Submit user turn WITHOUT specifying agent_id (should use default)
    codex
        .submit(Op::UserInput {
            items: vec![UserInput::Text {
                text: "Hello".to_string(),
            }],
            agent_id: None,
        })
        .await?;

    // Should work fine with default agent
    let item_started = wait_for_event_match(&codex, |ev| match ev {
        EventMsg::ItemStarted(e) => Some(e.clone()),
        _ => None,
    })
    .await;

    // Default agent may have None or Some(default_id) - both are valid
    assert!(
        item_started.agent_id.is_none()
            || item_started.agent_id == Some(AgentId::default())
    );

    Ok(())
}

/// Test registering multiple agents and switching between them.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_multiple_agents() -> Result<()> {
    skip_if_no_network!(Ok(()));

    let server = start_mock_server().await;
    let TestCodex { codex, .. } = test_codex().build(&server).await?;

    // Register first agent
    let planner_config = AgentConfig {
        id: AgentId::new("planner"),
        name: "Planner".to_string(),
        role: "planner".to_string(),
        system_prompt: None,
        allowed_tools: Some(vec!["Read".to_string()]),
        max_concurrent_tasks: 1,
    };

    codex
        .submit(Op::RegisterAgent {
            config: planner_config,
        })
        .await?;

    // Register second agent
    let executor_config = AgentConfig {
        id: AgentId::new("executor"),
        name: "Executor".to_string(),
        role: "executor".to_string(),
        system_prompt: None,
        allowed_tools: Some(vec!["Bash".to_string(), "Write".to_string()]),
        max_concurrent_tasks: 1,
    };

    codex
        .submit(Op::RegisterAgent {
            config: executor_config,
        })
        .await?;

    // Wait for both registrations
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

    // Now we have 3 agents total: default + planner + executor
    // Test that we can send turns to different agents
    let response1 = sse(vec![
        ev_response_created("resp-1"),
        ev_assistant_message("msg-1", "Planning..."),
        ev_completed("resp-1"),
    ]);
    mount_sse_once_match(&server, any(), response1).await;

    codex
        .submit(Op::UserInput {
            items: vec![UserInput::Text {
                text: "Plan this".to_string(),
            }],
            agent_id: Some(AgentId::new("planner")),
        })
        .await?;

    let planner_event = wait_for_event_match(&codex, |ev| match ev {
        EventMsg::ItemStarted(e) if e.agent_id == Some(AgentId::new("planner")) => Some(()),
        _ => None,
    })
    .await;

    assert_eq!(planner_event, ());

    Ok(())
}

/// Test agent configuration with custom settings.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_agent_custom_configuration() -> Result<()> {
    skip_if_no_network!(Ok(()));

    let server = start_mock_server().await;
    let TestCodex { codex, .. } = test_codex().build(&server).await?;

    // Register agent with custom configuration
    let config = AgentConfig {
        id: AgentId::new("custom"),
        name: "Custom Agent".to_string(),
        role: "specialist".to_string(),
        system_prompt: Some("You are a specialized agent with custom behavior".to_string()),
        allowed_tools: Some(vec!["Read".to_string(), "Grep".to_string()]),
        max_concurrent_tasks: 3,
    };

    codex.submit(Op::RegisterAgent { config }).await?;

    // Verify registration
    let registered = wait_for_event_match(&codex, |ev| match ev {
        EventMsg::AgentRegistered(e) => Some(e.clone()),
        _ => None,
    })
    .await;

    assert_eq!(registered.agent_id, AgentId::new("custom"));
    assert_eq!(registered.name, "Custom Agent");

    Ok(())
}

/// Test interrupt operation with specific agent.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_interrupt_specific_agent() -> Result<()> {
    skip_if_no_network!(Ok(()));

    let server = start_mock_server().await;
    let TestCodex { codex, .. } = test_codex().build(&server).await?;

    // Register an agent
    let config = AgentConfig {
        id: AgentId::new("worker"),
        name: "Worker Agent".to_string(),
        role: "worker".to_string(),
        system_prompt: None,
        allowed_tools: None,
        max_concurrent_tasks: 1,
    };

    codex.submit(Op::RegisterAgent { config }).await?;

    wait_for_event_match(&codex, |ev| match ev {
        EventMsg::AgentRegistered(e) if e.agent_id == AgentId::new("worker") => Some(()),
        _ => None,
    })
    .await;

    // Test interrupting specific agent
    codex
        .submit(Op::Interrupt {
            agent_id: Some(AgentId::new("worker")),
        })
        .await?;

    // The interrupt should be accepted (no error)
    // Note: We don't expect a TurnAborted event if there's no active turn

    Ok(())
}

/// Test interrupt all agents (no agent_id specified).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_interrupt_all_agents() -> Result<()> {
    skip_if_no_network!(Ok(()));

    let server = start_mock_server().await;
    let TestCodex { codex, .. } = test_codex().build(&server).await?;

    // Test interrupting all agents
    codex
        .submit(Op::Interrupt { agent_id: None })
        .await?;

    // The interrupt should be accepted (no error)
    Ok(())
}
