#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod app;
use app::{cmd, gpt};

fn main() {
  use tauri::Manager;
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![
    cmd::download,
    gpt::fetch_chat_api
    
    ])
    .setup(|app| {
      #[cfg(debug_assertions)] // only include this code on debug builds
      {
        let window = app.get_window("main").unwrap();
        window.open_devtools();
        window.close_devtools();
      }
      
      #[cfg(target_os = "macos")]
      app.set_activation_policy(tauri::ActivationPolicy::Accessory);
      Ok(())
    })
    .build(tauri::generate_context!())
    .expect("error while running tauri application")
    .run(|app, event| match event {
        tauri::RunEvent::WindowEvent {
            label,
            event: win_event,
            ..
        } => match win_event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                let win = app.get_window(label.as_str()).unwrap();
                win.minimize().unwrap();
                api.prevent_close();
            }
            _ => {}
        },
        _ => {}
    });
}
