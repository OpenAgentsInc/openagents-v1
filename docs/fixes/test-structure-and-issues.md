# Test Structure and Issues Analysis

## Current Test Structure

```
tests/
├── admin_middleware.rs    # Admin authentication middleware tests
├── admin_routes.rs       # Admin route handler tests
├── agent.rs             # Top-level agent tests
├── agent/               # Detailed agent testing modules
│   ├── core.rs         # Core agent functionality tests
│   ├── manager.rs      # Basic agent manager tests
│   ├── manager_comprehensive.rs  # Extended manager tests
│   ├── manager_impl.rs  # Manager implementation details
│   ├── mod.rs          # Module organization
│   └── nostr.rs        # Nostr-specific agent tests
├── emailoptin.rs       # Email opt-in functionality tests
├── health_check.rs     # Basic API health check tests
├── nostr/              # Nostr protocol testing
│   ├── database.rs     # Nostr database operations
│   ├── event.rs        # Nostr event handling
│   ├── mod.rs         # Module organization
│   └── subscription.rs # Nostr subscription handling
└── repomap.rs         # Repository mapping tests

## Current Test Issues

### 1. Database Setup Issues

Current failures in test setup:

```rust
Failed to create sessions updated_at trigger: function update_updated_at() does not exist
relation "sessions" does not exist
Failed to create users table: duplicate key value violates unique constraint
```

Problems identified:
- Trigger function creation failing
- Table creation order issues
- Race conditions in concurrent tests
- Incomplete cleanup between tests

### 2. Missing Test Coverage

#### Authentication and Sessions
- Need integration tests for full auth flow
- Missing session management tests
- Cookie handling tests needed
- Token refresh flow tests missing

#### Database Operations
- Missing transaction tests
- Need concurrent operation tests
- Missing edge case tests for DB errors

#### OIDC Integration
- Need mock OIDC provider tests
- Missing token validation tests
- Need JWKS endpoint tests
- Missing refresh token tests

## Required Fixes

### 1. Database Test Setup

```rust
// Need to implement in test_helpers.rs:
async fn setup_test_db_properly(pool: &PgPool) {
    // 1. Proper cleanup first
    // 2. Create types and functions
    // 3. Create tables in correct order
    // 4. Create triggers
    // 5. Add test data if needed
}

async fn cleanup_test_db_properly(pool: &PgPool) {
    // 1. Drop all triggers
    // 2. Drop all tables
    // 3. Drop all functions
    // 4. Drop all custom types
}
```

### 2. Test Isolation

Need to implement:
- Proper test database isolation
- Transaction wrapping for tests
- Better concurrent test handling
- Proper cleanup between tests

### 3. Missing Tests to Add

1. Authentication Tests
```rust
#[tokio::test]
async fn test_auth_flow_complete() {
    // Test full authentication flow:
    // 1. Start auth
    // 2. Handle callback
    // 3. Verify session
    // 4. Test refresh
    // 5. Test logout
}

#[tokio::test]
async fn test_concurrent_sessions() {
    // Test multiple active sessions:
    // 1. Create multiple sessions
    // 2. Verify all are valid
    // 3. Test expiration
    // 4. Test cleanup
}
```

2. OIDC Integration Tests
```rust
#[tokio::test]
async fn test_oidc_token_validation() {
    // Test token validation:
    // 1. Valid tokens
    // 2. Expired tokens
    // 3. Invalid signatures
    // 4. Wrong audience
}

#[tokio::test]
async fn test_jwks_handling() {
    // Test JWKS endpoint:
    // 1. Key rotation
    // 2. Invalid keys
    // 3. Endpoint failures
}
```

3. Database Operation Tests
```rust
#[tokio::test]
async fn test_concurrent_db_operations() {
    // Test concurrent operations:
    // 1. Simultaneous writes
    // 2. Read/write conflicts
    // 3. Transaction isolation
}
```

## Next Steps

1. Fix Database Setup
- Implement proper database setup/cleanup
- Add transaction wrapping
- Fix trigger creation
- Handle concurrent tests

2. Add Missing Tests
- Create authentication test suite
- Add OIDC integration tests
- Implement database operation tests
- Add session management tests

3. Improve Test Infrastructure
- Add test helpers for common operations
- Implement better test isolation
- Add proper error handling
- Improve test documentation

4. Documentation
- Document test patterns
- Add test coverage reports
- Document test database setup
- Add troubleshooting guide