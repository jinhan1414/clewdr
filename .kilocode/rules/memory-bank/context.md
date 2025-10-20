# Context: Recent Work and Focus

## 1. Current Focus: Removing Polling and Refactoring Authentication

The most recent work has centered on a significant refactoring of the application's core request handling logic. The primary goals were to:
- **Eliminate all forms of polling/retry loops**: This includes removing both cross-account polling and single-account retries.
- **Enforce Token-Based Authentication**: Transition from a shared, anonymous-access model to a strict, session-based authentication model where every request must be associated with a unique `Bearer Token`.

## 2. Key Recent Changes (Commit `3487ba62`)

- **Introduced `BearerToken` Authentication**: Middleware was updated to extract a `Bearer Token` from API requests and inject it into the request context. This is the foundation for session management.
- **Refactored `CookieActor`**: The cookie/account management service was modified to support `request_by_token`. This allows the system to fetch a specific account tied to a token, rather than randomly selecting one from a pool.
- **Removed Fallback to Polling**: The provider logic was changed to return an `InvalidAuth` error if a request is made without a token, completely removing the old fallback behavior of polling from a shared pool.
- **Removed All Retry Logic**: The `try_chat` and `try_count_tokens` methods in both `ClaudeWebState` and `ClaudeCodeState` were refactored to eliminate their `for` loop-based retry mechanisms. The system now fails fast on any error.

## 3. Immediate Next Steps

- Verify that the recent refactoring has not introduced any regressions.
- Ensure the new authentication-required logic is functioning as expected across all API endpoints.
- Update any relevant documentation to reflect the new authentication requirements.