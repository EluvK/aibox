use serde::{Deserialize, Serialize};

use crate::components::ChatMessage;

// Roles enum
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(super) struct GPTMessages {
    role: Role,
    content: String,
}

impl GPTMessages {
    pub fn into_contnet(self) -> String {
        self.content
    }
    pub fn new_system_message(content: String) -> Self {
        Self {
            role: Role::System,
            content,
        }
    }
    pub fn new_user_message(content: String) -> Self {
        Self {
            role: Role::User,
            content,
        }
    }
    pub fn new_assist_message(content: String) -> Self {
        Self {
            role: Role::Assistant,
            content,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GPTPostMessage {
    model: String,
    messages: Vec<GPTMessages>,
    max_tokens: u64,
    temperature: f64,
}

impl Default for GPTPostMessage {
    fn default() -> Self {
        Self {
            model: String::from("gpt-3.5-turbo"),
            messages: Vec::new(),
            max_tokens: 1000,
            temperature: 0.1,
        }
    }
}

impl GPTPostMessage {
    #[cfg(test)]
    pub fn test_new(message: String) -> Self {
        GPTPostMessage {
            messages: vec![GPTMessages::new_user_message(message)],
            ..Default::default()
        }
    }
    pub fn new_with_context(context: Vec<ChatMessage>, model: String) -> Self {
        GPTPostMessage {
            model,
            messages: context
                .into_iter()
                .map(|message| GPTMessages {
                    role: message.user,
                    content: message.content,
                })
                .collect(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct GPTUsage {
    prompt_tokens: i64,
    completion_tokens: i64,
    total_tokens: i64,
}

impl std::fmt::Display for GPTUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Prompt tokens used: {}, Completion tokens used: {}, Total tokens used: {}",
            self.prompt_tokens, self.completion_tokens, self.total_tokens
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct GPTRecvChoice {
    message: GPTMessages,

    // https://platform.openai.com/docs/guides/chat/response-format
    #[serde(skip)]
    _finish_reason: String, // not important actually, and it will be null but not "null", strange...
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GPTRecvMessage {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<GPTRecvChoice>,
    usage: GPTUsage,
}

impl GPTRecvMessage {
    pub(super) fn get_return_msg(&self) -> Option<GPTMessages> {
        println!("[COST] finish chatgpt-api call {}", self.usage);
        if self.choices.is_empty() {
            None
        } else {
            Some(self.choices[0].message.clone())
        }
    }
}
