use crate::errors::XaiError;
use crate::xai::{CallRequest, XaiServiceClient};
use tonic::transport::Channel;
use tonic::Request;

#[derive(Clone)]
pub struct Client {
    inner: XaiServiceClient<Channel>,
    api_key: String,
}

impl Client {
    pub fn new(api_key: impl AsRef<str>) -> Result<Self, XaiError> {
        let channel =
            futures::executor::block_on(Channel::from_static("https://api.x.ai").connect())?;
        let client =
            XaiServiceClient::new(channel).send_compressed(tonic::codec::CompressionEncoding::Gzip);
        Ok(Self {
            inner: client,
            api_key: api_key.as_ref().to_string(),
        })
    }

    pub fn call_api(&mut self, req: impl Into<CallRequest>) -> Result<String, XaiError> {
        let mut request = Request::new(req.into());
        request.metadata_mut().insert(
            "authorization",
            format!("Bearer {}", self.api_key)
                .parse()
                .map_err(|_| XaiError::InvalidApiKey)?,
        );

        let resp = futures::executor::block_on(self.inner.call_api(request))?;
        Ok(resp.into_inner().output)
    }
}
