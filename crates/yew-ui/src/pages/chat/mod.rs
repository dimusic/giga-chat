use bounce::use_atom_value;
use futures::StreamExt;
use gigachat_models::{AppSettings, UpdateIgnoreEventsArgs};
use log::info;
use stylist::yew::styled_component;
use tauri_sys::{event::listen, tauri::invoke};
use yew::{html, platform::spawn_local, use_callback, use_state, Html, UseStateHandle};
use yew_hooks::use_effect_once;

use crate::{
    components::layout::drag_overlay::DragOverlay,
    pages::chat::twitch_messages_list::TwitchMessagesList, utils::colors::hex_to_rgb,
};

mod twitch_chat_message;
pub mod twitch_messages_list;

#[derive(Clone, Debug, PartialEq)]
struct SaveAlwaysOnTopDeps {
    always_on_top: UseStateHandle<bool>,
    settings: UseStateHandle<AppSettings>,
}

#[styled_component(ChatPage)]
pub fn chat_page() -> Html {
    let app_settings = use_atom_value::<AppSettings>();
    let is_ui_locked = use_state(|| true);

    {
        let is_ui_locked = is_ui_locked.clone();

        use_effect_once(move || {
            spawn_local(async move {
                let mut stream = listen::<UpdateIgnoreEventsArgs>("toggle_ui_lock")
                    .await
                    .unwrap();

                while let Some(ui_locked_event) = stream.next().await {
                    let ui_locked_value = ui_locked_event.payload;
                    is_ui_locked.set(ui_locked_value.ignore_events);

                    info!("Updating ignore events settings: {:?}", ui_locked_value);
                }
            });

            || {}
        });
    }

    let unlock_ui = {
        let is_ui_locked = is_ui_locked.clone();

        use_callback(
            move |_e, is_ui_locked| {
                let new_lock_value = !**is_ui_locked;
                is_ui_locked.set(new_lock_value);

                spawn_local(async move {
                    let _: () = invoke("lock_ui", &()).await.unwrap();
                });
            },
            is_ui_locked,
        )
    };

    let background_rgb = hex_to_rgb(&app_settings.background_color[1..]);
    let style = css! {
        min-height: 100%;
        font-size: ${app_settings.font_size}px;
        background: rgba(${background_rgb.0}, ${background_rgb.1}, ${background_rgb.2}, ${app_settings.background_opacity});
    };

    html! {
        <div class={style}>
            {
                if !*is_ui_locked {
                    html!{<DragOverlay on_done={unlock_ui.clone()} />}
                }
                else {
                    html!{}
                }
            }

            <TwitchMessagesList sort_asc={app_settings.messages_sort_asc} />
        </div>
    }
}
