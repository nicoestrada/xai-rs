// src/models/mod.rs
use crate::client::async_client::xai_api::{Content, GetCompletionsRequest, Message, MessageRole};

#[derive(Debug)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
}

impl ChatMessage {
    pub fn user(text: impl Into<String>) -> Self {
        Self {
            role: MessageRole::RoleUser,
            content: text.into(),
        }
    }
    pub fn assistant(text: impl Into<String>) -> Self {
        Self {
            role: MessageRole::RoleAssistant,
            content: text.into(),
        }
    }
}

impl From<ChatMessage> for Message {
    fn from(m: ChatMessage) -> Self {
        Message {
            role: m.role as i32,
            content: vec![Content {
                content: Some(
                    crate::client::async_client::xai_api::content::Content::Text(m.content),
                ),
            }],
            ..Default::default()
        }
    }
}

pub fn build_request(messages: Vec<ChatMessage>, model: &str) -> GetCompletionsRequest {
    GetCompletionsRequest {
        messages: messages.into_iter().map(Into::into).collect(),
        model: model.to_string(),
        ..Default::default()
    }
}
