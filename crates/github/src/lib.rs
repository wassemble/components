use bindings::Guest;
use serde::{Deserialize, Serialize};
use waki::Client;

use crate::bindings::{Issue, Repository, User};

#[allow(warnings)]
mod bindings;

const GITHUB_API_BASE: &str = "https://api.github.com";

#[derive(Deserialize, Serialize)]
struct GitHubResponse<T> {
    #[serde(flatten)]
    data: T,
}

#[derive(Deserialize, Serialize)]
struct GitHubIssue {
    body: String,
    number: u32,
    title: String,
}

#[derive(Deserialize, Serialize)]
struct GitHubRepository {
    description: String,
    name: String,
    owner: GitHubUser,
}

#[derive(Deserialize, Serialize)]
struct GitHubUser {
    #[serde(rename = "avatar_url")]
    avatar_url: String,
    id: u64,
    login: String,
}

struct Component;

impl Guest for Component {
    fn create_issue(
        token: String,
        owner: String,
        repo: String,
        title: String,
        body: String,
    ) -> Issue {
        let response = Client::new()
            .post(&format!("{GITHUB_API_BASE}/repos/{owner}/{repo}/issues"))
            .header("Authorization", format!("Bearer {token}"))
            .header("Accept", "application/vnd.github.v3+json")
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "title": title,
                "body": body
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

        let issue: GitHubIssue = serde_json::from_str(&body_str).unwrap();
        Issue {
            body: issue.body,
            number: issue.number,
            title: issue.title,
        }
    }

    fn create_repository(token: String, name: String, description: String) -> Repository {
        let response = Client::new()
            .post(&format!("{GITHUB_API_BASE}/user/repos"))
            .header("Authorization", format!("Bearer {token}"))
            .header("Accept", "application/vnd.github.v3+json")
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "name": name,
                "description": description,
                "private": false
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

        let repo: GitHubRepository = serde_json::from_str(&body_str).unwrap();
        Repository {
            description: repo.description,
            name: repo.name,
            owner: repo.owner.login,
        }
    }

    fn delete_repository(token: String, owner: String, repo: String) -> bool {
        let response = Client::new()
            .delete(&format!("{GITHUB_API_BASE}/repos/{owner}/{repo}"))
            .header("Authorization", format!("Bearer {token}"))
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .map_err(|e| format!("Failed to send request: {e}"))
            .unwrap();

        response.status_code() == 204
    }

    fn get_user(token: String) -> User {
        let response = Client::new()
            .get(&format!("{GITHUB_API_BASE}/user"))
            .header("Authorization", format!("Bearer {token}"))
            .header("Accept", "application/vnd.github.v3+json")
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

        let user: GitHubUser = serde_json::from_str(&body_str).unwrap();
        User {
            avatar_url: user.avatar_url,
            id: user.id,
            login: user.login,
        }
    }

    fn update_issue(
        token: String,
        owner: String,
        repo: String,
        number: u32,
        title: String,
        body: String,
    ) -> Issue {
        let response = Client::new()
            .patch(&format!(
                "{GITHUB_API_BASE}/repos/{owner}/{repo}/issues/{number}"
            ))
            .header("Authorization", format!("Bearer {token}"))
            .header("Accept", "application/vnd.github.v3+json")
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "title": title,
                "body": body
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

        let issue: GitHubIssue = serde_json::from_str(&body_str).unwrap();
        Issue {
            body: issue.body,
            number: issue.number,
            title: issue.title,
        }
    }
}

bindings::export!(Component with_types_in bindings);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_github_issue_struct() {
        let issue = GitHubIssue {
            body: "body".to_string(),
            number: 1,
            title: "title".to_string(),
        };
        assert_eq!(issue.body, "body");
        assert_eq!(issue.number, 1);
        assert_eq!(issue.title, "title");
    }
}
