use std::io::{self, BufRead, Write};

use serde::{Deserialize, Serialize};

// --- Protocol types ---

#[derive(Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
enum Request {
    GetInfo,
    GetTargets,
    Send {
        target_id: String,
        content: String,
        format: String,
    },
}

#[derive(Serialize)]
struct InfoResponse {
    name: &'static str,
    version: &'static str,
    description: &'static str,
    author: &'static str,
}

#[derive(Serialize)]
struct Target {
    id: &'static str,
    provider: &'static str,
    formats: Vec<&'static str>,
    title: &'static str,
    description: &'static str,
    image: &'static str,
}

#[derive(Serialize)]
struct TargetsResponse {
    targets: Vec<Target>,
}

#[derive(Serialize)]
struct SendResponse {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

// --- Handlers ---

fn handle(request: Request) -> serde_json::Value {
    match request {
        Request::GetInfo => serde_json::to_value(InfoResponse {
            name: "Demo Plugin",
            version: "1.0.0",
            description: "Demo subprocess target provider for clipygo",
            author: "clipygo",
        })
        .unwrap(),

        Request::GetTargets => serde_json::to_value(TargetsResponse {
            targets: vec![
                Target {
                    id: "demo-target-1",
                    provider: "Demo Plugin",
                    formats: vec!["text"],
                    title: "Demo Target 1",
                    description: "First demo target",
                    image: "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==",
                },
                Target {
                    id: "demo-target-2",
                    provider: "Demo Plugin",
                    formats: vec!["text"],
                    title: "Demo Target 2",
                    description: "Second demo target",
                    image: "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8v59hPwAEaAI+rVUOawAAAABJRU5ErkJggg==",
                },
            ],
        })
        .unwrap(),

        Request::Send { target_id, content, format } => {
            eprintln!("[demo] send → target={} format={} content={:?}", target_id, format, &content[..content.len().min(80)]);
            serde_json::to_value(SendResponse { success: true, error: None }).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_info_returns_name_and_version() {
        let resp = handle(Request::GetInfo);
        assert_eq!(resp["name"], "Demo Plugin");
        assert_eq!(resp["version"], "1.0.0");
        assert!(resp["description"].is_string());
        assert!(resp["author"].is_string());
    }

    #[test]
    fn get_targets_returns_two_targets() {
        let resp = handle(Request::GetTargets);
        let targets = resp["targets"].as_array().expect("targets must be array");
        assert_eq!(targets.len(), 2);
        assert_eq!(targets[0]["id"], "demo-target-1");
        assert_eq!(targets[1]["id"], "demo-target-2");
        for t in targets {
            assert!(t["formats"]
                .as_array()
                .unwrap()
                .contains(&serde_json::json!("text")));
        }
    }

    #[test]
    fn send_returns_success() {
        let resp = handle(Request::Send {
            target_id: "demo-target-1".to_string(),
            content: "hello".to_string(),
            format: "text".to_string(),
        });
        assert_eq!(resp["success"], true);
        assert!(resp.get("error").is_none());
    }

    #[test]
    fn invalid_json_produces_error_field() {
        let result = serde_json::from_str::<Request>("not json");
        assert!(result.is_err());
    }

    #[test]
    fn unknown_command_produces_error_field() {
        let result = serde_json::from_str::<Request>(r#"{"command":"unknown"}"#);
        assert!(result.is_err());
    }
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };

        let response = match serde_json::from_str::<Request>(&line) {
            Ok(request) => handle(request),
            Err(e) => serde_json::json!({ "error": format!("Bad request: {}", e) }),
        };

        let _ = writeln!(out, "{}", response);
        let _ = out.flush();
    }
}
