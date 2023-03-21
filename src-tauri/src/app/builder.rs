use tauri::{App, WindowBuilder, WindowUrl};
use log;



pub fn setup(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    // debug tool
    // #[cfg(debug_assertions)] // only include this code on debug builds
    // {
    // let window = app.get_window("main").unwrap();
    // window.open_devtools();
    // window.close_devtools();
    // }
    
    let app = app.handle();
    tauri::async_runtime::spawn(async move {
      log::info!("main_window:");
      let mut main_win = WindowBuilder::new(&app, "core", WindowUrl::App("index.html".into()))
        .title("ChatGPT-Tauri")
        .resizable(true)
        .fullscreen(false)
        .inner_size(800.0, 600.0);

      #[cfg(target_os = "macos")]
      {
        main_win = main_win
          .title_bar_style(tauri::TitleBarStyle::Overlay)
          .hidden_title(true);
      }
      main_win.build().unwrap();
    });
    
    // hidden docker icon
    // #[cfg(target_os = "macos")]
    // app.set_activation_policy(tauri::ActivationPolicy::Accessory);


    Ok(())
}