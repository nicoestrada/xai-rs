## xAI Rust SDK (`xai-rs`)

The **xAI Rust SDK** is a gRPC client that lets you talk to xAI's API in Rust. It currently exposes an **asynchronous** client built on [`tonic`](https://github.com/hyperium/tonic).

### Status
- Early preview focused on unary chat completions (`GetCompletion`) and streaming chunks (`GetCompletionChunk`)
- Only asynchronous usage is supported today

### Features
- Async `AsyncClient` that authenticates with the xAI API
- Unary (`get_completion`) and server‑streaming (`get_completion_chunk`) helpers
- Generated protobuf types from xAI's published `.proto` files
- Helper builders to construct chat messages (`ChatMessage`) and requests (`build_request`)

### Installation

```bash
cargo add xai-rs
```

Alternatively, add this to your `Cargo.toml`:

```toml
[dependencies]
xai-rs = "0.0.5"
```

### Examples
- Simple chat completion example in the `examples/` dir
- `examples/chat.rs` – single response via `GetCompletion`
- `examples/chat_streaming.rs` – server-streaming chunks via `GetCompletionChunk`
- Run them with `cargo run --example chat` or `cargo run --example chat_streaming` after setting `XAI_API_KEY` (and optionally `XAI_MODEL`)

#### ***This is a work in progress and not all features of the xAI Python SDK are implemented yet.***

## Contributing

Contributions are welcome! Please open issues or submit pull requests on the GitHub repository.  

Open gaps include, additional RPCs (images, models, telemetry, etc.), a sync client, retries, and richer error types.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
