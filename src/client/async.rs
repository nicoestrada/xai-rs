use crate::errors::XaiError;
use crate::xai::{CallRequest, XaiServiceClient};
use tonic::transport::Channel;
use tonic::Request;

#[derive(Clone)]
pub struct AsyncClient {
    inner: XaiServiceClient<Channel>,
    api_key: String,
}

impl AsyncClient {
    pub async fn new(api_key: impl AsRef<str>) -> Result<Self, XaiError> {
        let channel = Channel::from_static("https://api.x.ai").connect().await?;
        let client =
            XaiServiceClient::new(channel).send_compressed(tonic::codec::CompressionEncoding::Gzip);
        Ok(Self {
            inner: client,
            api_key: api_key.as_ref().to_string(),
        })
    }

    pub async fn call_api(&mut self, req: impl Into<CallRequest>) -> Result<String, XaiError> {
        let mut request = Request::new(req.into());
        request.metadata_mut().insert(
            "authorization",
            format!("Bearer {}", self.api_key)
                .parse()
                .map_err(|_| XaiError::InvalidApiKey)?,
        );

        let resp = self.inner.call_api(request).await?;
        Ok(resp.into_inner().output)
    }
}
