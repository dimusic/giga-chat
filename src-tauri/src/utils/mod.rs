use gigachat_models::{
    AppSettings, GigaChatMessage, UpdateAlwaysOnTopEventArgs, UpdateIgnoreEventsArgs,
};
use tauri::{AppHandle, LogicalSize, Manager};

pub fn emit_new_message<R: tauri::Runtime>(message: GigaChatMessage, manager: &impl Manager<R>) {
    manager.emit_all("new_message", message).unwrap();
}

pub fn emit_toggle_ui_lock<R: tauri::Runtime>(
    manager: &impl Manager<R>,
    value: UpdateIgnoreEventsArgs,
) {
    manager.emit_all("toggle_ui_lock", value).unwrap();
}

pub fn emit_toggle_always_on_top<R: tauri::Runtime>(
    manager: &impl Manager<R>,
    value: UpdateAlwaysOnTopEventArgs,
) {
    manager.emit_all("toggle_always_on_top", value).unwrap();
}

pub fn emit_init_settings<R: tauri::Runtime>(manager: &impl Manager<R>, settings: &AppSettings) {
    info!("Emitting init_settings");

    manager.emit_all("init_settings", settings).unwrap();
}

pub fn create_settings_window(app_handle: &AppHandle) -> tauri::Window {
    let settings_window = tauri::WindowBuilder::new(
        app_handle,
        "settings",
        tauri::WindowUrl::App("/settings".into()),
    )
    .resizable(false)
    .decorations(true)
    .center()
    // .inner_size(600_f64, 400_f64)
    .title("Settings")
    .build()
    .expect("Failed to create settings window");

    // settings_window.open_devtools();

    settings_window
}

pub fn open_settings_window(app_handle: &AppHandle) -> tauri::Window {
    let window = app_handle
        .get_window("settings")
        .unwrap_or_else(|| create_settings_window(app_handle));

    let _ = window.set_size(LogicalSize {
        width: 600_f64,
        height: 420_f64,
    });

    window
}

pub fn close_settings_window(app_handle: &AppHandle) {
    let settings_window = app_handle.get_window("settings");

    if let Some(settings_window) = settings_window {
        settings_window
            .close()
            .expect("Couldn't close settings window");
    }
}
