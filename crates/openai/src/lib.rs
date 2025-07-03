#[allow(warnings)]
mod bindings;

use bindings::Guest;
use serde::{Deserialize, Serialize};
use waki::Client;

use crate::bindings::{ChatCompletion, ChatResponse, Embedding, EmbeddingResponse};

const OPENAI_API_BASE: &str = "https://api.openai.com/v1";

// TODO: remove unwrap

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bindings::{ChatCompletion, ChatMessage, Embedding};

    #[test]
    fn test_serializable_chat_message_creation() {
        let message = SerializableChatMessage {
            role: "user",
            content: "Hello, world!",
        };

        assert_eq!(message.role, "user");
        assert_eq!(message.content, "Hello, world!");
    }

    #[test]
    fn test_serializable_chat_completion_creation() {
        let messages = vec![SerializableChatMessage {
            role: "user",
            content: "Test message",
        }];

        let completion = SerializableChatCompletion {
            model: "gpt-3.5-turbo",
            messages,
            temperature: Some(0.7),
            max_tokens: Some(100),
        };

        assert_eq!(completion.model, "gpt-3.5-turbo");
        assert_eq!(completion.messages.len(), 1);
        assert_eq!(completion.temperature, Some(0.7));
        assert_eq!(completion.max_tokens, Some(100));
    }

    #[test]
    fn test_serializable_embedding_creation() {
        let embedding = SerializableEmbedding {
            model: "text-embedding-ada-002",
            input: "Test text",
        };

        assert_eq!(embedding.model, "text-embedding-ada-002");
        assert_eq!(embedding.input, "Test text");
    }

    #[test]
    fn test_openai_chat_response_deserialization() {
        let json = r#"
        {
            "id": "chatcmpl-123",
            "model": "gpt-3.5-turbo",
            "choices": [
                {
                    "message": {
                        "content": "Hello! How can I help you today?"
                    },
                    "finish_reason": "stop"
                }
            ]
        }
        "#;

        let response: OpenAIChatResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.id, "chatcmpl-123");
        assert_eq!(response.model, "gpt-3.5-turbo");
        assert_eq!(response.choices.len(), 1);
        assert_eq!(
            response.choices[0].message.content,
            "Hello! How can I help you today?"
        );
        assert_eq!(response.choices[0].finish_reason, "stop");
    }

    #[test]
    fn test_openai_embedding_response_deserialization() {
        let json = r#"
        {
            "model": "text-embedding-ada-002",
            "data": [
                {
                    "embedding": [0.1, 0.2, 0.3, 0.4, 0.5]
                }
            ]
        }
        "#;

        let response: OpenAIEmbeddingResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.model, "text-embedding-ada-002");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].embedding, vec![0.1, 0.2, 0.3, 0.4, 0.5]);
    }

    #[test]
    fn test_chat_message_to_serializable_conversion() {
        let chat_message = ChatMessage {
            role: "user".to_string(),
            content: "Hello, AI!".to_string(),
        };

        let serializable = SerializableChatMessage {
            role: &chat_message.role,
            content: &chat_message.content,
        };

        assert_eq!(serializable.role, "user");
        assert_eq!(serializable.content, "Hello, AI!");
    }

    #[test]
    fn test_chat_completion_message_mapping() {
        let completion = ChatCompletion {
            id: "test-id".to_string(),
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: "You are a helpful assistant.".to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: "Hello!".to_string(),
                },
            ],
            temperature: Some(0.7),
            max_tokens: Some(150),
        };

        let messages: Vec<SerializableChatMessage> = completion
            .messages
            .iter()
            .map(|m| SerializableChatMessage {
                role: &m.role,
                content: &m.content,
            })
            .collect();

        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].role, "system");
        assert_eq!(messages[0].content, "You are a helpful assistant.");
        assert_eq!(messages[1].role, "user");
        assert_eq!(messages[1].content, "Hello!");
    }

    #[test]
    fn test_embedding_to_serializable_conversion() {
        let embedding = Embedding {
            model: "text-embedding-ada-002".to_string(),
            input: "This is a test string".to_string(),
        };

        let serializable = SerializableEmbedding {
            model: &embedding.model,
            input: &embedding.input,
        };

        assert_eq!(serializable.model, "text-embedding-ada-002");
        assert_eq!(serializable.input, "This is a test string");
    }

    #[test]
    fn test_openai_api_base_constant() {
        assert_eq!(OPENAI_API_BASE, "https://api.openai.com/v1");
    }

    #[test]
    fn test_chat_completion_with_optional_fields() {
        let completion = ChatCompletion {
            id: "test-id-2".to_string(),
            model: "gpt-4".to_string(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: "Test".to_string(),
            }],
            temperature: None,
            max_tokens: None,
        };

        let serializable = SerializableChatCompletion {
            model: &completion.model,
            messages: completion
                .messages
                .iter()
                .map(|m| SerializableChatMessage {
                    role: &m.role,
                    content: &m.content,
                })
                .collect(),
            temperature: completion.temperature,
            max_tokens: completion.max_tokens,
        };

        assert_eq!(serializable.model, "gpt-4");
        assert_eq!(serializable.temperature, None);
        assert_eq!(serializable.max_tokens, None);
    }
}
