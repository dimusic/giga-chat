use std::rc::Rc;

use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{
    html::{onchange::Wrapper as OnChangeWrapper, oninput::Wrapper as OnInputWrapper, *},
    prelude::*,
    virtual_dom::VTag,
};

#[derive(Clone, PartialEq, Properties)]
pub struct SettingsInputControlProps {
    pub on_change: Option<Callback<String>>,
    pub on_input: Option<Callback<String>>,
    pub value: AttrValue,

    #[prop_or("text".into())]
    pub input_type: AttrValue,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or_default]
    pub disabled: Option<bool>,

    #[prop_or(None)]
    pub min: Option<AttrValue>,

    #[prop_or(None)]
    pub max: Option<AttrValue>,

    #[prop_or(None)]
    pub step: Option<AttrValue>,
}

#[styled_component(SettingsInputControl)]
pub fn settings_input_control(props: &SettingsInputControlProps) -> Html {
    let on_change = {
        let on_change_prop = props.on_change.clone();

        Callback::from(move |e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");

            let value = target.unchecked_into::<HtmlInputElement>().value();

            if let Some(on_change_prop) = on_change_prop.clone() {
                on_change_prop.emit(value);
            }
        })
    };

    let on_input = {
        let on_input_prop = props.on_input.clone();

        Callback::from(move |e: InputEvent| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");

            let value = target.unchecked_into::<HtmlInputElement>().value();

            if let Some(on_input_prop) = on_input_prop.clone() {
                on_input_prop.emit(value);
            }
        })
    };

    let class_names = classes!(props.class.clone());

    let mut input_field = VTag::new("input");
    input_field.add_attribute("type", props.input_type.clone());
    input_field.add_attribute("class", class_names.to_string());
    input_field.add_attribute("id", props.id.clone());
    input_field.set_value(props.value.clone());

    if let Some(disabled) = props.disabled {
        input_field.add_attribute("disabled", disabled.to_string());
    }

    if let Some(min) = props.min.clone() {
        input_field.add_attribute("min", min);
    }

    if let Some(max) = props.max.clone() {
        input_field.add_attribute("max", max);
    }

    if let Some(step) = props.step.clone() {
        input_field.add_attribute("step", step);
    }

    if props.on_change.clone().is_some() {
        input_field.add_listener(Rc::new(OnChangeWrapper::new(on_change)));
    }

    if props.on_input.clone().is_some() {
        input_field.add_listener(Rc::new(OnInputWrapper::new(on_input)));
    }

    input_field.into()

    // html! {
    //     <input
    //         type={props.input_type.clone()}
    //         class={classes!(props.class.clone())}
    //         id={props.id.clone()}
    //         value={props.value.clone()}
    //         onchange={on_change}
    //         oninput={on_input} />
    // }
}
