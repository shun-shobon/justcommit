use reqwest::{header, Client};
use serde::ser::SerializeMap as _;
use serde::{Deserialize, Serialize};

const OPENAI_COMPLETIONS_URL: &str = "https://api.openai.com/v1/chat/completions";

pub async fn send_request(
    openai_token: &str,
    functions: Option<&[Function<'_>]>,
    function_call: FunctionCallRequest<'_>,
    messages: &[Message],
) -> anyhow::Result<Message> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Content-Type",
        header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        "Authorization",
        header::HeaderValue::from_str(&format!("Bearer {openai_token}"))?,
    );

    let request_body = CompletionRequest {
        model: Model::Gpt35Turbo0613,
        messages,
        functions,
        function_call,
    };

    let response = Client::new()
        .post(OPENAI_COMPLETIONS_URL)
        .headers(headers)
        .json(&request_body)
        .send()
        .await?
        .json::<CompletionResponse>()
        .await?;

    let message = response.choices[0].message.clone();
    Ok(message)
}

#[derive(Debug, Clone, Serialize)]
struct CompletionRequest<'a> {
    model: Model,
    messages: &'a [Message],
    #[serde(skip_serializing_if = "Option::is_none")]
    functions: Option<&'a [Function<'a>]>,
    function_call: FunctionCallRequest<'a>,
}

#[derive(Debug, Clone, Deserialize)]
struct CompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Clone, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Clone, Serialize)]
pub struct Function<'a> {
    pub name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    pub parameters: serde_json::Value,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub enum FunctionCallRequest<'a> {
    #[default]
    Auto,
    None,
    Call(&'a str),
}

impl Serialize for FunctionCallRequest<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Auto => serializer.serialize_str("auto"),
            Self::None => serializer.serialize_str("none"),
            Self::Call(name) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("name", name)?;
                map.end()
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Model {
    #[serde(rename = "gpt-3.5-turbo-0613")]
    Gpt35Turbo0613,
    #[serde(rename = "gpt-4-0613")]
    Gpt40613,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCall>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Function,
}
