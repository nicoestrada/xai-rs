## xAI Rust SDK (`xai-rs`)

The **xAI Rust SDK** is a gRPC client that lets you talk to xAI's chat models in Rust. It currently exposes an **asynchronous** client built on [`tonic`](https://github.com/hyperium/tonic).

### Status
- Early preview focused on chat completions (`GetCompletion`)
- Only asynchronous usage is supported today
- Expect breaking changes while the API surface grows

### Features
- Async `AsyncClient` that authenticates with the xAI API
- Generated protobuf types from xAI's published `.proto` files
- Helper builders to construct chat messages (`ChatMessage`) and requests (`build_request`)

#### ***This is a work in progress and not all features of the xAI Python SDK are implemented yet.***

## Usage

```rust
use dotenv::dotenv;
use std::env;
use xai_rs::{
    AsyncClient, XaiError,
    models::{ChatMessage, build_request},
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
```

## Contributing

Contributions are welcome! Please open issues or submit pull requests on the GitHub repository.  
Open gaps include streaming responses, additional RPCs (images, models, telemetry, etc.), a sync client, retries, and richer error types.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
