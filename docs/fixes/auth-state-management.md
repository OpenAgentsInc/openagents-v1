# Auth State Management in Axum 0.8

## Overview
When building authentication systems with Axum 0.8, proper state management is crucial for handling dependencies like database connections and configuration. This document explains common patterns and fixes for state management issues.

## Common Issues

### 1. Multiple State Types
When your handlers need access to multiple types of state (e.g., database connections and configuration), you might be tempted to use multiple `State` extractors:

```rust
// ❌ Problematic approach
async fn handler(
    State(config): State<OIDCConfig>,
    State(pool): State<PgPool>,
) -> impl IntoResponse
```

This causes trait bound errors because Axum expects a single state type.

### 2. Combined Tuple State
Another common attempt is to use tuple state:

```rust
// ❌ Also problematic
.with_state((config, pool.clone()))
```

This causes issues with trait bounds and type inference.

## Solution: AppState Pattern

The recommended approach is to create a dedicated state struct:

```rust
#[derive(Clone)]
pub struct AppState {
    config: OIDCConfig,
    pool: PgPool,
}

impl AppState {
    pub fn new(config: OIDCConfig, pool: PgPool) -> Self {
        Self { config, pool }
    }
}
```

### Usage in Handlers

```rust
// ✅ Correct approach
async fn handler(
    State(state): State<AppState>,
) -> impl IntoResponse {
    // Access state.config and state.pool
}
```

### Router Configuration

```rust
let state = AppState::new(config, pool);
let app = Router::new()
    .route("/path", get(handler))
    .with_state(state);
```

## Benefits

1. **Type Safety**: Single, well-defined state type
2. **Maintainability**: Easy to add new state fields
3. **Performance**: Single state extraction per request
4. **Clarity**: Clear ownership and lifetime semantics
5. **Extensibility**: Easy to add methods to AppState

## Common Gotchas

1. State type must implement `Clone`
2. All state fields should be thread-safe (Send + Sync)
3. Consider using Arc for shared resources
4. Be careful with lifetime parameters

## Testing

When testing handlers with state:

```rust
#[tokio::test]
async fn test_handler() {
    let state = AppState::new(
        test_config(),
        test_pool().await
    );
    
    let app = Router::new()
        .route("/test", get(handler))
        .with_state(state);
    
    // Run tests...
}
```

## Related Issues
- Issue #562: OIDC Authentication Implementation
- Axum State Management Documentation
- Axum Extractor Documentation

## See Also
- [Axum State Documentation](https://docs.rs/axum/0.8/axum/extract/struct.State.html)
- [Router State Configuration](https://docs.rs/axum/0.8/axum/struct.Router.html#method.with_state)