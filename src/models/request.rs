#[derive(Clone, Debug)]
pub struct RequestModel {
    pub input: String,
}

impl RequestModel {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
        }
    }
}

// Allows `client.call_api(req)` directly
impl From<RequestModel> for crate::xai::CallRequest {
    fn from(m: RequestModel) -> Self {
        Self { input: m.input }
    }
}
