use std::{collections::HashMap, sync::Mutex};

use tauri::{AppHandle, Manager};

pub type EmotesMap = HashMap<String, String>;

#[derive(Debug, Clone)]
pub struct SavedEmotes {
    pub global_emotes: EmotesMap,
    pub channel_emotes: EmotesMap,
}

impl SavedEmotes {
    pub fn new(global_emotes: EmotesMap, channel_emotes: EmotesMap) -> Self {
        Self {
            global_emotes,
            channel_emotes,
        }
    }
}

impl Default for SavedEmotes {
    fn default() -> Self {
        Self::new(HashMap::new(), HashMap::new())
    }
}

pub fn reset_channel_emotes_state(app_handle: &AppHandle) {
    let saved_emotes_state = app_handle.state::<Mutex<SavedEmotes>>();
    saved_emotes_state.lock().unwrap().channel_emotes = HashMap::<String, String>::new();
}

pub fn update_channel_emotes_state(app_handle: &AppHandle, channel_emotes: EmotesMap) {
    let saved_emotes_state = app_handle.state::<Mutex<SavedEmotes>>();
    saved_emotes_state.lock().unwrap().channel_emotes = channel_emotes;
}

pub fn update_global_emotes_state(app_handle: &AppHandle, global_emotes: EmotesMap) {
    let saved_emotes_state = app_handle.state::<Mutex<SavedEmotes>>();
    saved_emotes_state.lock().unwrap().global_emotes = global_emotes;
}
