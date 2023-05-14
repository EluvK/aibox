use std::rc::Rc;

use thiserror::Error;

use crate::{
    components::{ChatContext, ChatMessage, ChatQuerier},
    config::Config,
};

use super::{
    types::{GPTPostMessage, GPTRecvMessage},
    Role,
};

// temp type for trait.
#[derive(Debug, Clone, PartialEq)]
pub struct Openaigpt {
    config: Rc<Config>,
}

static OPENAIAPIURL: &str = "https://api.openai.com/v1/chat/completions";

impl ChatQuerier for Openaigpt {
    async fn query(&self, chatcontext: &ChatContext) -> ChatMessage {
        let post_message = GPTPostMessage::new_with_context(
            chatcontext.Message.clone(),
            String::from("gpt-3.5-turbo"),
        );

        let r = match self.post_gpt_request(post_message).await {
            Ok(gpt_recv_msg) => {
                return match gpt_recv_msg.get_return_msg() {
                    Some(msg) => {
                        ChatMessage {
                            message_id: rand::random::<i32>(), // todo
                            user: Role::Assistant,
                            content: msg.into_contnet(),
                            time: "2020-01-01 00:00:00".to_string(), //todo
                        }
                    }
                    None => {
                        println!("DEBUG: {:?}", gpt_recv_msg);
                        ChatMessage {
                            message_id: rand::random::<i32>(), // todo
                            user: Role::System,
                            content: String::from("Error"),
                            time: "2020-01-01 00:00:00".to_string(), //todo
                        }
                    }
                };
            }
            Err(e) => {
                println!("DEBUG: {:?}", e);
                ChatMessage {
                    message_id: rand::random::<i32>(), // todo
                    user: Role::System,
                    content: String::from("Error") + e.to_string().as_str(),
                    time: "2020-01-01 00:00:00".to_string(), //todo
                }
            }
        };

        r
    }
}

impl Openaigpt {
    pub fn new(config: Rc<Config>) -> Self {
        Self { config }
    }

    async fn post_gpt_request(
        &self,
        post_message: GPTPostMessage,
    ) -> Result<GPTRecvMessage, GPTError> {
        let body = serde_json::to_string(&post_message).unwrap();
        println!("body:{}", body);

        let c = match self.config.proxy_addr() {
            Some(addr) if !addr.is_empty() => {
                let proxy = reqwest::Proxy::https(addr)?;
                reqwest::Client::builder().proxy(proxy).build()?
            }
            _ => reqwest::Client::new(),
        };

        let resp_text = c
            .post(OPENAIAPIURL)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.config.api_key()))
            .body(body)
            .send()
            .await?
            .text()
            .await?;

        let gpt_recv_msg = serde_json::from_str::<GPTRecvMessage>(&resp_text).map_err(|e| {
            println!("DEBUG: {:?}", resp_text);

            // try get GPT error message
            lazy_static::lazy_static! {
                static ref RE: regex::Regex = regex::Regex::new(
                    r#"message": "(.*?)""#,
                ).unwrap();
            }
            if let Some(str) = RE.find(&resp_text) {
                GPTError::GPTReturnError(str.as_str().into())
            } else {
                GPTError::GPTReturnError(e.to_string())
            }
        })?;

        Ok(gpt_recv_msg)
    }
}

// gpt error
#[derive(Debug, Error)]
pub enum GPTError {
    #[error("Https request error {0}")]
    HttpsRequestError(String),

    #[error("GPT return error {0}")]
    GPTReturnError(String),
}

impl From<reqwest::Error> for GPTError {
    fn from(e: reqwest::Error) -> Self {
        GPTError::HttpsRequestError(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    async fn do_test_send_req() {
        let config = Config::test_new();
        let config = Rc::new(config);
        let gpt = Openaigpt::new(config);
        let return_message = gpt
            .post_gpt_request(GPTPostMessage::test_new("hello".into()))
            .await;
        println!("return_message:{:?}", return_message);
    }

    #[test]
    fn test_send_req() {
        tokio_test::block_on(do_test_send_req())
    }
}
