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
    use std::env;

    use super::*;
    use crate::bindings::{ChatCompletion, ChatMessage, Embedding, Guest};

    fn get_api_key() -> Option<String> {
        env::var("OPENAI_SK").ok()
    }

    fn create_test_chat_completion() -> ChatCompletion {
        ChatCompletion {
            id: "chat_completion_id".to_string(),
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: "You are a helpful assistant.".to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: "Say hello in exactly 3 words.".to_string(),
                },
            ],
            temperature: Some(0.7),
            max_tokens: Some(50),
        }
    }

    fn create_test_embedding() -> Embedding {
        Embedding {
            model: "text-embedding-ada-002".to_string(),
            input: "Hello, world!".to_string(),
        }
    }

    #[test]
    fn test_create_chat_completion_success() {
        run_if_api_key_available(|| {
            let api_key = get_api_key().unwrap();
            let completion = create_test_chat_completion();

            let response = Component::create_chat_completion(api_key, completion);

            // Verify response structure
            assert!(!response.id.is_empty(), "Response ID should not be empty");
            assert!(
                !response.model.is_empty(),
                "Response model should not be empty"
            );
            assert!(
                !response.content.is_empty(),
                "Response content should not be empty"
            );
            assert!(
                !response.finish_reason.is_empty(),
                "Finish reason should not be empty"
            );

            // Verify the model matches what we requested
            assert!(
                response.model.contains("gpt-3.5-turbo"),
                "Model should contain gpt-3.5-turbo"
            );

            // Verify finish reason is valid
            assert!(
                response.finish_reason == "stop"
                    || response.finish_reason == "length"
                    || response.finish_reason == "content_filter"
                    || response.finish_reason == "tool_calls",
                "Finish reason should be a valid OpenAI finish reason"
            );
        });
    }

    #[test]
    fn test_create_chat_completion_with_different_models() {
        run_if_api_key_available(|| {
            let api_key = get_api_key().unwrap();
            let models = vec!["gpt-3.5-turbo", "gpt-4"];

            for model in models {
                let mut completion = create_test_chat_completion();
                completion.model = model.to_string();

                let response = Component::create_chat_completion(api_key.clone(), completion);

                assert!(
                    !response.content.is_empty(),
                    "Response should have content for model {}",
                    model
                );
                assert!(
                    response.model.contains(model),
                    "Response model should contain requested model"
                );
            }
        });
    }

    #[test]
    fn test_create_chat_completion_with_temperature_variations() {
        run_if_api_key_available(|| {
            let api_key = get_api_key().unwrap();
            let temperatures = vec![0.0, 0.5, 1.0];

            for temp in temperatures {
                let mut completion = create_test_chat_completion();
                completion.temperature = Some(temp);

                let response = Component::create_chat_completion(api_key.clone(), completion);

                assert!(
                    !response.content.is_empty(),
                    "Response should have content for temperature {}",
                    temp
                );
            }
        });
    }

    #[test]
    fn test_create_chat_completion_with_max_tokens() {
        run_if_api_key_available(|| {
            let api_key = get_api_key().unwrap();
            let mut completion = create_test_chat_completion();
            completion.max_tokens = Some(10);

            let response = Component::create_chat_completion(api_key, completion);

            assert!(
                !response.content.is_empty(),
                "Response should have content even with low max_tokens"
            );
            // Note: We can't easily verify the exact token count without tokenizing
        });
    }

    #[test]
    fn test_create_embedding_success() {
        run_if_api_key_available(|| {
            let api_key = get_api_key().unwrap();
            let embedding = create_test_embedding();

            let response = Component::create_embedding(api_key, embedding);

            // Verify response structure
            assert!(
                !response.model.is_empty(),
                "Response model should not be empty"
            );
            assert!(
                !response.embedding.is_empty(),
                "Embedding vector should not be empty"
            );

            // Verify the model matches what we requested
            assert!(
                response.model.contains("text-embedding-ada-002"),
                "Model should contain text-embedding-ada-002"
            );

            // Verify embedding dimensions (ada-002 should return 1536 dimensions)
            assert_eq!(
                response.embedding.len(),
                1536,
                "Ada-002 embedding should have 1536 dimensions"
            );

            // Verify embedding values are reasonable (should be between -1 and 1)
            for value in &response.embedding {
                assert!(
                    value.abs() <= 1.0,
                    "Embedding values should be between -1 and 1"
                );
            }
        });
    }

    #[test]
    fn test_create_embedding_with_different_inputs() {
        run_if_api_key_available(|| {
            let api_key = get_api_key().unwrap();
            let inputs = vec![
                "Short text",
                "This is a longer piece of text that should still work fine with the embedding API.",
                "12345",
                "Special characters: !@#$%^&*()",
            ];

            for input in inputs {
                let embedding = Embedding {
                    model: "text-embedding-ada-002".to_string(),
                    input: input.to_string(),
                };

                let response = Component::create_embedding(api_key.clone(), embedding);

                assert!(
                    !response.embedding.is_empty(),
                    "Should get embedding for input: {}",
                    input
                );
                assert_eq!(
                    response.embedding.len(),
                    1536,
                    "Should have correct dimensions for input: {}",
                    input
                );
            }
        });
    }

    #[test]
    fn test_create_embedding_reproducibility() {
        run_if_api_key_available(|| {
            let api_key = get_api_key().unwrap();
            let embedding = create_test_embedding();

            let response1 = Component::create_embedding(api_key.clone(), embedding.clone());
            let response2 = Component::create_embedding(api_key, embedding);

            // Embeddings should be identical for the same input
            assert_eq!(response1.embedding.len(), response2.embedding.len());
            for (val1, val2) in response1.embedding.iter().zip(response2.embedding.iter()) {
                assert!(
                    (val1 - val2).abs() < 1e-10,
                    "Embeddings should be identical for same input"
                );
            }
        });
    }

    #[test]
    fn test_create_chat_completion_invalid_api_key() {
        run_if_api_key_available(|| {
            let invalid_api_key = "invalid_key".to_string();
            let completion = create_test_chat_completion();

            let result = std::panic::catch_unwind(|| {
                Component::create_chat_completion(invalid_api_key, completion);
            });

            assert!(result.is_err(), "Should panic with invalid API key");
        });
    }

    #[test]
    fn test_create_embedding_invalid_api_key() {
        run_if_api_key_available(|| {
            let invalid_api_key = "invalid_key".to_string();
            let embedding = create_test_embedding();

            let result = std::panic::catch_unwind(|| {
                Component::create_embedding(invalid_api_key, embedding);
            });

            assert!(result.is_err(), "Should panic with invalid API key");
        });
    }

    #[test]
    fn test_create_chat_completion_conversation_flow() {
        run_if_api_key_available(|| {
            let api_key = get_api_key().unwrap();

            // First message
            let completion1 = ChatCompletion {
                id: "chat_completion_id".to_string(),
                model: "gpt-3.5-turbo".to_string(),
                messages: vec![ChatMessage {
                    role: "user".to_string(),
                    content: "What is 2+2?".to_string(),
                }],
                temperature: Some(0.1),
                max_tokens: Some(100),
            };

            let response1 = Component::create_chat_completion(api_key.clone(), completion1);
            assert!(!response1.content.is_empty());

            // Follow-up message using the response
            let completion2 = ChatCompletion {
                id: "chat_completion_id".to_string(),
                model: "gpt-3.5-turbo".to_string(),
                messages: vec![
                    ChatMessage {
                        role: "user".to_string(),
                        content: "What is 2+2?".to_string(),
                    },
                    ChatMessage {
                        role: "assistant".to_string(),
                        content: response1.content.clone(),
                    },
                    ChatMessage {
                        role: "user".to_string(),
                        content: "Now what is 3+3?".to_string(),
                    },
                ],
                temperature: Some(0.1),
                max_tokens: Some(100),
            };

            let response2 = Component::create_chat_completion(api_key, completion2);
            assert!(!response2.content.is_empty());
            assert_ne!(
                response1.content, response2.content,
                "Responses should be different"
            );
        });
    }

    // Helper function to run tests conditionally based on environment
    fn run_if_api_key_available<F>(test_fn: F)
    where
        F: FnOnce(),
    {
        if env::var("OPENAI_SK").is_ok() {
            test_fn();
        } else {
            println!("Skipping test - OPENAI_SK not set");
        }
    }

    #[test]
    fn test_conditional_execution() {
        run_if_api_key_available(|| {
            let api_key = get_api_key().unwrap();
            let completion = create_test_chat_completion();
            let response = Component::create_chat_completion(api_key, completion);
            assert!(!response.content.is_empty());
        });
    }
}
