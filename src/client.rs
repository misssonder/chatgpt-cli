use crate::error::{ChatGPTError, ChatGPTResult};
use crate::prompt::{Prompt, PROMPTS};
use crate::tokenizer::encode;
use crate::types::{ChatRequest, ChatResponse, ChatStreamResponse, HistoryMessage, Message};
use derive_builder::Builder;
use eventsource_stream::Eventsource;
use futures_util::{Stream, StreamExt, TryStreamExt};
use lazy_static::lazy_static;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use std::collections::HashMap;
use std::future;
use std::string::ToString;
lazy_static! {
    static ref API_URL: String = String::from("https://api.openai.com/v1/chat/completions");
}
#[allow(dead_code)]
#[derive(Builder)]
pub struct Client {
    #[builder(private)]
    client: reqwest::Client,
    #[builder(setter(custom))]
    api_key: String,
    #[builder(default = "\"gpt-3.5-turbo\".to_string()")]
    pub model: String,
    #[builder(default = "4096")]
    max_token: usize,
    #[builder(default = "PROMPTS.get(&Prompt::Default.to_string()).unwrap().into()")]
    system_message: String,
    #[builder(setter(skip))]
    history_messages: HashMap<String, HistoryMessage>,
}

impl ClientBuilder {
    pub fn api_key(&mut self, api_key: String) -> &mut Self {
        let mut new = self;
        new.api_key = Some(api_key.clone());
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
        );
        new.client = Some(
            reqwest::ClientBuilder::new()
                .default_headers(headers)
                .build()
                .unwrap(),
        );
        new
    }
}

impl Client {
    pub fn new(api_key: String) -> ChatGPTResult<Self> {
        Ok(ClientBuilder::default().api_key(api_key).build()?)
    }

    async fn build_messages(&mut self, text: String, parent_id: Option<String>) -> Vec<Message> {
        let mut messages = Vec::new();
        messages.push(Message {
            role: "system".to_string(),
            content: self.system_message.clone(),
        });
        messages.push(Message {
            role: "user".to_string(),
            content: text,
        });
        match parent_id {
            None => messages,
            Some(mut parent_id) => {
                loop {
                    match self.history_messages.get(&*parent_id) {
                        None => {
                            break;
                        }
                        Some(parent_message) => {
                            let contents: Vec<_> =
                                messages.iter().map(|m| m.content.clone()).collect();
                            let total_token = encode(
                                format!(
                                    "{}{}",
                                    contents.join(""),
                                    parent_message.message.content.clone().as_str()
                                )
                                .as_str(),
                            )
                            .len();
                            if total_token > self.max_token {
                                break;
                            };
                            messages.insert(1, parent_message.message.clone());
                            match parent_message.clone().parent_id {
                                None => break,
                                Some(id) => parent_id = id,
                            }
                        }
                    }
                }
                messages
            }
        }
    }

    async fn get_chat_stream(
        &mut self,
        req: ChatRequest,
    ) -> ChatGPTResult<impl Stream<Item = ChatGPTResult<ChatStreamResponse>>> {
        let stream = self
            .client
            .post(API_URL.clone())
            .json(&req)
            .send()
            .await?
            .bytes_stream()
            .eventsource();
        Ok(stream
            .take_while(|chunk| match chunk {
                Ok(message) => future::ready(message.data != "[DONE]"),
                Err(_) => future::ready(false),
            })
            .map(|chunk| {
                let message = chunk?;
                let resp: ChatStreamResponse = serde_json::from_str(&message.data)?;
                Ok(resp)
            }))
    }

    pub async fn send_message(
        &mut self,
        text: String,
        parent_id: Option<String>,
    ) -> ChatGPTResult<(String, String)> {
        let req = ChatRequest {
            model: self.model.clone(),
            messages: self.build_messages(text.clone(), parent_id.clone()).await,
            stream: None,
        };
        let resp: ChatResponse = self
            .client
            .post(API_URL.clone())
            .json(&req)
            .send()
            .await?
            .json()
            .await?;
        if let Some(err) = resp.error {
            return Err(ChatGPTError::ChatGtp(err.message));
        }

        match resp.choices {
            None => Err(ChatGPTError::ChatGtp(format!("response choices is none"))),
            Some(choices) => {
                let req_id = uuid::Uuid::new_v4().to_string();
                let resp_id = resp
                    .id
                    .ok_or(ChatGPTError::ChatGtp(format!("response id is none")))?;
                self.history_messages.insert(
                    req_id.to_string(),
                    HistoryMessage {
                        id: req_id.clone(),
                        parent_id,
                        message: Message {
                            role: "user".to_string(),
                            content: text,
                        },
                    },
                );
                self.history_messages.insert(
                    resp_id.clone(),
                    HistoryMessage {
                        id: resp_id.clone(),
                        parent_id: Some(req_id),
                        message: choices[0].clone().message,
                    },
                );
                Ok((choices[0].to_owned().message.content, resp_id))
            }
        }
    }

    pub async fn send_message_stream(
        &mut self,
        text: String,
        parent_id: Option<String>,
    ) -> ChatGPTResult<impl Stream<Item = ChatGPTResult<String>>> {
        let req = ChatRequest {
            model: self.model.clone(),
            messages: self.build_messages(text.clone(), parent_id.clone()).await,
            stream: Some(true),
        };
        let stream = self.get_chat_stream(req).await?.into_stream();
        Ok(stream.map(|stream| Ok(stream?.choices[0].to_owned().delta.content)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_chat_stream() -> ChatGPTResult<()> {
        let mut client = Client::new("".into())?;
        let req = ChatRequest {
            model: client.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: "who are you".to_string(),
            }],
            stream: Some(true),
        };
        let mut stream = client.get_chat_stream(req).await?;
        while let Some(message) = stream.next().await {
            println!("{:?}", message)
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_send_message() -> ChatGPTResult<()> {
        let mut client = Client::new("".to_string()).unwrap();
        let (resp, resp_id) = client
            .send_message(
                "From now on, your answer must start with 😄".to_string(),
                None,
            )
            .await?;
        println!("{}", resp);
        let (resp, resp_id) = client
            .send_message("who are you".to_string(), Some(resp_id))
            .await?;
        println!("{}", resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_build_messages() {
        let mut client = Client::new("".to_string()).unwrap();
        let mut ids = Vec::new();
        let mut history_messages = Vec::new();
        let mut parent_id = None;
        for _i in 0..10 {
            let id = uuid::Uuid::new_v4().to_string();
            ids.push(id.clone());
            history_messages.push(HistoryMessage {
                id: id.clone(),
                parent_id,
                message: Message {
                    role: "user".to_string(),
                    content: id.clone(),
                },
            });
            parent_id = Some(id);
        }

        for history_message in history_messages {
            client
                .history_messages
                .insert(history_message.id.clone(), history_message);
        }
        let messages = client
            .build_messages("hello".to_string(), ids.last().map(|x| x.to_string()))
            .await;
        println!("{:#?}", messages);
        assert_eq!(messages.len(), 12)
    }

    #[tokio::test]
    async fn test_send_message_stream() -> ChatGPTResult<()> {
        let mut client = Client::new("".to_string())?;
        let mut stream = client
            .send_message_stream("hello! who are you?".to_string(), None)
            .await?;
        let mut text = String::from("");
        while let Some(message) = stream.next().await {
            text = format!("{}{}", text, message?);
            println!("{}", text)
        }
        println!("..");
        Ok(())
    }
}
