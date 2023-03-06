#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod commands;
mod emote_providers;
mod services;
mod state;
mod utils;

use crate::{
    commands::{app_ready, lock_ui, open_settings, save_settings, sync_settings, unlock_ui},
    emote_providers::EmoteProvider,
    services::EmotesService,
    state::{
        app_settings_state::{toggle_always_on_top_state, toggle_ui_lock_state},
        current_channel_state::{update_current_channel_state, CurrentChannel},
        saved_emotes_state::{
            reset_channel_emotes_state, update_channel_emotes_state, update_global_emotes_state,
            SavedEmotes,
        },
    },
    utils::{emit_new_message, emit_toggle_always_on_top, emit_toggle_ui_lock},
};
use dotenvy::dotenv;
use gigachat_models::{
    AppSettings, GigaChatMessage, GigaChatSender, UpdateAlwaysOnTopEventArgs,
    UpdateIgnoreEventsArgs,
};
use lazy_static::{__Deref, lazy_static};
use services::SettingsService;
use state::ui_lock_state::UiLockState;
use std::sync::Mutex;
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};
use tauri_plugin_window_state::{AppHandleExt, StateFlags};
use twitch_irc::{
    login::StaticLoginCredentials, message::ServerMessage, SecureTCPTransport, TwitchIRCClient,
};
use utils::open_settings_window;

lazy_static! {
    static ref EMOTE_PROVIDERS: Vec<Box<dyn EmoteProvider>> = {
        vec![
            Box::<emote_providers::better_ttv_emote_provider::BetterTtvEmoteProvider>::default(),
            Box::<emote_providers::seven_tv_emote_provider::SevenTvEmoteProvider>::default(),
            Box::<emote_providers::ffz_emote_provider::FfzEmoteProvider>::default(),
        ]
    };
}

fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    //Tray menu setup
    let settings = CustomMenuItem::new("settings", "Settings");
    let mut lock = CustomMenuItem::new("lock", "Lock");
    lock = lock.selected();
    let always_on_top = CustomMenuItem::new("always_on_top".to_string(), "Always On Top");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");

    let tray_menu = SystemTrayMenu::new()
        .add_item(settings)
        .add_item(always_on_top)
        .add_item(lock)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .manage(Mutex::new(SavedEmotes::default()))
        .manage(Mutex::new(CurrentChannel::default()))
        .manage(UiLockState::new(true))
        .invoke_handler(tauri::generate_handler![
            app_ready,
            save_settings,
            unlock_ui,
            lock_ui,
            open_settings,
            sync_settings
        ])
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                match id.as_str() {
                    "settings" => {
                        let app_handle = app.app_handle();
                        let _settings_window = open_settings_window(&app_handle);

                        toggle_ui_lock_state(&app_handle, false);
                        emit_toggle_ui_lock(
                            &app_handle,
                            UpdateIgnoreEventsArgs {
                                ignore_events: false,
                            },
                        );
                    }
                    "always_on_top" => {
                        let app_handle = app.app_handle();
                        let app_settings_state = app_handle.state::<Mutex<AppSettings>>();
                        let app_settings_state = app_settings_state.lock().unwrap();
                        let new_always_on_top_state = !app_settings_state.always_on_top;
                        drop(app_settings_state);

                        toggle_always_on_top_state(&app_handle, new_always_on_top_state);

                        emit_toggle_always_on_top(
                            &app_handle,
                            UpdateAlwaysOnTopEventArgs {
                                always_on_top: new_always_on_top_state,
                            },
                        );
                    }
                    "lock" => {
                        let app_handle = app.app_handle();
                        let ui_locked_state = app_handle.state::<UiLockState>();
                        let ui_locked_state = ui_locked_state.lock().unwrap();
                        let new_lock_value = !*ui_locked_state;
                        drop(ui_locked_state);

                        toggle_ui_lock_state(&app_handle, new_lock_value);

                        emit_toggle_ui_lock(
                            &app_handle,
                            UpdateIgnoreEventsArgs {
                                ignore_events: new_lock_value,
                            },
                        );
                    }
                    "quit" => {
                        let app_handle = app.app_handle();
                        let _ = app_handle.save_window_state(StateFlags::all());
                        app_handle.exit(0);
                    }
                    _ => {}
                }
            }
        })
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            let _ = main_window.set_ignore_cursor_events(true);

            let app_handle = app.handle();

            let settings = SettingsService::get_settings(&app_handle);
            info!("Startup Settings: {:?}", settings);

            app_handle.manage::<Mutex<AppSettings>>(Mutex::new(settings.clone()));

            tauri::async_runtime::spawn(async move {
                let config = twitch_irc::ClientConfig::default();
                let (mut incoming_messages, client) = twitch_irc::TwitchIRCClient::<
                    twitch_irc::SecureTCPTransport,
                    twitch_irc::login::StaticLoginCredentials,
                >::new(config);

                app_handle
                    .manage::<Mutex<TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>>>(
                        Mutex::new(client.clone()),
                    );

                let app_handle_clone = app_handle.clone();
                tokio::spawn(async move {
                    let global_emotes =
                        EmotesService::get_global_emotes(EMOTE_PROVIDERS.deref()).await;

                    update_global_emotes_state(&app_handle_clone, global_emotes);
                });

                // Initial app state based on settings
                if settings.always_on_top {
                    toggle_always_on_top_state(&app_handle, true);
                }

                if !settings.channel_name.is_empty() {
                    let _ = client.join(settings.channel_name);
                } else {
                    open_settings_window(&app_handle);
                    toggle_ui_lock_state(&app_handle, false);
                }

                let join_handle = tokio::spawn(async move {
                    while let Some(message) = incoming_messages.recv().await {
                        match message {
                            ServerMessage::RoomState(msg) => {
                                update_current_channel_state(
                                    &app_handle,
                                    Some(msg.channel_id.clone()),
                                    Some(msg.channel_login.clone()),
                                );

                                reset_channel_emotes_state(&app_handle);

                                let app_handle_clone = app_handle.clone();
                                tokio::spawn(async move {
                                    let channel_emotes = EmotesService::get_channel_emotes(
                                        EMOTE_PROVIDERS.deref(),
                                        &msg.channel_id,
                                    )
                                    .await;

                                    update_channel_emotes_state(&app_handle_clone, channel_emotes);
                                });
                            }

                            ServerMessage::Part(msg) => {
                                update_current_channel_state(&app_handle, None, None);
                                reset_channel_emotes_state(&app_handle);

                                info!("Part: {:?}", msg);
                            }

                            ServerMessage::Privmsg(msg) => {
                                let mut emotes = {
                                    let emotes_state = app_handle.state::<Mutex<SavedEmotes>>();
                                    let emotes_state = emotes_state.lock().unwrap();
                                    (*emotes_state).clone()
                                };

                                let message_emotes =
                                    EmotesService::parse_twitch_message_emotes(&msg);
                                emotes.channel_emotes.extend(message_emotes);

                                let message_with_emotes = EmotesService::replace_message_emotes(
                                    &emotes,
                                    &msg.message_text,
                                );

                                debug!("message_with_emotes {:?}", message_with_emotes);

                                let color = match msg.name_color {
                                    Some(color) => color.to_string(),
                                    None => "#999999".to_string(),
                                };

                                let chat_msg = GigaChatMessage {
                                    sender: GigaChatSender::new(
                                        msg.sender.login,
                                        msg.sender.name,
                                        color,
                                    ),
                                    message: msg.message_text.to_string(),
                                    message_html: message_with_emotes,
                                    message_id: msg.message_id.to_string(),
                                    timestamp: msg.server_timestamp,
                                };

                                emit_new_message(chat_msg, &app_handle);
                            }
                            _ => {
                                trace!("unmatched: {:?}", message);
                            }
                        }
                    }
                });

                join_handle.await.unwrap();
            });

            Ok(())
        })
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
