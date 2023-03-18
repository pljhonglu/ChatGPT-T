use std::{fs::{self}, path::PathBuf};

#[tauri::command]
pub async fn download(name: String, blob: Vec<u8>) {
  println!("save image {}", name);
  // let win = app.app_handle().get_window("core");
  let download_path = tauri::api::path::download_dir().unwrap().join(PathBuf::from(name));
  fs::write(&download_path, blob).unwrap();
  
  
  Ok::<std::string::String, String>(download_path.display().to_string());
}
