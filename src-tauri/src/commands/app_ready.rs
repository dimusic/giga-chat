use std::sync::Mutex;

use gigachat_models::{AppSettings, UpdateIgnoreEventsArgs};
use tauri::AppHandle;

use crate::{
    state::ui_lock_state::UiLockState,
    utils::{emit_init_settings, emit_toggle_ui_lock},
};

#[tauri::command]
pub fn app_ready(
    app_handle: AppHandle,
    settings_state: tauri::State<'_, Mutex<AppSettings>>,
    ui_lock_state: tauri::State<'_, UiLockState>,
) {
    info!("'app_ready' command invoked");

    let settings = settings_state.lock().unwrap();
    emit_init_settings(&app_handle, &settings);

    let is_ui_locked = ui_lock_state.lock().unwrap();
    emit_toggle_ui_lock(
        &app_handle,
        UpdateIgnoreEventsArgs {
            ignore_events: *is_ui_locked,
        },
    );
}
