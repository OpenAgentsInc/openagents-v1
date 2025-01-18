# Axum 0.8 Extractor Patterns

## Overview
This document covers common patterns and fixes for working with extractors in Axum 0.8, particularly around state management and cookie handling.

## Common Issues

### 1. FromRequestParts Implementation
When implementing `FromRequestParts`, use the standard trait implementation:

```rust
#[async_trait]
impl<S> FromRequestParts<S> for MyExtractor
where
    S: Send + Sync,
{
    type Rejection = MyError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Implementation
    }
}
```

### 2. Handler Return Types
Make sure handler return types implement all necessary traits:

```rust
// ❌ Problematic
pub async fn handler() -> impl IntoResponse {
    // ...
}

// ✅ Better
pub async fn handler() -> impl IntoResponse + Send + 'static {
    // ...
}
```

### 3. Router Configuration
Use Extension for shared state:

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
) -> impl IntoResponse + Send + 'static
```

### 2. State Management
Use a single state type:

```rust
#[derive(Clone)]
pub struct AppState {
    config: Config,
    pool: PgPool,
}

impl AppState {
    pub fn new(config: Config, pool: PgPool) -> Self {
        Self { config, pool }
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

### 5. Handler Bounds
Make sure handlers have proper bounds:

```rust
pub async fn handler(
    Extension(state): Extension<AppState>,
) -> impl IntoResponse + Send + 'static {
    // Implementation
}
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

1. **Handler Bounds**: Always include `Send + 'static` for handler return types
2. **State Access**: Use Extension instead of State for shared state
3. **Cookie Handling**: Use the tuple form for cookie building
4. **Handler Types**: Make sure handler return types implement `IntoResponse`
5. **Router Configuration**: Use layer for Extension state

## Related Issues
- Issue #562: OIDC Authentication Implementation
- [Axum Extractors Documentation](https://docs.rs/axum/0.8/axum/extract/index.html)
- [Cookie Handling in Axum](https://docs.rs/axum-extra/latest/axum_extra/extract/cookie/index.html)

## See Also
- [Axum Router Documentation](https://docs.rs/axum/0.8/axum/struct.Router.html)
- [Extension Documentation](https://docs.rs/axum/0.8/axum/extract/struct.Extension.html)
- [FromRequestParts Documentation](https://docs.rs/axum/0.8/axum/extract/trait.FromRequestParts.html)