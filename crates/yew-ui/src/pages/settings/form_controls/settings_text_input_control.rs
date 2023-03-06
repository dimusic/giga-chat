use super::SettingsInputControl;
use yew::{function_component, html, AttrValue, Callback, Classes, Html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct SettingsTextInputControlProps {
    pub value: AttrValue,

    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub on_change: Option<Callback<String>>,
    #[prop_or_default]
    pub on_input: Option<Callback<String>>,
    #[prop_or_default]
    pub id: AttrValue,
}

#[function_component(SettingsTextInputControl)]
pub fn settings_input_control(props: &SettingsTextInputControlProps) -> Html {
    html! {
        <SettingsInputControl
            input_type="text"
            id={props.id.clone()}
            class={props.class.clone()}
            on_change={props.on_change.clone()}
            on_input={props.on_input.clone()}
            value={props.value.clone()} />
    }
}
