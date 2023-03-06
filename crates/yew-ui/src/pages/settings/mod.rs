mod form_controls;
mod save_settings_args;
mod settings_form;

use bounce::use_atom;
use gigachat_models::AppSettings;
use log::info;
use settings_form::SettingsForm;

use stylist::yew::styled_component;
use tauri_sys::tauri::invoke;
use wasm_bindgen_futures::spawn_local;
use yew::{html, Callback, Html};

pub use form_controls::{SettingsInputControl, SettingsTextInputControl};
pub use save_settings_args::SaveSettingsArgs;

#[styled_component(SettingsPage)]
pub fn settings_page() -> Html {
    let app_settings = use_atom::<AppSettings>();

    let save_settings = {
        let app_settings = app_settings.clone();

        Callback::from(move |settings: AppSettings| {
            let prev_settings = (*app_settings).clone();
            info!("previous settings: {:?}", prev_settings);

            gloo_console::info!("previous settings: {:?}", prev_settings.enable_animation);

            let new_settings = AppSettings {
                channel_name: settings.channel_name,
                background_color: settings.background_color,
                background_opacity: settings.background_opacity,
                font_size: settings.font_size,
                enable_animation: settings.enable_animation,
                messages_sort_asc: settings.messages_sort_asc,
                ..prev_settings
            };

            app_settings.set(new_settings.clone());

            spawn_local(async move {
                let _: () = invoke("save_settings", &SaveSettingsArgs { new_settings })
                    .await
                    .unwrap();
            });
        })
    };

    let style = css! {
        height: 100%;
        padding: 20px 15px 20px;
        background: #3c3c3c;
        color: #fafafa;
    };

    html! {
        <div data-tauri-drag-region="true" class={style}>
            <SettingsForm initial_settings={(*app_settings).clone()} on_save={save_settings} />
        </div>
    }
}
