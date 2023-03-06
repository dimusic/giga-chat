use std::fmt::Display;

use bounce::Atom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Atom, PartialEq)]
pub struct AppSettings {
    pub channel_name: String,
    pub always_on_top: bool,
    pub show_timestamp: bool,
    pub background_color: String,
    pub background_opacity: f32,
    pub font_size: i32,
    pub enable_animation: bool,
    pub messages_sort_asc: bool,
}

impl AppSettings {
    pub fn new(channel_name: String, always_on_top: bool) -> Self {
        Self {
            channel_name,
            always_on_top,
            show_timestamp: true,
            background_color: String::from("#000000"),
            background_opacity: 0.4,
            font_size: 14,
            enable_animation: true,
            messages_sort_asc: true,
        }
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        Self::new(String::from(""), true)
    }
}

impl Display for AppSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AppSettings {{
                channel_name: {},
                always_on_top: {},
                show_timestamp: {},
                background_color: {},
                background_opacity: {},
                font_size: {},
                enable_animation: {},
                messages_sort_asc: {}
            }}",
            self.channel_name,
            self.always_on_top,
            self.show_timestamp,
            self.background_color,
            self.background_opacity,
            self.font_size,
            self.enable_animation,
            self.messages_sort_asc
        )
    }
}
