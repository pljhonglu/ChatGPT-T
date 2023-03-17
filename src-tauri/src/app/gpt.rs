use tauri::{AppHandle, Manager, Runtime, Window};
use reqwest;
use eventsource_stream::{Eventsource, EventStream, EventStreamError};
use serde_json::{json, Value};
use serde::{ser::Serializer, Serialize, Deserialize};
use futures::{TryStreamExt};
use tokio::sync::mpsc::error;
use std::{collections::HashMap};

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
    messages: Vec<HashMap<String, String>>,
    temperature: f32
) -> Result<u64> {
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
        .header("Authorization", "Bearer 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")
        .body(data.to_string())
        .send()
        .await?;

    let mut stream = res.bytes_stream().eventsource();
    while let Some(chunk) = stream.try_next().await? {
        let chunk = chunk.data;
        if chunk == "[DONE]" {
            let role = String::from("");
            let content = String::from("");
            let finish_reason = String::from("");
            let progress = ProgressPayload {id, detail:content, finish_reason};
            progress.emit_finished(&handle);
            break;
        } else {
            let object:Value = serde_json::from_str(&chunk)?;
            let delta = &object["choices"][0]["delta"];
            let content = String::from(delta["content"].as_str().unwrap_or(""));
            let finish_reason = String::from(object["finish_reason"].as_str().unwrap_or(""));
            let progress = ProgressPayload {id, detail:content, finish_reason};
            progress.emit_progress(&handle);
        }
    }
    Ok(id)
}