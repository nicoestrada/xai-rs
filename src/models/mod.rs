// src/models/mod.rs
use crate::client::async_client::xai_api::{
    self,
    content,
    tool,
    CodeExecution,
    CollectionsSearch,
    Content,
    DocumentSearch,
    Function,
    GetCompletionsRequest,
    Mcp,
    Message,
    MessageRole,
    Tool,
    ToolChoice,
    ToolMode,
    WebSearch,
    XSearch,
};
use serde_json::Value;

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

    pub fn system(text: impl Into<String>) -> Self {
        Self {
            role: MessageRole::RoleSystem,
            content: text.into(),
        }
    }

    pub fn tool(text: impl Into<String>) -> Self {
        Self {
            role: MessageRole::RoleTool,
            content: text.into(),
        }
    }
}

impl From<ChatMessage> for Message {
    fn from(m: ChatMessage) -> Self {
        Message {
            role: m.role as i32,
            content: vec![Content {
                content: Some(content::Content::Text(m.content)),
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

pub struct ToolDefinition {
    inner: Tool,
}

impl ToolDefinition {
    pub fn function(
        name: impl Into<String>,
        description: impl Into<String>,
        parameters: Value,
    ) -> Self {
        let function = Function {
            name: name.into(),
            description: description.into(),
            strict: false,
            parameters: parameters.to_string(),
        };

        Self {
            inner: Tool {
                tool: Some(tool::Tool::Function(function)),
            },
        }
    }

    pub fn web_search() -> Self {
        Self {
            inner: Tool {
                tool: Some(tool::Tool::WebSearch(WebSearch::default())),
            },
        }
    }

    pub fn x_search() -> Self {
        Self {
            inner: Tool {
                tool: Some(tool::Tool::XSearch(XSearch::default())),
            },
        }
    }

    pub fn code_execution() -> Self {
        Self {
            inner: Tool {
                tool: Some(tool::Tool::CodeExecution(CodeExecution {})),
            },
        }
    }

    pub fn collections_search() -> Self {
        Self {
            inner: Tool {
                tool: Some(tool::Tool::CollectionsSearch(CollectionsSearch::default())),
            },
        }
    }

    pub fn document_search() -> Self {
        Self {
            inner: Tool {
                tool: Some(tool::Tool::DocumentSearch(DocumentSearch::default())),
            },
        }
    }

    pub fn mcp(config: Mcp) -> Self {
        Self {
            inner: Tool {
                tool: Some(tool::Tool::Mcp(config)),
            },
        }
    }

    pub fn into_inner(self) -> Tool {
        self.inner
    }
}

impl From<ToolDefinition> for Tool {
    fn from(definition: ToolDefinition) -> Self {
        definition.into_inner()
    }
}

impl From<Tool> for ToolDefinition {
    fn from(tool: Tool) -> Self {
        Self { inner: tool }
    }
}

pub trait GetCompletionsRequestExt {
    fn with_tools(self, tools: Vec<ToolDefinition>) -> Self;
    fn with_tool_choice(self, tool_choice: ToolChoice) -> Self;
    fn with_tool_mode(self, mode: ToolMode) -> Self;
    fn with_parallel_tool_calls(self, enabled: bool) -> Self;
}

impl GetCompletionsRequestExt for GetCompletionsRequest {
    fn with_tools(mut self, tools: Vec<ToolDefinition>) -> Self {
        self.tools = tools.into_iter().map(Into::into).collect();
        self
    }

    fn with_tool_choice(mut self, tool_choice: ToolChoice) -> Self {
        self.tool_choice = Some(tool_choice);
        self
    }

    fn with_tool_mode(mut self, mode: ToolMode) -> Self {
        self.tool_choice = Some(ToolChoice {
            tool_choice: Some(xai_api::tool_choice::ToolChoice::Mode(mode as i32)),
        });
        self
    }

    fn with_parallel_tool_calls(mut self, enabled: bool) -> Self {
        self.parallel_tool_calls = Some(enabled);
        self
    }
}
