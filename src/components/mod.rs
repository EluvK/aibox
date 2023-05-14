mod dialog_components;
pub use dialog_components::Dialog;

mod message_components;
pub use message_components::{Input, Message};

use crate::gpt::Role;

// Dialogs data struct
#[derive(Debug, Clone, PartialEq)]
pub struct ChatContext {
    pub id: i32,
    pub name: String,
    pub Message: Vec<ChatMessage>,
    // todo add role,
}

// Dialogs IDs data struct
#[derive(Debug)]
pub struct ChatContextID {
    pub current_id: i32,
    pub next_id: i32,
}

// Messages data struct
#[derive(Debug, Clone, PartialEq)]
pub struct ChatMessage {
    pub message_id: i32,
    pub user: Role,
    pub content: String,
    pub time: String,
}

#[derive(PartialEq, dioxus::prelude::Props)]
pub struct RenderMessageProp {
    content: String,
    id: i32,
}

pub trait ChatQuerier {
    async fn query(&self, chatcontext: &ChatContext) -> ChatMessage;
}
