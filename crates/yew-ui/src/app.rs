use bounce::use_atom;
use futures::StreamExt;
use gigachat_models::AppSettings;
use log::info;
use stylist::yew::styled_component;
use tauri_sys::{event::listen, tauri::invoke};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yew_router::prelude::*;

use crate::{
    pages::{ChatPage, SettingsPage},
    route::Route,
};

fn switch_route(route: Route) -> Html {
    match route {
        Route::Chat => html! { <ChatPage /> },
        Route::Settings => html! { <SettingsPage /> },
    }
}

#[styled_component(App)]
pub fn app() -> Html {
    let app_settings = use_atom::<AppSettings>();
    let settings_initialized = use_state(|| false);

    let style = css! {
        r#"
            position: relative;
            width: 100%;
            height: 100%;
        "#
    };

    {
        let app_settings = app_settings.clone();
        let settings_initialized = settings_initialized.clone();

        use_effect_once(move || {
            spawn_local(async move {
                let mut stream = listen::<AppSettings>("init_settings").await.unwrap();
                let _: () = invoke("app_ready", &()).await.unwrap();

                while let Some(app_settings_event) = stream.next().await {
                    let app_settings_val = app_settings_event.payload;

                    info!("Initializing settings: {:?}", app_settings_val);

                    app_settings.set(app_settings_val);
                    settings_initialized.set(true);
                }
            });

            || {}
        });
    }

    use_effect_once(move || {
        spawn_local(async move {
            let mut stream = listen::<AppSettings>("sync_settings").await.unwrap();

            while let Some(new_settings_event) = stream.next().await {
                info!("sync_settings event: {:?}", new_settings_event);

                let new_settings = new_settings_event.payload;

                app_settings.set(new_settings);
            }
        });

        || {}
    });

    if !*settings_initialized {
        html! {
            <div>
                {"Loading..."}
            </div>
        }
    } else {
        html! {
            <div class={classes!(style)}>
                <BrowserRouter>
                    <Switch<Route> render={switch_route} />
                </BrowserRouter>
            </div>
        }
    }
}
