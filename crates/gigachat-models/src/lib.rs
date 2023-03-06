mod settings;

use chrono::Utc;
use serde::{Deserialize, Serialize};

pub use settings::AppSettings;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GigaChatSender {
    pub login: String,
    pub display_name: String,
    pub color: String,
}

impl GigaChatSender {
    pub fn new(login: String, display_name: String, color: String) -> Self {
        Self {
            login,
            display_name,
            color,
        }
    }
}

impl Default for GigaChatSender {
    fn default() -> Self {
        Self::new("".to_string(), "".to_string(), "#000000".to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GigaChatMessage {
    pub sender: GigaChatSender,
    pub message: String,
    pub message_html: String,
    pub message_id: String,
    pub timestamp: chrono::DateTime<Utc>,
}

impl GigaChatMessage {
    pub fn new(
        sender: GigaChatSender,
        message: String,
        message_with_emotes: String,
        message_id: String,
        timestamp: chrono::DateTime<Utc>,
    ) -> Self {
        Self {
            sender,
            message,
            message_html: message_with_emotes,
            message_id,
            timestamp,
        }
    }
}

impl Default for GigaChatMessage {
    fn default() -> Self {
        Self::new(
            GigaChatSender::default(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            chrono::DateTime::<Utc>::default(),
        )
    }
}

impl PartialEq for GigaChatMessage {
    fn eq(&self, other: &Self) -> bool {
        self.message_id == other.message_id
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateIgnoreEventsArgs {
    pub ignore_events: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateAlwaysOnTopEventArgs {
    pub always_on_top: bool,
}
