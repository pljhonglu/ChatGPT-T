use tauri::{Manager};

#[tauri::command]
pub fn new_window(app: tauri::AppHandle, label: String, title: String, url: String){
    let win = app.get_window(&label);
    
    if win.is_none() {
      tauri::async_runtime::spawn(async move {
        tauri::WindowBuilder::new(&app, label, tauri::WindowUrl::App(url.parse().unwrap()))
          .title(title)
          .inner_size(700.0, 550.0)
          .resizable(true)
          .build()
          .unwrap();
      });
    } else if let Some(v) = win {
      if !v.is_visible().unwrap() {
        v.show().unwrap();
      }
      v.eval("window.location.reload()").unwrap();
      v.set_focus().unwrap();
    }
}

#[tauri::command]
pub fn window_reload(app: tauri::AppHandle, label: &str) {
  app
    .app_handle()
    .get_window(label)
    .unwrap()
    .eval("window.location.reload()")
    .unwrap();
}