use thiserror::Error;

#[derive(Error, Debug)]
pub enum XaiError {
    #[error("Transport error: {0}")]
    Transport(#[from] tonic::transport::Error),

    #[error("gRPC status: {0}")]
    Status(#[from] tonic::Status),

    #[error("Invalid API key")]
    InvalidApiKey,
}
