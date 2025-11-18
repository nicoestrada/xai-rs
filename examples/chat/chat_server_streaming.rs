use dotenv::dotenv;
use futures::StreamExt;
use std::env;
use xai_rs::{
    models::{build_request, ChatMessage},
    AsyncClient, XaiError,
};

#[tokio::main]
async fn main() -> Result<(), XaiError> {
    dotenv().ok();
    let api_key = env::var("XAI_API_KEY")
        .map_err(|e| XaiError::Status(tonic::Status::internal(e.to_string())))?;
    let model = env::var("XAI_MODEL").unwrap_or_else(|_| "grok-4-fast-non-reasoning".to_owned());
    let mut client = AsyncClient::new(&api_key).await?;
    let req = build_request(
        vec![ChatMessage::user("Stream this response,please.")],
        &model,
    );
    let mut stream = client.get_completion_chunk(req).await?.into_inner();
    println!("{}:", model);
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        for output in chunk.outputs {
            if let Some(delta) = output.delta {
                print!("{}", delta.content);
            }
        }
    }
    Ok(())
}
