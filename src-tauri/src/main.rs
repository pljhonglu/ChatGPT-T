#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod app;
mod utils;

use app::{builder, cmd};
use log::{error, info};
use tauri::Manager;
use tauri_plugin_log::{
  fern::colors::{Color, ColoredLevelConfig},
  LogTarget,
};

fn main() {


  let mut log = tauri_plugin_log::Builder::default()
  .targets([
    LogTarget::Folder(utils::app_root()),
    LogTarget::Stdout,
    LogTarget::Webview,
  ])
  .level(log::LevelFilter::Debug);

  if cfg!(debug_assertions) {
    log = log.with_colors(ColoredLevelConfig {
      error: Color::Red,
      warn: Color::Yellow,
      debug: Color::Blue,
      info: Color::BrightGreen,
      trace: Color::Cyan,
    });
  }


  let mut builder = tauri::Builder::default()
  .plugin(log.build())
  .invoke_handler(tauri::generate_handler![
    cmd::gpt::fetch_chat_api,
    cmd::download::download_img
  ])
  .setup(builder::setup)
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
