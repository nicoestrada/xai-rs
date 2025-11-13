use thiserror::Error;

#[derive(Debug, Error)]
pub enum XaiError {
    #[error("transport error: {0}")]
    Transport(#[from] tonic::transport::Error),

    #[error("status error: {0}")]
    Status(#[from] tonic::Status),
}
