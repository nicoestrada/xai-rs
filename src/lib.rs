//! xAI Rust SDK – gRPC client for the official xAI API.

pub mod xai {
    tonic::include_proto!("xai");

    pub use xai_service_client::XaiServiceClient;
}

pub mod client;
pub mod errors;
pub mod models;

pub use client::{AsyncClient, Client};
pub use errors::XaiError;
pub use models::RequestModel;

// Generated protobuf code
include!(concat!(env!("OUT_DIR"), "/xai.rs"));
