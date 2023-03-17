use std::{fs::{self}, path::PathBuf,};
use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn download(app: AppHandle, name: String, blob: Vec<u8>) {
  println!("save image {}", name);
  let win = app.app_handle().get_window("core");
  let download_path = tauri::api::path::download_dir().unwrap().join(PathBuf::from(name));
  fs::write(&download_path, blob).unwrap();
  tauri::api::dialog::message(
    win.as_ref(),
    "Save File",
    format!("PATH: {}", download_path.display()),
  );
}
