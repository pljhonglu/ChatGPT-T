use tauri::{App, Manager};



pub fn setup(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    // debug tool
    #[cfg(debug_assertions)] // only include this code on debug builds
    {
    let window = app.get_window("main").unwrap();
    window.open_devtools();
    window.close_devtools();
    }
    
    // hidden docker icon
    // #[cfg(target_os = "macos")]
    // app.set_activation_policy(tauri::ActivationPolicy::Accessory);


    Ok(())
}