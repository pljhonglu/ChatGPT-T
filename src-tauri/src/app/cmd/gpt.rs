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
    #[error("Custom Error: (code: {code:?}, message: {msg:?})")]
    Custom{code: u16, msg: String}
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
}

#[tauri::command]
pub async fn fetch_chat_api(
    handle: AppHandle,
    id: u64,
    proxy: Option<String>,
    token: String,
    model: String,
    messages: Vec<HashMap<String, String>>,
    temperature: f32
) -> Result<u64> {
    // https://platform.openai.com/docs/guides/chat/introduction
    // "https://api.openai.com/v1/chat/completions";
    let url = "https://api.openai.com/v1/chat/completions";
    let data = json!({
        "model": model,
        "messages": messages,
        "temperature": temperature,
        "stream": true
    });
    let proxy_str = proxy.unwrap_or(String::from(""));

    let client : reqwest::Client = {
        log::info!("proxy is: {}", proxy_str);
        let mut client_builder = reqwest::Client::builder();
        if proxy_str.len()>0 {
            let proxy = reqwest::Proxy::all(proxy_str).unwrap();
            client_builder = client_builder.proxy(proxy);
        }
        client_builder.build().unwrap()
    };
    let res = client.post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .timeout(Duration::from_secs(300))
        .body(data.to_string())
        .send()
        .await?;
    info!("send message: {}", json!(messages));
    
    if res.status().as_u16() != 200 {
        return Err(Error::Custom {code: res.status().as_u16(), msg:String::from("openai api request error!")})
    }

    let mut stream = res.bytes_stream().eventsource();
    while let Some(chunk) = stream.try_next().await? {
        let chunk = chunk.data;
        if chunk == "[DONE]" {
            return Ok(id)
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