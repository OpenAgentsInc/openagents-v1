use openagents::agents::{AgentManager, InstanceStatus};
use serde_json::json;
use sqlx::PgPool;
use std::env;

async fn setup_test_db() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url).await.unwrap()
}

#[tokio::test]
async fn test_agent_creation_and_validation() {
    let pool = setup_test_db().await;
    let manager = AgentManager::new(pool.clone());

    // Test valid agent creation
    let result = manager
        .create_agent(
            "Test Agent",
            "A test agent",
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            json!({
                "version": "1.0.0",
                "memory_limit": 512,
                "cpu_limit": 100.0,
                "max_instances": 2
            }),
        )
        .await;

    assert!(result.is_ok());
    let agent = result.unwrap();
    assert_eq!(agent.name, "Test Agent");
    assert!(agent.enabled);

    // Test invalid pubkey
    let result = manager
        .create_agent(
            "Invalid Agent",
            "Invalid pubkey",
            "invalid",
            json!({}),
        )
        .await;
    assert!(result.is_err());

    // Test invalid config
    let result = manager
        .create_agent(
            "Invalid Config",
            "Bad limits",
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            json!({
                "memory_limit": 0,
                "cpu_limit": -1.0
            }),
        )
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_instance_lifecycle() {
    let pool = setup_test_db().await;
    let manager = AgentManager::new(pool.clone());

    // Create test agent
    let agent = manager
        .create_agent(
            "Lifecycle Test",
            "Testing instance lifecycle",
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            json!({
                "max_instances": 2,
                "memory_limit": 512,
                "cpu_limit": 100.0,
                "initial_state": {"status": "ready"}
            }),
        )
        .await
        .unwrap();

    // Create instance
    let instance = manager.create_instance(agent.id).await.unwrap();
    assert!(matches!(instance.status, InstanceStatus::Starting));

    // Update status
    let result = manager
        .update_instance_status(instance.id, InstanceStatus::Running)
        .await
        .unwrap();
    assert!(result);

    // Verify state
    let state = manager.get_instance_state(instance.id).await.unwrap();
    assert!(state.is_some());
    assert_eq!(state.unwrap()["status"], "ready");

    // Update metrics
    manager
        .update_instance_metrics(
            instance.id,
            json!({
                "memory_usage": 256,
                "cpu_usage": 50.0,
                "task_count": 1,
                "error_count": 0,
                "uptime": 60
            }),
        )
        .await
        .unwrap();

    // Check resource limits
    let within_limits = manager.check_resource_limits(instance.id).await.unwrap();
    assert!(within_limits);
}

#[tokio::test]
async fn test_instance_limits() {
    let pool = setup_test_db().await;
    let manager = AgentManager::new(pool.clone());

    // Create agent with max 1 instance
    let agent = manager
        .create_agent(
            "Limited Agent",
            "Testing instance limits",
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            json!({
                "max_instances": 1,
                "memory_limit": 512,
                "cpu_limit": 100.0
            }),
        )
        .await
        .unwrap();

    // First instance should succeed
    let instance1 = manager.create_instance(agent.id).await.unwrap();
    assert!(matches!(instance1.status, InstanceStatus::Starting));

    // Second instance should fail
    let result = manager.create_instance(agent.id).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_resource_monitoring() {
    let pool = setup_test_db().await;
    let manager = AgentManager::new(pool);

    // Create agent with strict limits
    let agent = manager
        .create_agent(
            "Resource Test",
            "Testing resource monitoring",
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            json!({
                "memory_limit": 256,
                "cpu_limit": 50.0
            }),
        )
        .await
        .unwrap();

    let instance = manager.create_instance(agent.id).await.unwrap();

    // Test within limits
    manager
        .update_instance_metrics(
            instance.id,
            json!({
                "memory_usage": 128,
                "cpu_usage": 25.0,
                "task_count": 1,
                "error_count": 0,
                "uptime": 300
            }),
        )
        .await
        .unwrap();

    assert!(manager.check_resource_limits(instance.id).await.unwrap());

    // Test exceeding limits
    manager
        .update_instance_metrics(
            instance.id,
            json!({
                "memory_usage": 512,
                "cpu_usage": 75.0,
                "task_count": 1,
                "error_count": 0,
                "uptime": 360
            }),
        )
        .await
        .unwrap();

    assert!(!manager.check_resource_limits(instance.id).await.unwrap());
}

#[tokio::test]
async fn test_state_management() {
    let pool = setup_test_db().await;
    let manager = AgentManager::new(pool);

    // Create agent
    let agent = manager
        .create_agent(
            "State Test",
            "Testing state management",
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            json!({
                "initial_state": {
                    "counter": 0,
                    "status": "initialized"
                }
            }),
        )
        .await
        .unwrap();

    let instance = manager.create_instance(agent.id).await.unwrap();

    // Verify initial state
    let state = manager.get_instance_state(instance.id).await.unwrap().unwrap();
    assert_eq!(state["counter"], 0);
    assert_eq!(state["status"], "initialized");

    // Update state
    manager
        .set_instance_state(
            instance.id,
            json!({
                "counter": 1,
                "status": "running",
                "data": {"key": "value"}
            }),
        )
        .await
        .unwrap();

    // Verify updated state
    let state = manager.get_instance_state(instance.id).await.unwrap().unwrap();
    assert_eq!(state["counter"], 1);
    assert_eq!(state["status"], "running");
    assert_eq!(state["data"]["key"], "value");
}