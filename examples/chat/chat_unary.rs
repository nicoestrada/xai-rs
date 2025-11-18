use dotenv::dotenv;
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
    let mut client = AsyncClient::new(api_key).await?;
    let req = build_request(vec![ChatMessage::user("Hey, my name is Nico!")], &model);
    let response = client.get_completion(req).await?;
    let body = response.into_inner();
    let result = body
        .outputs
        .first()
        .and_then(|f| f.message.as_ref())
        .ok_or_else(|| XaiError::Status(tonic::Status::internal("Missing message")))?;
    println!("{}", result.content);
    Ok(())
}
