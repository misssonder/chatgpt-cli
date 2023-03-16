use crate::client::ClientBuilderError;
use thiserror::Error;

pub type ChatGPTResult<T> = Result<T, ChatGPTError>;

#[derive(Error, Debug)]
pub enum ChatGPTError {
    #[error("reqwest_error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("serde_error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("chatgpt_error: {0}")]
    ChatGtp(String),
    #[error("invalid_header: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error("builder_error: {0}")]
    Builder(#[from] ClientBuilderError),
    #[error("event_stream: {0}")]
    EventStream(#[from] eventsource_stream::EventStreamError<reqwest::Error>),
}
