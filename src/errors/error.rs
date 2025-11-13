use thiserror::Error;
use tonic::Status;

#[derive(Error, Debug)]
pub enum XaiError {
    #[error("Transport error: {0}")]
    Transport(#[from] tonic::transport::Error),

    #[error("gRPC status: {0}")]
    Status(#[from] Status),

    #[error("Invalid API key")]
    InvalidApiKey,
}
