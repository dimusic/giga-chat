use std::{fs, path::PathBuf};

use gigachat_models::AppSettings;
use tauri::AppHandle;

pub struct SettingsService {}

impl SettingsService {
    pub fn get_settings_path(app_handle: &AppHandle) -> PathBuf {
        let app_settings_path = app_handle
            .path_resolver()
            .app_config_dir()
            .expect("Failed to get app config path");

        app_settings_path.join("app_settings.json")
    }

    pub fn get_settings(app_handle: &AppHandle) -> AppSettings {
        let app_settings_path = Self::get_settings_path(app_handle);
        let settings_content = fs::read_to_string(app_settings_path);

        if let Ok(settings) = settings_content {
            info!("Settings file found... loading");
            serde_json::from_str(&settings).unwrap_or_else(|_| {
                error!("Failed to parse settings file, using defaults");
                AppSettings::default()
            })
        } else {
            info!("Settings file not found, using defaults");
            AppSettings::default()
        }
    }

    pub fn save_settings(app_handle: &AppHandle, settings: &AppSettings) -> anyhow::Result<()> {
        let app_settings_path = Self::get_settings_path(app_handle);

        if let Some(parent) = app_settings_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(app_settings_path, serde_json::to_string_pretty(&settings)?)?;

        Ok(())
    }
}
