use dotenv::dotenv;
use futures::StreamExt;
use std::env;
use xai_rs::client::async_client::xai_api::tool_call;
use xai_rs::{
    models::{build_request, ChatMessage, GetCompletionsRequestExt, ToolDefinition},
    AsyncClient, XaiError,
};

#[tokio::main]
async fn main() -> Result<(), XaiError> {
    dotenv().ok();
    let api_key = env::var("XAI_API_KEY")
        .map_err(|e| XaiError::Status(tonic::Status::internal(e.to_string())))?;

    // grok-4-fast is strongly recommended for agentic tool calling
    let model = env::var("XAI_MODEL").unwrap_or_else(|_| "grok-4-fast".to_owned());

    let mut client = AsyncClient::new(&api_key).await?;

    let req = build_request(
        vec![ChatMessage::user(
            "Write a small script to count the number of words in a given text. Then use a web search to find out how many words are in the US Declaration of Independence and run the script on that text.",
        )],
        &model,
    )
    // you can modify tool selection for agent here. See models/mod.rs for available tools
    // keep in mind tradeoffs for giving access
    .with_tools(vec![
        ToolDefinition::web_search(),
        ToolDefinition::x_search(),
        ToolDefinition::code_execution(),
        //ToolDefinition::document_search(),
        //ToolDefinition::collection_search(),
        //ToolDefinition::mcp(),
    ]);

    let mut stream = client.get_completion_chunk(req).await?.into_inner();
    println!("{}:", model);
    let mut printed_header = false;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;

        for output in chunk.outputs {
            if let Some(delta) = output.delta {
                for tool_call in delta.tool_calls {
                    println!("Running {:#?} tool...", tool_call.r#type());
                    if let Some(tool_call::Tool::Function(function)) = tool_call.tool {
                        println!(
                            "\nCalling tool: {} with arguments: {}",
                            function.name, function.arguments
                        );
                    }
                }

                if !delta.content.is_empty() {
                    if !printed_header {
                        println!("\n\nFinal Response:");
                        printed_header = true;
                    }
                    print!("{}", delta.content);
                }
            }
        }
    }

    println!();
    Ok(())
}
