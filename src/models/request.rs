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
