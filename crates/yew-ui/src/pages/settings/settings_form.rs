use gigachat_models::AppSettings;
use log::info;
use stylist::yew::styled_component;
use tauri_sys::tauri::invoke;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, HtmlSelectElement};
use yew::{platform::spawn_local, prelude::*};

use super::SaveSettingsArgs;
use crate::pages::settings::{SettingsInputControl, SettingsTextInputControl};

#[derive(Clone, PartialEq, Properties)]
pub struct SettingsFormProps {
    pub initial_settings: AppSettings,
    pub on_save: Callback<AppSettings>,
}

async fn sync_settings(settings: AppSettings) {
    spawn_local(async move {
        let _: () = invoke(
            "sync_settings",
            &SaveSettingsArgs {
                new_settings: settings,
            },
        )
        .await
        .unwrap();
    });
}

#[styled_component(SettingsForm)]
pub fn settings_form(props: &SettingsFormProps) -> Html {
    let settings = use_state(|| props.initial_settings.clone());
    let channel_name = use_state(|| settings.channel_name.clone());

    use_effect_with_deps(
        move |settings| {
            let settings = (**settings).clone();

            spawn_local(async move {
                sync_settings(settings).await;
            });
        },
        settings.clone(),
    );

    let on_save = props.on_save.clone();
    let save = {
        let settings = settings.clone();
        let channel_name = channel_name.clone();

        Callback::from(move |_| {
            info!("Saving settings: {}", *channel_name.clone());

            on_save.emit(AppSettings {
                channel_name: (*channel_name).clone(),
                ..(*settings).clone()
            });
        })
    };

    let on_channel_name_change = {
        let channel_name = channel_name.clone();

        Callback::from(move |value: String| {
            channel_name.set(value);
        })
    };

    let on_background_color_change = {
        let settings = settings.clone();

        Callback::from(move |value: String| {
            settings.set(AppSettings {
                background_color: value,
                ..(*settings).clone()
            });
        })
    };

    let on_background_opacity_change = {
        let settings = settings.clone();

        Callback::from(move |value: String| {
            settings.set(AppSettings {
                background_opacity: value
                    .parse::<f32>()
                    .expect("Failed to parse background opacity value"),
                ..(*settings).clone()
            });
        })
    };

    let on_font_size_change = {
        let settings = settings.clone();

        Callback::from(move |value: String| {
            settings.set(AppSettings {
                font_size: value
                    .parse::<i32>()
                    .expect("Failed to parse font size value"),
                ..(*settings).clone()
            });
        })
    };

    let on_sort_change = {
        let settings = settings.clone();

        Callback::from(move |e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            let value = target.unchecked_into::<HtmlSelectElement>().value();
            let value = &value == "bottom";

            settings.set(AppSettings {
                messages_sort_asc: value,
                ..(*settings).clone()
            });
        })
    };

    let on_enable_animation_change = {
        let settings = settings.clone();

        Callback::from(move |e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");

            let checked = target.unchecked_into::<HtmlInputElement>().checked();

            settings.set(AppSettings {
                enable_animation: checked,
                ..(*settings).clone()
            });
        })
    };

    let slider_setting_value_style = css! {
        flex-grow: 0;
        flex-shrink: 0;
        flex-basis: 50px;
    };

    let label_style = css! {
        font-size: 16px;
        font-weight: 200;
        line-height: 38px;
    };

    html! {
        <div>
            <div class="mb-3 row">
                <label class={classes!("col-4", label_style.clone())} for="channel_name" >{"Channel Name"}</label>

                <div class="col-8">
                    <SettingsTextInputControl
                        id="channel_name"
                        class={classes!("form-control")}
                        on_change={on_channel_name_change.clone()}
                        on_input={on_channel_name_change.clone()}
                        value={(*channel_name).clone()}
                    />
                </div>
            </div>

            <div class="mb-3 row">
                <label class={classes!("col-4", label_style.clone())} for="new_messages_sort">{"Sorting"}</label>

                <div class="col-8">
                    <select
                        class="form-select"
                        id="new_messages_sort"
                        onchange={on_sort_change}
                    >
                        <option value="top" selected={!settings.messages_sort_asc}>{"New at the top"}</option>
                        <option value="bottom" selected={settings.messages_sort_asc}>{"New at the bottom"}</option>
                    </select>
                </div>
            </div>

            <div class="mb-3 row">
                <label class={classes!("col-4", label_style.clone())} for="background_color">{"Background Color"}</label>
                <div class="col-2">
                    <SettingsInputControl
                        input_type="color"
                        id="background_color"
                        class={classes!("form-control", "form-control-color", "p-0", "border-0")}
                        on_input={on_background_color_change}
                        value={settings.background_color.clone()} />
                </div>
            </div>

            <div class="mb-3 row">
                <label class={classes!("col-4", label_style.clone())} for="background_opacity" class="form-label">{"Background Opacity"}</label>

                <div class="col-8 d-flex align-items-center">
                    <div class={classes!(slider_setting_value_style.clone())}>
                        {(settings.background_opacity * 100_f32).floor().to_string()} {"%"}
                    </div>

                    <SettingsInputControl
                        input_type="range"
                        id="background_opacity"
                        class={classes!("form-range")}
                        on_input={on_background_opacity_change}
                        value={settings.background_opacity.to_string()}
                        step="0.01"
                        min="0"
                        max="1" />
                </div>
            </div>

            <div class="mb-3 row">
                <label class={classes!("col-4", label_style.clone())} for="font_size">{"Font Size"}</label>

                <div class="col-8 d-flex align-items-center">
                    <div class={classes!(slider_setting_value_style)}>
                        {settings.font_size.to_string()}
                    </div>

                    <SettingsInputControl
                        input_type="range"
                        id="font_size"
                        class={classes!("form-range")}
                        on_input={on_font_size_change}
                        value={settings.font_size.to_string()}
                        step="1"
                        min="8"
                        max="25" />
                </div>
            </div>

            <div class="mb-3 form-check">
                <input
                    class="form-check-input"
                    type="checkbox"
                    class="form-check-input"
                    id="enable_animation"
                    checked={settings.enable_animation}
                    onchange={on_enable_animation_change} />

                <label class={classes!("form-check-label")} for="enable_animation">{"Enable Animation"}</label>
            </div>

            <div class="d-flex justify-content-end">
                <button class="btn btn-primary" onclick={save}>{"Save"}</button>
            </div>
        </div>
    }
}
