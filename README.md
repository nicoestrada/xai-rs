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
use xai_rs::{
    models::{build_request, ChatMessage},
    AsyncClient, XaiError,
};

#[tokio::main]
async fn main() -> Result<(), XaiError> {
    // Expect an xAI API key in the environment.
    let api_key = std::env::var("XAI_API_KEY")?;

    // Build the client and request.
    let mut client = AsyncClient::new(api_key).await?;
    let request = build_request(
        vec![ChatMessage::user("Hello, xAI!")],
        std::env::var("XAI_MODEL").as_deref().unwrap_or("grok-code-fast-1"),
    );

    let response = client.get_completion(request).await?;
    if let Some(message) = response
        .get_ref()
        .outputs
        .first()
        .and_then(|output| output.message.as_ref())
    {
        println!("{}", message.content);
    }

    Ok(())
}
```

## Contributing

Contributions are welcome! Please open issues or submit pull requests on the GitHub repository.  
Open gaps include streaming responses, additional RPCs (images, models, telemetry, etc.), a sync client, retries, and richer error types.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
