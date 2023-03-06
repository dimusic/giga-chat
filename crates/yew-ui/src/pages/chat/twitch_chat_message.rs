use bounce::use_atom_value;
use chrono::{DateTime, Local, Utc};
use gigachat_models::AppSettings;
use stylist::yew::use_style;

use yew::{
    classes, function_component, html, use_effect_with_deps, use_state, AttrValue, Html, Properties,
};

#[derive(Debug, PartialEq, Properties)]
pub struct TwitchChatMessageProps {
    pub sender_name: AttrValue,
    pub sender_color: AttrValue,
    pub message: AttrValue,
    pub message_id: AttrValue,
    pub timestamp: chrono::DateTime<Utc>,

    #[prop_or(false)]
    pub show_timestamp: bool,
}

#[derive(Debug, PartialEq, Properties)]
struct TwitchChatTimestampProps {
    pub timestamp: DateTime<Utc>,
}

#[function_component(TwitchChatTimestamp)]
fn twitch_chat_timestamp(props: &TwitchChatTimestampProps) -> Html {
    let timestamp_local: DateTime<Local> = chrono::DateTime::from(props.timestamp);

    html! {
        <span>
            {timestamp_local.format("%H:%M")}
        </span>
    }
}

#[function_component(TwitchChatMessage)]
pub fn twitch_chat_message(props: &TwitchChatMessageProps) -> Html {
    let container_style = use_style! {
        position: relative;
        left: 0;
        padding: 0.2rem;
        transition: 300ms;
        opacity: 1;

        &.appear-right-transition {
            opacity: 0;
            left: 220px;
        }

        img {
            vertical-align: middle;
            margin: -0.2rem 0;
        }

        .sender {
            font-weight: bold;
        }

        .message {
            text-shadow: 1px 1px 0 #000;
        }
    };

    let sender_color_style = use_style!(
        color: ${props.sender_color.clone()};
    );
    let init_class_name = use_state(|| "appear-right-transition");
    let app_settings = use_atom_value::<AppSettings>();

    {
        let init_class_name = init_class_name.clone();

        use_effect_with_deps(
            move |_message_id| {
                let animation_class_name = if app_settings.enable_animation {
                    "appear-right-transition"
                } else {
                    ""
                };

                let init_class_name = init_class_name.clone();
                init_class_name.set(animation_class_name);

                let timeout = gloo_timers::callback::Timeout::new(10, move || {
                    init_class_name.set("");
                });

                move || {
                    timeout.cancel();
                }
            },
            props.message_id.clone(),
        );
    }

    html! {
        <div class={classes!(container_style, *init_class_name.clone())} id={props.message_id.clone()}>
            {
                if props.show_timestamp {
                    html! { <TwitchChatTimestamp timestamp={props.timestamp} /> }
                }
                else {
                    html!{}
                }
            }

            {" "}
            <span class={classes!(sender_color_style, "sender")}>{ props.sender_name.clone() }</span>
            {": "}
            <span class="message">
                { Html::from_html_unchecked(props.message.clone()) }
            </span>
        </div>
    }
}
