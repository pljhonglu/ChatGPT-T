use tauri::{AppHandle, Manager};
use reqwest;
use eventsource_stream::{Eventsource, EventStreamError};
use serde_json::{json, Value};
use serde::{ser::Serializer, Serialize, Deserialize};
use futures::{TryStreamExt};
use std::{collections::HashMap, time::Duration};
use log::{error, info};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Stream(#[from] EventStreamError<reqwest::Error>),
    #[error("Custom Error")]
    Custom(String)
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProgressPayload {
    pub id: u64,
    pub detail: String,
    pub role: String,
    pub finish_reason: String,
}

impl ProgressPayload {
    pub fn emit_progress(&self, handle: &AppHandle) {
        handle.emit_all("CHAT_FETCHEING_PROGRESS", &self).ok();
    }

    pub fn emit_finished(&self, handle: &AppHandle) {
        handle.emit_all("CHAT_FETCHEING_FINISHED", &self).ok();
    }
}

#[tauri::command]
pub async fn fetch_chat_api(
    handle: AppHandle,
    id: u64,
    token: String,
    messages: Vec<HashMap<String, String>>,
    temperature: f32
) -> Result<u64> {
    // https://platform.openai.com/docs/guides/chat/introduction
    let url = "https://api.openai.com/v1/chat/completions";
    let model = "gpt-3.5-turbo";
    let data = json!({
        "model": model,
        "messages": messages,
        "temperature": temperature,
        "stream": true
    });
    let client = reqwest::Client::new();
    let res = client.post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .timeout(Duration::from_secs(300))
        .body(data.to_string())
        .send()
        .await?;
    info!("send message: {}", json!(messages));
    
    if res.status().as_u16() != 200 {
        let content = format!("Request Error: {}", res.status().as_str());
        let role = String::from("");
        let finish_reason = String::from("error");
        let progress = ProgressPayload {id, detail:content, role, finish_reason};
        progress.emit_finished(&handle);
        Error::Custom(format!("Request Error: {}", res.status().as_str()));
    }

    let mut stream = res.bytes_stream().eventsource();
    while let Some(chunk) = stream.try_next().await? {
        let chunk = chunk.data;
        if chunk == "[DONE]" {
            let content = String::from("");
            let role = String::from("");
            let finish_reason = String::from("");
            let progress = ProgressPayload {id, detail:content, role, finish_reason};
            progress.emit_finished(&handle);
            break;
        } else {
            let object:Value = serde_json::from_str(&chunk)?;
            let delta = &object["choices"][0]["delta"];
            let content = String::from(delta["content"].as_str().unwrap_or(""));
            let role = String::from(delta["role"].as_str().unwrap_or(""));
            let finish_reason = String::from(object["finish_reason"].as_str().unwrap_or(""));
            let progress = ProgressPayload {id, detail:content, role, finish_reason};
            progress.emit_progress(&handle);
        }
    }
    Ok(id)
}