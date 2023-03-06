use gigachat_models::AppSettings;
use serde::Serialize;

#[derive(Serialize)]
pub struct SaveSettingsArgs {
    #[serde(rename = "newSettings")]
    pub new_settings: AppSettings,
}
