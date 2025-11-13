// src/client/async.rs

use crate::errors::XaiError;
use tonic::{
    transport::{Channel, ClientTlsConfig},
    Request,
};

pub mod xai_api {
    tonic::include_proto!("xai_api");
}

use xai_api::chat_client::ChatClient;

#[derive(Clone)]
pub struct AsyncClient {
    inner: ChatClient<Channel>,
    api_key: String,
}

impl AsyncClient {
    pub async fn new(api_key: impl Into<String>) -> Result<Self, XaiError> {
        let api_key = api_key.into();

        let tls = ClientTlsConfig::new()
            .with_native_roots()
            .domain_name("api.x.ai");
        let channel = Channel::from_static("https://api.x.ai")
            .tls_config(tls)?
            .connect()
            .await?;

        let client = ChatClient::new(channel);

        Ok(Self {
            inner: client,
            api_key,
        })
    }

    // Non-streaming completion
    pub async fn get_completion(
        &mut self,
        req: xai_api::GetCompletionsRequest,
    ) -> Result<tonic::Response<xai_api::GetChatCompletionResponse>, XaiError> {
        // Wrap the request in a tonic::Request to add metadata
        let mut request = Request::new(req);
        request.metadata_mut().insert(
            "authorization",
            format!("Bearer {}", self.api_key).parse().unwrap(),
        );

        Ok(self.inner.get_completion(request).await?)
    }

    // Streaming completion (optional)
    // pub async fn get_completion_chunk(
    //     &mut self,
    //     req: xai_api::GetCompletionsRequest,
    // ) -> Result<tonic::Response<tonic::Streaming<xai_api::GetChatCompletionChunk>>, XaiError> {
    //     let mut request = Request::new(req);
    //     request
    //         .metadata_mut()
    //         .insert("authorization", format!("Bearer {}", self.api_key).parse().unwrap());
    //
    //     Ok(self.inner.get_completion_chunk(request).await?)
    // }
}
