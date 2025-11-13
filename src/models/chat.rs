// src/models/chat.rs
use xai_api::chat::{Content as ProtoContent, MessageRole};

#[derive(Clone, Debug)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl From<ChatMessage> for crate::xai_api::chat::Message {
    fn from(msg: ChatMessage) -> Self {
        Self {
            role: match msg.role.as_str() {
                "user" => MessageRole::RoleUser as i32,
                "assistant" => MessageRole::RoleAssistant as i32,
                "system" => MessageRole::RoleSystem as i32,
                _ => MessageRole::InvalidRole as i32,
            },
            content: vec![ProtoContent::Text(msg.content)],
            ..Default::default()
        }
    }
}
