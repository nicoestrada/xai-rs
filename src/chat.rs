// src/chat.rs
use std::io;
use xai_rs::{
    models::{build_request, ChatMessage},
    AsyncClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("XAI_API_KEY")?;
    let mut client = AsyncClient::new(api_key).await?;
    let model =
        std::env::var("XAI_MODEL").unwrap_or_else(|_| "grok-4-fast-non-reasoning".to_string());

    let msgs = vec![ChatMessage::user(
        "Explain quantum entanglement in one sentence.",
    )];
    let req = build_request(msgs, &model);

    let resp = client.get_completion(req).await?;
    let message = resp
        .get_ref()
        .outputs
        .first()
        .and_then(|output| output.message.as_ref())
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "missing completion message"))?;

    println!("{}: {}", model, message.content);
    Ok(())
}
