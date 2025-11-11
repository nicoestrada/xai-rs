## xAI Rust SDK (`xai-rs`)

The **xAI Rust SDK** is a **gRPC-based** library for interacting with xAI's APIs in Rust.  

It mirrors the public API of the official [xAI Python SDK](https://github.com/xai-org/xai-sdk-python), providing **synchronous** (`Client`) and **asynchronous** (`AsyncClient`) interfaces.

-Uses the `tonic` crate for gRPC communication.

-Provides easy-to-use methods for common xAI API operations.

-Supports both sync and async programming models.

-Includes comprehensive error handling and type safety.

-Well-documented with examples for quick integration.

-Actively maintained to keep up with xAI API changes.

#### ***This is a work in progress and not all features of the xAI Python SDK are implemented yet.***

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
xai-rs = "0.0.1"
tonic = "0.7"
tokio = { version = "1", features = ["full"] } # For async support
```

## Usage

### Synchronous Client

```rust
use xai_rs::{Client, RequestModel, XaiError};

fn main() -> Result<(), XaiError> {
    let mut client = Client::new("sk-...")?;
    let req = RequestModel::new("Hello, xAI!");
    let resp = client.call_api(req)?;
    println!("Response: {resp}");
    Ok(())
}
```

### Asynchronous Client

```rust
use xai_rs::{AsyncClient, RequestModel, XaiError};
use tokio;

#[tokio::main]
async fn main() -> Result<(), XaiError> {
    let mut client = AsyncClient::new("sk-...").await?;
    let req = RequestModel::new("Hello, xAI!");
    let resp = client.call_api(req).await?;
    println!("Response: {resp}");
    Ok(())
}
```

## Documentation

Not available at the moment.

## Contributing

Contributions are welcome! Please open issues or submit pull requests on the GitHub repository.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
