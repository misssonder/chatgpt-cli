use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(default)]
pub struct Message {
    #[serde(default = "default_role")]
    pub role: String,
    pub content: String,
}

fn default_role() -> String {
    "assistant".to_string()
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChatResponse {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<i64>,
    pub choices: Option<Vec<Choice>>,
    pub usage: Option<Usage>,
    pub error: Option<Error>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(default)]
pub struct Choice {
    pub index: i64,
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Usage {
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Error {
    pub message: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub param: Value,
    pub code: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatStreamResponse {
    pub id: Option<String>,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<ChatChoiceDelta>,
    pub usage: Option<Usage>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatChoiceDelta {
    pub delta: Message,
    pub index: i64,
    pub finish_reason: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryMessage {
    pub id: String,
    pub parent_id: Option<String>,
    pub message: Message,
}
