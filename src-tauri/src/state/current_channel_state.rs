use std::sync::Mutex;

use tauri::{AppHandle, Manager};

#[derive(Debug, Clone)]
pub struct CurrentChannel {
    pub channel_login: Option<String>,
    pub channel_id: Option<String>,
}

impl CurrentChannel {
    pub fn new(channel_login: Option<String>, channel_id: Option<String>) -> Self {
        Self {
            channel_login,
            channel_id,
        }
    }
}

impl Default for CurrentChannel {
    fn default() -> Self {
        Self::new(None, None)
    }
}

pub fn update_current_channel_state(
    app_handle: &AppHandle,
    new_channel_id: Option<String>,
    new_channel_login: Option<String>,
) {
    let current_channel_state = app_handle.state::<Mutex<CurrentChannel>>();
    let mut current_channel_state = current_channel_state.lock().unwrap();
    current_channel_state.channel_id = new_channel_id;
    current_channel_state.channel_login = new_channel_login;
}
