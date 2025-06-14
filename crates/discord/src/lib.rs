use bindings::Guest;
use serde::{Deserialize, Serialize};
use waki::Client;

use crate::bindings::{Channel, Message, User, Webhook};

#[allow(warnings)]
mod bindings;

const DISCORD_API_BASE: &str = "https://discord.com/api/v10";

#[derive(Deserialize, Serialize)]
struct DiscordResponse<T> {
    id: String,
    #[serde(flatten)]
    data: T,
}

#[derive(Deserialize)]
struct DiscordWebhook {
    id: String,
    token: String,
    url: String,
}

#[derive(Deserialize)]
struct DiscordChannel {
    id: String,
    name: String,
    #[serde(rename = "type")]
    ty: u32,
    #[serde(rename = "guild_id")]
    guild_id: Option<String>,
}

#[derive(Deserialize)]
struct DiscordUser {
    id: String,
    username: String,
    discriminator: String,
    avatar: Option<String>,
}

struct Component;

impl Guest for Component {
    fn create_webhook(token: String, channel_id: String, name: String) -> Webhook {
        let response = Client::new()
            .post(&format!(
                "{DISCORD_API_BASE}/channels/{channel_id}/webhooks"
            ))
            .header("Authorization", format!("Bot {token}"))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "name": name
            }))
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

        let webhook: DiscordWebhook = serde_json::from_str(&body_str).unwrap();
        Webhook {
            id: webhook.id,
            token: webhook.token,
            url: webhook.url,
        }
    }

    fn delete_webhook(token: String, webhook_id: String, webhook_token: String) -> bool {
        let response = Client::new()
            .delete(&format!(
                "{DISCORD_API_BASE}/webhooks/{webhook_id}/{webhook_token}"
            ))
            .header("Authorization", format!("Bot {token}"))
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

        body_str.contains("200")
    }

    fn delete_message(token: String, channel_id: String, message_id: String) -> bool {
        let response = Client::new()
            .delete(&format!(
                "{DISCORD_API_BASE}/channels/{channel_id}/messages/{message_id}"
            ))
            .header("Authorization", format!("Bot {token}"))
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

        body_str.contains("200")
    }

    fn edit_message(
        token: String,
        channel_id: String,
        message_id: String,
        content: String,
    ) -> bool {
        let response = Client::new()
            .patch(&format!(
                "{DISCORD_API_BASE}/channels/{channel_id}/messages/{message_id}"
            ))
            .header("Authorization", format!("Bot {token}"))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "content": content
            }))
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

        body_str.contains("200")
    }

    fn get_channel(token: String, channel_id: String) -> Channel {
        let response = Client::new()
            .get(&format!("{DISCORD_API_BASE}/channels/{channel_id}"))
            .header("Authorization", format!("Bot {token}"))
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

        let channel: DiscordChannel = serde_json::from_str(&body_str).unwrap();
        Channel {
            id: channel.id,
            name: channel.name,
            ty: channel.ty,
            guild_id: channel.guild_id,
        }
    }

    fn get_user(token: String, user_id: String) -> User {
        let response = Client::new()
            .get(&format!("{DISCORD_API_BASE}/users/{user_id}"))
            .header("Authorization", format!("Bot {token}"))
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

        let user: DiscordUser = serde_json::from_str(&body_str).unwrap();
        User {
            id: user.id,
            username: user.username,
            discriminator: user.discriminator,
            avatar: user.avatar,
        }
    }

    fn send_message(token: String, message: Message) -> String {
        let response = Client::new()
            .post(&format!(
                "{}/channels/{}/messages",
                DISCORD_API_BASE, message.channel_id
            ))
            .header("Authorization", format!("Bot {token}"))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "content": message.content,
                "guild_id": message.guild_id
            }))
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

        let message_data: DiscordResponse<()> = serde_json::from_str(&body_str).unwrap();
        message_data.id
    }

    fn send_webhook_message(token: String, webhook: Webhook, content: String) -> String {
        let response = Client::new()
            .post(&format!(
                "{}/webhooks/{}/{}",
                DISCORD_API_BASE, webhook.id, webhook.token
            ))
            .header("Authorization", format!("Bot {token}"))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "content": content
            }))
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

        let message_data: DiscordResponse<()> = serde_json::from_str(&body_str).unwrap();
        message_data.id
    }
}

bindings::export!(Component with_types_in bindings);
