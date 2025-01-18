# Axum 0.8 Extractor Patterns

## Overview
This document covers common patterns and fixes for working with extractors in Axum 0.8, particularly around state management and cookie handling.

## Common Issues

### 1. FromRequestParts Lifetime Parameters
When implementing `FromRequestParts`, you need to specify lifetime parameters correctly:

```rust
// ❌ Wrong
async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection>

// ✅ Correct
async fn from_request_parts<'a, 'b>(parts: &'a mut Parts, state: &'b S) -> Result<Self, Self::Rejection>
```

### 2. State Management
Use Extension instead of State for shared state:

```rust
// ❌ Problematic
pub async fn handler(
    State(state): State<AppState>,
    cookies: CookieJar,
) -> impl IntoResponse

// ✅ Better
pub async fn handler(
    Extension(state): Extension<AppState>,
    cookies: CookieJar,
) -> impl IntoResponse
```

### 3. Router Configuration
Use layer for Extension state:

```rust
// ❌ Wrong
.with_state(state)

// ✅ Correct
.layer(Extension(state))
```

## Best Practices

### 1. Handler Signatures
Keep handler signatures simple and consistent:

```rust
pub async fn handler(
    Extension(state): Extension<AppState>,
    Query(params): Query<Params>,
) -> impl IntoResponse
```

### 2. FromRequestParts Implementation
Proper implementation for custom extractors:

```rust
#[async_trait]
impl<S> FromRequestParts<S> for MyExtractor
where
    S: Send + Sync,
{
    type Rejection = MyError;

    async fn from_request_parts<'a, 'b>(
        parts: &'a mut Parts,
        state: &'b S,
    ) -> Result<Self, Self::Rejection> {
        // Implementation
    }
}
```

### 3. Cookie Handling
Use the new cookie builder API:

```rust
// ❌ Old way
Cookie::build(name)
    .value(value)
    .build()

// ✅ New way
Cookie::build((name, value))
    .build()
```

### 4. Error Handling
Use proper error types and IntoResponse:

```rust
#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("Not authenticated")]
    NotAuthenticated,
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        let status = StatusCode::UNAUTHORIZED;
        let body = Json(ErrorResponse {
            error: self.to_string(),
        });
        (status, body).into_response()
    }
}
```

### 5. State Types
When using multiple state types, combine them into a single type:

```rust
#[derive(Clone)]
pub struct AppState {
    config: Config,
    pool: PgPool,
}

// Then use in router
let app = Router::new()
    .route("/path", get(handler))
    .layer(Extension(AppState::new(config, pool)));
```

## Testing

When testing handlers with state:

```rust
#[tokio::test]
async fn test_handler() {
    let state = AppState::new(/* ... */);
    
    let app = Router::new()
        .route("/test", get(handler))
        .layer(Extension(state));
    
    let response = app
        .oneshot(Request::new(Body::empty()))
        .await
        .unwrap();
}
```

## Common Gotchas

1. **Lifetime Parameters**: Always specify lifetimes in `FromRequestParts` implementations
2. **State Access**: Use `parts.extensions.get()` to access state in extractors
3. **Cookie Handling**: Use the tuple form for cookie building
4. **Handler Types**: Make sure handler return types implement `IntoResponse`
5. **Extension vs State**: Prefer Extension for shared state

## Related Issues
- Issue #562: OIDC Authentication Implementation
- [Axum Extractors Documentation](https://docs.rs/axum/0.8/axum/extract/index.html)
- [Cookie Handling in Axum](https://docs.rs/axum-extra/latest/axum_extra/extract/cookie/index.html)

## See Also
- [Axum Router Documentation](https://docs.rs/axum/0.8/axum/struct.Router.html)
- [Extension Documentation](https://docs.rs/axum/0.8/axum/extract/struct.Extension.html)
- [FromRequestParts Documentation](https://docs.rs/axum/0.8/axum/extract/trait.FromRequestParts.html)