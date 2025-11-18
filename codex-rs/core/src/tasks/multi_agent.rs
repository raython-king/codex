//! Multi-agent collaborative task wrapper
//!
//! This module provides integration between the multi-agent system
//! and the task framework.

use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use codex_protocol::user_input::UserInput;

use crate::multi_agent::{MultiAgentConfig, MultiAgentTask as CoreMultiAgentTask};
use crate::state::TaskKind;
use crate::tasks::{SessionTask, SessionTaskContext};
use crate::codex::TurnContext;

/// Wrapper for multi-agent task that implements SessionTask
pub(crate) struct MultiAgentTask {
    inner: Arc<CoreMultiAgentTask>,
}

impl MultiAgentTask {
    pub(crate) fn new(config: MultiAgentConfig) -> Self {
        Self {
            inner: Arc::new(CoreMultiAgentTask::new(config)),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn with_defaults() -> Self {
        Self {
            inner: Arc::new(CoreMultiAgentTask::with_defaults()),
        }
    }
}

#[async_trait]
impl SessionTask for MultiAgentTask {
    fn kind(&self) -> TaskKind {
        TaskKind::MultiAgent
    }

    async fn run(
        self: Arc<Self>,
        session: Arc<SessionTaskContext>,
        ctx: Arc<TurnContext>,
        input: Vec<UserInput>,
        cancellation_token: CancellationToken,
    ) -> Option<String> {
        Arc::clone(&self.inner).run(session, ctx, input, cancellation_token).await
    }

    async fn abort(&self, session: Arc<SessionTaskContext>, ctx: Arc<TurnContext>) {
        self.inner.abort(session, ctx).await
    }
}
