use std::sync::Mutex;

use gigachat_models::{AppSettings, UpdateIgnoreEventsArgs};
use tauri::AppHandle;
use twitch_irc::{login::StaticLoginCredentials, SecureTCPTransport, TwitchIRCClient};

use crate::{
    commands::update_settings_state,
    services::SettingsService,
    state::app_settings_state::{toggle_always_on_top_state, toggle_ui_lock_state},
    utils::{close_settings_window, emit_toggle_ui_lock},
};

#[tauri::command]
pub fn save_settings(
    app_handle: AppHandle,
    settings_state: tauri::State<'_, Mutex<AppSettings>>,
    irc_client_state: tauri::State<
        '_,
        Mutex<TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>>,
    >,
    new_settings: AppSettings,
) -> Result<(), ()> {
    let old_settings = update_settings_state(settings_state, new_settings.clone());

    SettingsService::save_settings(&app_handle, &new_settings).expect("Failed to save settings");

    debug!("Saving settings current: {:?}", old_settings);
    debug!("Saving settings new: {:?}", new_settings);

    if old_settings.channel_name != new_settings.channel_name {
        info!("changing channel to: {}", new_settings.channel_name);

        let irc_client = irc_client_state.lock().unwrap();

        if !old_settings.channel_name.is_empty() {
            irc_client.part(old_settings.channel_name)
        }

        let _ = irc_client.join(new_settings.channel_name);
    }

    toggle_ui_lock_state(&app_handle, true);
    toggle_always_on_top_state(&app_handle, new_settings.always_on_top);

    close_settings_window(&app_handle);

    emit_toggle_ui_lock(
        &app_handle,
        UpdateIgnoreEventsArgs {
            ignore_events: true,
        },
    );

    Ok(())
}
