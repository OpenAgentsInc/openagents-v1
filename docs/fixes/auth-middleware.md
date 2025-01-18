# Auth Middleware in Axum 0.8

## Overview
Authentication middleware in Axum 0.8 requires careful handling of extractors, state, and lifetimes. This document explains common issues and their solutions.

## Common Issues

### 1. Lifetime Parameters
When implementing `FromRequestParts`, you need to handle lifetimes correctly:

```rust
// ❌ Wrong - missing lifetime parameters
async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection>

// ✅ Correct - proper lifetime handling
async fn from_request_parts<'a>(parts: &'a mut Parts, state: &'a S) -> Result<Self, Self::Rejection>
```

### 2. State Access
There are multiple ways to access state in middleware:

```rust
// ❌ Problematic - trying to extract state directly
let state = State::<AppState>::from_request_parts(parts, state).await?;

// ✅ Better - using extensions
let state = parts.extensions.get::<AppState>()
    .ok_or(AuthError::NotAuthenticated)?;
```

### 3. Cookie Handling
Cookie extraction should be done before state access:

```rust
// Get cookies first
let cookies = CookieJar::from_headers(&parts.headers);

// Then get session token
let session_token = cookies
    .get(SESSION_COOKIE_NAME)
    .ok_or(AuthError::NotAuthenticated)?
    .value()
    .to_string();
```

## Solution Pattern

Here's a complete pattern for auth middleware:

```rust
#[async_trait::async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts<'a>(
        parts: &'a mut Parts,
        state: &'a S,
    ) -> Result<Self, Self::Rejection> {
        // 1. Extract cookies
        let cookies = CookieJar::from_headers(&parts.headers);
        
        // 2. Get session token
        let token = cookies.get(COOKIE_NAME)?.value().to_string();
        
        // 3. Get app state
        let state = parts.extensions.get::<AppState>()?;
        
        // 4. Validate session
        let session = validate_session(token, &state.pool).await?;
        
        // 5. Get user
        let user = get_user(session.user_id, &state.pool).await?;
        
        Ok(AuthenticatedUser { user, session })
    }
}
```

## Testing

When testing auth middleware:

```rust
#[tokio::test]
async fn test_auth_middleware() {
    // 1. Setup test state
    let state = AppState::new(config, pool);
    
    // 2. Create test app
    let app = Router::new()
        .route("/test", get(handler))
        .with_state(state);
    
    // 3. Test without auth
    let response = app.oneshot(Request::new(Body::empty())).await?;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    
    // 4. Test with auth
    let response = app
        .oneshot(
            Request::builder()
                .header("Cookie", "session=token")
                .body(Body::empty())?
        )
        .await?;
    assert_eq!(response.status(), StatusCode::OK);
}
```

## Best Practices

1. Handle lifetimes explicitly
2. Use extensions for state access
3. Extract cookies before state
4. Proper error handling and types
5. Comprehensive testing
6. Clear rejection types

## Related Issues
- Issue #562: OIDC Authentication Implementation
- Axum Extractor Documentation
- Axum Middleware Guide

## See Also
- [Axum Authentication Guide](https://docs.rs/axum/0.8/axum/extract/index.html#authentication)
- [Cookie Handling in Axum](https://docs.rs/axum-extra/latest/axum_extra/extract/cookie/index.html)
- [FromRequestParts Documentation](https://docs.rs/axum/0.8/axum/extract/trait.FromRequestParts.html)