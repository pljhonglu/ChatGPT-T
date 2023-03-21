use std::{fs::{self}, path::PathBuf};

use log::info;

#[tauri::command]
pub async fn download_img(name: String, blob: Vec<u8>) -> Result<String, String> {
  info!("save image {}", name);
  // let win = app.app_handle().get_window("core");
  let download_path = tauri::api::path::download_dir().unwrap().join(PathBuf::from(name));
  fs::write(&download_path, blob).unwrap();
  
  Ok(download_path.display().to_string())
}
