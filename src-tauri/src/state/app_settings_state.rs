use std::sync::Mutex;

use gigachat_models::AppSettings;
use tauri::{AppHandle, Manager};
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

use super::ui_lock_state::UiLockState;

pub fn toggle_always_on_top_state(app_handle: &AppHandle, state: bool) {
    let app_window = app_handle.get_window("main").unwrap();
    let app_state = app_handle.state::<Mutex<AppSettings>>();

    app_state.lock().unwrap().always_on_top = state;

    let _ = app_window.set_always_on_top(state);

    let _ = app_handle
        .tray_handle()
        .get_item("always_on_top")
        .set_selected(state);
}

pub fn toggle_ui_lock_state(app_handle: &AppHandle, is_ui_locked: bool) {
    let app_window = app_handle.get_window("main").unwrap();
    let ui_lock_state = app_handle.state::<UiLockState>();
    *ui_lock_state.lock().unwrap() = is_ui_locked;

    if is_ui_locked {
        match app_handle.save_window_state(StateFlags::all()) {
            Ok(res) => debug!("save window state success: {:?}", res),
            Err(err) => error!("save window state failed: {:?}", err),
        }
    }

    let _ = app_window.set_ignore_cursor_events(is_ui_locked);

    let _ = app_handle
        .tray_handle()
        .get_item("lock")
        .set_selected(is_ui_locked);
}
