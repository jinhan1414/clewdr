use std::sync::Arc;

use axum::{Extension, extract::State, response::Response};

use crate::{
    error::ClewdrError,
    middleware::claude::{ClaudeCodePreprocess, ClaudeContext},
    providers::LLMProvider,
    providers::claude::{ClaudeCodeProvider, ClaudeInvocation, ClaudeProviderResponse},
};

pub async fn api_claude_code(
    State(provider): State<Arc<ClaudeCodeProvider>>,
    Extension(token): Extension<crate::middleware::auth::BearerToken>,
    mut preprocess: ClaudeCodePreprocess,
) -> Result<(Extension<ClaudeContext>, Response), ClewdrError> {
    // Set the token in the context for session-based access
    preprocess.1.set_token(token.0);

    let ClaudeProviderResponse { context, response } = provider
        .invoke(ClaudeInvocation::messages(preprocess.0, preprocess.1))
        .await?;
    Ok((Extension(context), response))
}

pub async fn api_claude_code_count_tokens(
    State(provider): State<Arc<ClaudeCodeProvider>>,
    Extension(token): Extension<crate::middleware::auth::BearerToken>,
    mut preprocess: ClaudeCodePreprocess,
) -> Result<Response, ClewdrError> {
    // Set the token in the context for session-based access
    preprocess.1.set_token(token.0);

    preprocess.0.stream = Some(false);
    let ClaudeProviderResponse { response, .. } = provider
        .invoke(ClaudeInvocation::count_tokens(preprocess.0, preprocess.1))
        .await?;
    Ok(response)
}
