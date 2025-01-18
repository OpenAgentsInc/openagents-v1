# Axum 0.8 Extractor Order Fix

## Issue
When using Axum 0.8 with multiple extractors including `State` and `CookieJar`, you might encounter this error:

```
error[E0277]: the trait bound `CookieJar: FromRequestParts<Pool<Postgres>>` is not satisfied
```

This occurs because Axum 0.8 is strict about extractor order, particularly with State extractors.

## Root Cause
In Axum 0.8, the order of extractors matters. State extractors must come before other extractors (like CookieJar) in handler function signatures. This is because each extractor's implementation depends on the state type being properly set up before other extractors can be processed.

## Example of the Problem

```rust
// ❌ Wrong order - will cause trait bound error
pub async fn handler(
    cookies: CookieJar,
    State(pool): State<PgPool>,
    State(config): State<OIDCConfig>,
) -> impl IntoResponse {
    // ...
}
```

## Solution
Always put State extractors first, followed by other extractors:

```rust
// ✅ Correct order
pub async fn handler(
    State(pool): State<PgPool>,
    State(config): State<OIDCConfig>,
    cookies: CookieJar,
) -> impl IntoResponse {
    // ...
}
```

## Real-World Example
In our OIDC authentication handlers, we fixed this by reordering the extractors:

```rust
// Before (causing error)
pub async fn callback(
    cookies: CookieJar,
    State(config): State<OIDCConfig>,
    State(pool): State<PgPool>,
    Query(params): Query<CallbackParams>,
) -> Result<impl IntoResponse, impl IntoResponse>

// After (fixed)
pub async fn callback(
    State(config): State<OIDCConfig>,
    State(pool): State<PgPool>,
    cookies: CookieJar,
    Query(params): Query<CallbackParams>,
) -> Result<impl IntoResponse, impl IntoResponse>
```

## Best Practices
1. Always put State extractors first in handler function signatures
2. Keep the order consistent across all handlers
3. Group similar extractors together (e.g., all State extractors together)
4. Document the required order in module-level comments

## Related Links
- [Axum Extractors Documentation](https://docs.rs/axum/0.8/axum/extract/index.html)
- [Issue #562: Add email/pass login via Scramble OIDC](https://github.com/OpenAgentsInc/openagents/issues/562)

## See Also
- [Axum Router Configuration](https://docs.rs/axum/0.8/axum/struct.Router.html)
- [State Extractor Documentation](https://docs.rs/axum/0.8/axum/extract/struct.State.html)