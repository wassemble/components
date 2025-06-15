#[allow(warnings)]
mod bindings;

use bindings::Guest;
use serde::{Deserialize, Serialize};
use waki::Client;

use crate::bindings::{ChatCompletion, ChatResponse, Embedding, EmbeddingResponse};

const OPENAI_API_BASE: &str = "https://api.openai.com/v1";

#[derive(Deserialize, Serialize)]
struct OpenAIChatResponse {
    id: String,
    model: String,
    choices: Vec<OpenAIChoice>,
}

#[derive(Deserialize, Serialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
    finish_reason: String,
}

#[derive(Deserialize, Serialize)]
struct OpenAIMessage {
    content: String,
}

#[derive(Deserialize, Serialize)]
struct OpenAIEmbeddingResponse {
    model: String,
    data: Vec<OpenAIEmbeddingData>,
}

#[derive(Deserialize, Serialize)]
struct OpenAIEmbeddingData {
    embedding: Vec<f64>,
}

#[derive(Serialize)]
struct SerializableChatMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Serialize)]
struct SerializableChatCompletion<'a> {
    model: &'a str,
    messages: Vec<SerializableChatMessage<'a>>,
    temperature: Option<f64>,
    max_tokens: Option<u32>,
}

#[derive(Serialize)]
struct SerializableEmbedding<'a> {
    model: &'a str,
    input: &'a str,
}

struct Component;

impl Guest for Component {
    fn create_chat_completion(api_key: String, completion: ChatCompletion) -> ChatResponse {
        let messages: Vec<SerializableChatMessage> = completion
            .messages
            .iter()
            .map(|m| SerializableChatMessage {
                role: &m.role,
                content: &m.content,
            })
            .collect();
        let serializable = SerializableChatCompletion {
            model: &completion.model,
            messages,
            temperature: completion.temperature,
            max_tokens: completion.max_tokens,
        };
        let response = Client::new()
            .post(&format!("{OPENAI_API_BASE}/chat/completions"))
            .header("Authorization", format!("Bearer {api_key}"))
            .header("Content-Type", "application/json")
            .json(&serializable)
            .send()
            .map_err(|e| format!("Failed to send request: {e}"))
            .unwrap();

        let body = response
            .body()
            .map_err(|e| format!("Failed to get response body: {e}"))
            .unwrap();

        let body_str = String::from_utf8(body)
            .map_err(|e| format!("Failed to parse response as UTF-8: {e}"))
            .unwrap();

        let openai_response: OpenAIChatResponse = serde_json::from_str(&body_str).unwrap();
        let choice = &openai_response.choices[0];

        ChatResponse {
            id: openai_response.id,
            model: openai_response.model,
            content: choice.message.content.clone(),
            finish_reason: choice.finish_reason.clone(),
        }
    }

    fn create_embedding(api_key: String, embedding: Embedding) -> EmbeddingResponse {
        let serializable = SerializableEmbedding {
            model: &embedding.model,
            input: &embedding.input,
        };
        let response = Client::new()
            .post(&format!("{OPENAI_API_BASE}/embeddings"))
            .header("Authorization", format!("Bearer {api_key}"))
            .header("Content-Type", "application/json")
            .json(&serializable)
            .send()
            .map_err(|e| format!("Failed to send request: {e}"))
            .unwrap();

        let body = response
            .body()
            .map_err(|e| format!("Failed to get response body: {e}"))
            .unwrap();

        let body_str = String::from_utf8(body)
            .map_err(|e| format!("Failed to parse response as UTF-8: {e}"))
            .unwrap();

        let openai_response: OpenAIEmbeddingResponse = serde_json::from_str(&body_str).unwrap();
        let data = &openai_response.data[0];

        EmbeddingResponse {
            model: openai_response.model,
            embedding: data.embedding.clone(),
        }
    }
}

bindings::export!(Component with_types_in bindings);
