use dotenvy::dotenv;
use openagents::server::services::deepseek::{ChatMessage, DeepSeekService, ToolChoice};
use serde_json::json;
use std::env;
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::test]
async fn test_routing_decision() {
    // Initialize logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Load environment variables from .env file
    dotenv().ok();

    // Create a real DeepSeek service instance
    let api_key = env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY must be set in .env file");
    let service = DeepSeekService::new(api_key);

    // Create a dummy tool for routing tests
    let dummy_tool = DeepSeekService::create_tool(
        "dummy_tool".to_string(),
        Some("A dummy tool for testing routing decisions".to_string()),
        json!({
            "type": "object",
            "properties": {
                "message": {
                    "type": "string",
                    "description": "A test message"
                }
            }
        }),
    );

    // System prompt for routing decisions
    let system_message = ChatMessage {
        role: "system".to_string(),
        content: r#"You are a routing assistant that determines whether a user message requires tool usage.
DO NOT USE ANY TOOLS DIRECTLY. Instead, analyze the user's message and respond with a JSON object containing:
1. "needs_tool": boolean - whether any tools are needed
2. "reasoning": string - brief explanation of your decision
3. "suggested_tool": string | null - name of suggested tool if applicable

IMPORTANT: Your response must be a valid JSON object and nothing else.

Example responses:
{
    "needs_tool": true,
    "reasoning": "User is requesting to view a GitHub issue",
    "suggested_tool": "read_github_issue"
}

{
    "needs_tool": false,
    "reasoning": "General chat message that doesn't require tools",
    "suggested_tool": null
}

Remember: Only respond with a JSON object, do not use any tools, and do not add any additional text."#.to_string(),
        tool_call_id: None,
        tool_calls: None,
    };

    // Test cases for routing decisions
    let test_cases = vec![
        (
            "Can you check issue #595?",
            json!({
                "needs_tool": true,
                "reasoning": "User is requesting to view a GitHub issue",
                "suggested_tool": "read_github_issue"
            }),
        ),
        (
            "Hello, how are you today?",
            json!({
                "needs_tool": false,
                "reasoning": "General chat message that doesn't require tools",
                "suggested_tool": null
            }),
        ),
    ];

    for (input, expected_decision) in test_cases {
        info!("\n\nTesting routing for input: {}", input);
        info!("Expected decision: {}", expected_decision);

        // Create messages with system context and user input
        let messages = vec![
            system_message.clone(),
            ChatMessage {
                role: "user".to_string(),
                content: input.to_string(),
                tool_call_id: None,
                tool_calls: None,
            },
        ];

        let (response, _, _) = service
            .chat_with_tools_messages(
                messages,
                vec![dummy_tool.clone()],
                Some(ToolChoice::Auto("auto".to_string())),
                false,
            )
            .await
            .unwrap();

        info!("Response: {}", response);

        // Parse the response as JSON
        let decision: serde_json::Value = serde_json::from_str(&response)
            .expect("Response should be valid JSON");

        // Verify the decision structure
        assert!(decision.get("needs_tool").is_some(), "Missing needs_tool field");
        assert!(decision.get("reasoning").is_some(), "Missing reasoning field");
        assert!(decision.get("suggested_tool").is_some(), "Missing suggested_tool field");

        // Verify the needs_tool field is a boolean
        assert!(decision["needs_tool"].is_boolean(), "needs_tool should be a boolean");

        // Verify the reasoning field is a non-empty string
        assert!(decision["reasoning"].is_string(), "reasoning should be a string");
        assert!(!decision["reasoning"].as_str().unwrap().is_empty(), "reasoning should not be empty");

        // Verify the suggested_tool field is either a string or null
        assert!(
            decision["suggested_tool"].is_null() || decision["suggested_tool"].is_string(),
            "suggested_tool should be a string or null"
        );

        // Compare with expected decision
        assert_eq!(
            decision["needs_tool"], expected_decision["needs_tool"],
            "needs_tool mismatch"
        );
        
        if decision["needs_tool"].as_bool().unwrap() {
            assert_eq!(
                decision["suggested_tool"], expected_decision["suggested_tool"],
                "suggested_tool mismatch for tool-requiring message"
            );
        }
    }
}