use gigachat_models::AppSettings;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};

mod app_ready;
mod save_settings;

pub use app_ready::app_ready;
pub use save_settings::save_settings;

use crate::{state::app_settings_state::toggle_ui_lock_state, utils::open_settings_window};

// Returns old settings
fn update_settings_state(
    settings_state: tauri::State<'_, Mutex<AppSettings>>,
    new_settings: AppSettings,
) -> AppSettings {
    let mut settings = settings_state.lock().unwrap();
    let old_settings = (*settings).clone();
    *settings = new_settings;

    old_settings
}

#[tauri::command]
pub fn unlock_ui(app_handle: AppHandle) {
    toggle_ui_lock_state(&app_handle, false);
}

#[tauri::command]
pub fn lock_ui(app_handle: AppHandle) {
    toggle_ui_lock_state(&app_handle, true);
}

#[tauri::command]
pub async fn open_settings(app_handle: AppHandle) {
    open_settings_window(&app_handle);
}

#[tauri::command]
pub fn sync_settings(app_handle: AppHandle, new_settings: AppSettings) {
    app_handle
        .emit_all("sync_settings", new_settings)
        .expect("Failed to emit sync_settings event");
}
