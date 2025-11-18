use serde_json::json;
use xai_rs::client::async_client::xai_api::{
    content, tool, tool_choice, MessageRole, ToolChoice, ToolMode,
};
use xai_rs::models::{build_request, ChatMessage, GetCompletionsRequestExt, ToolDefinition};

#[test]
fn build_request_wraps_messages_with_expected_roles() {
    let request = build_request(
        vec![
            ChatMessage::system("system prompt"),
            ChatMessage::user("hello"),
            ChatMessage::assistant("hi there"),
            ChatMessage::tool("tool output"),
        ],
        "grok-test",
    );

    assert_eq!(request.model, "grok-test");
    assert_eq!(request.messages.len(), 4);

    let expected_roles = [
        MessageRole::RoleSystem,
        MessageRole::RoleUser,
        MessageRole::RoleAssistant,
        MessageRole::RoleTool,
    ];
    let expected_text = ["system prompt", "hello", "hi there", "tool output"];

    for ((message, role), text) in request
        .messages
        .iter()
        .zip(expected_roles.into_iter())
        .zip(expected_text)
    {
        assert_eq!(message.role, role as i32);

        let Some(content::Content::Text(value)) =
            message.content.first().and_then(|c| c.content.as_ref())
        else {
            panic!("message content missing");
        };

        assert_eq!(value, text);
    }
}

#[test]
fn request_extensions_attach_function_tools() {
    let schema = json!({
        "type": "object",
        "properties": {
            "query": { "type": "string" }
        },
        "required": ["query"]
    });

    let request =
        build_request(vec![ChatMessage::user("Search the web")], "grok-test").with_tools(vec![
            ToolDefinition::function(
                "search_web",
                "Search the public internet for results",
                schema.clone(),
            ),
        ]);

    assert_eq!(request.tools.len(), 1);

    let Some(tool::Tool::Function(function)) = request.tools[0].tool.as_ref() else {
        panic!("expected function tool definition");
    };

    assert_eq!(function.name, "search_web");
    assert_eq!(
        function.description,
        "Search the public internet for results"
    );
    assert_eq!(function.parameters, schema.to_string());
}

#[test]
fn request_extensions_control_tool_decisions() {
    let forced_function = ToolChoice {
        tool_choice: Some(tool_choice::ToolChoice::FunctionName(
            "web_lookup".to_string(),
        )),
    };

    let forced_tool_request = build_request(
        vec![ChatMessage::user("Call the search tool for me")],
        "grok-test",
    )
    .with_tool_choice(forced_function)
    .with_parallel_tool_calls(false);

    let choice = forced_tool_request
        .tool_choice
        .as_ref()
        .and_then(|choice| choice.tool_choice.as_ref())
        .expect("tool choice missing");

    match choice {
        tool_choice::ToolChoice::FunctionName(name) => assert_eq!(name, "web_lookup"),
        _ => panic!("expected function name tool choice"),
    }
    assert_eq!(forced_tool_request.parallel_tool_calls, Some(false));

    let mode_request = build_request(vec![ChatMessage::user("Use tools if needed")], "grok-test")
        .with_tool_mode(ToolMode::Required)
        .with_parallel_tool_calls(true);

    let choice = mode_request
        .tool_choice
        .as_ref()
        .and_then(|choice| choice.tool_choice.as_ref())
        .expect("tool mode missing");

    match choice {
        tool_choice::ToolChoice::Mode(mode) => {
            assert_eq!(*mode, ToolMode::Required as i32);
        }
        _ => panic!("expected tool mode choice"),
    }
    assert_eq!(mode_request.parallel_tool_calls, Some(true));
}
