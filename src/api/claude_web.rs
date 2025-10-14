use std::sync::Arc;

use axum::{Extension, extract::State, response::Response};

use crate::{
    error::ClewdrError,
    middleware::claude::{ClaudeContext, ClaudeWebPreprocess},
    providers::LLMProvider,
    providers::claude::{ClaudeInvocation, ClaudeProviderResponse, ClaudeWebProvider},
};
/// Axum handler for the API messages
/// Main API endpoint for handling message requests to Claude
/// Processes messages, handles retries, and returns responses in stream or non-stream mode
///
/// # Arguments
/// * `XApiKey(_)` - API key authentication
/// * `state` - Application state containing client information
/// * `p` - Request body containing messages and configuration
///
/// # Returns
/// * `Response` - Stream or JSON response from Claude
pub async fn api_claude_web(
    State(provider): State<Arc<ClaudeWebProvider>>,
    Extension(token): Extension<crate::middleware::auth::BearerToken>,
    mut preprocess: ClaudeWebPreprocess,
) -> Result<(Extension<ClaudeContext>, Response), ClewdrError> {
    // Set the token in the context for session-based access
    preprocess.1.set_token(token.0);

    let ClaudeProviderResponse { context, response } = provider
        .invoke(ClaudeInvocation::messages(preprocess.0, preprocess.1))
        .await?;
    Ok((Extension(context), response))
}
