use std::collections::VecDeque;

use futures::StreamExt;
use gigachat_models::{GigaChatMessage, GigaChatSender};
use stylist::yew::styled_component;
use tauri_sys::event::listen;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlElement, ScrollBehavior, ScrollIntoViewOptions};
use yew::{
    html, use_effect_with_deps, use_node_ref, use_state, Html, NodeRef, Properties, UseStateHandle,
};
use yew_hooks::{use_effect_once, use_latest};

use crate::pages::chat::twitch_chat_message::TwitchChatMessage;

#[derive(Clone, Debug)]
struct MessagesQueue {
    capacity: usize,
    sort_asc: bool,
    data: VecDeque<GigaChatMessage>,
}

impl MessagesQueue {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            sort_asc: false,
            data: VecDeque::with_capacity(capacity),
        }
    }

    fn push(&mut self, message: GigaChatMessage) {
        if self.data.len() == self.capacity - 1 {
            if self.sort_asc {
                self.data.remove(0);
            } else {
                self.data.pop_back();
            }
        }

        if self.sort_asc {
            self.data.push_back(message);
        } else {
            self.data.push_front(message);
        }
    }

    fn set_sort(&mut self, sort_asc: bool) {
        self.sort_asc = sort_asc;

        self.data = self.data.clone().into_iter().rev().collect();
    }
}

impl Default for MessagesQueue {
    fn default() -> Self {
        Self::new(10)
    }
}

impl PartialEq for MessagesQueue {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

#[derive(Clone, Debug, PartialEq)]
struct ScrollListDependents {
    scroll_to_last_ref: NodeRef,
    scroll_to_first_ref: NodeRef,
    messages: UseStateHandle<MessagesQueue>,
    scroll_asc: bool,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TwitchMessagesListProps {
    #[prop_or(true)]
    pub sort_asc: bool,
}

#[styled_component(TwitchMessagesList)]
pub fn twitch_messages_list(props: &TwitchMessagesListProps) -> Html {
    let init_messages = vec![
        GigaChatMessage::new(
            GigaChatSender::new(
                "test_login".to_string(),
                "test_display_name".to_string(),
                "#999999".to_string(),
            ),
            "message1".to_string(),
            "<span>message1</span>".to_string(),
            "message1_id".to_string(),
            chrono::DateTime::<chrono::Utc>::default(),
        ),
        GigaChatMessage::new(
            GigaChatSender::new(
                "test_login".to_string(),
                "test_display_name2".to_string(),
                "#999999".to_string(),
            ),
            "message2".to_string(),
            "<span>message2</span>".to_string(),
            "message2_id".to_string(),
            chrono::DateTime::<chrono::Utc>::default(),
        ),
        GigaChatMessage::new(
            GigaChatSender::new(
                "test_login2".to_string(),
                "test_display_name3".to_string(),
                "red".to_string(),
            ),
            "message3".to_string(),
            "<span>message3</span>".to_string(),
            "message3_id".to_string(),
            chrono::DateTime::<chrono::Utc>::default(),
        ),
        GigaChatMessage::new(
            GigaChatSender::new(
                "test_login2".to_string(),
                "test_display_name4".to_string(),
                "red".to_string(),
            ),
            "message4".to_string(),
            "<span>message4</span>".to_string(),
            "message4_id".to_string(),
            chrono::DateTime::<chrono::Utc>::default(),
        ),
    ];
    let mut init_queue = MessagesQueue::new(50);
    for message in init_messages {
        init_queue.push(message);
    }
    let messages: UseStateHandle<MessagesQueue> = use_state(|| init_queue);
    let scroll_to_last_ref = use_node_ref();
    let scroll_to_first_ref = use_node_ref();
    let latest_messages = use_latest(messages.clone());

    {
        let messages = messages.clone();
        let latest_messages = latest_messages.clone();

        use_effect_once(move || {
            spawn_local(async move {
                let mut stream = listen::<GigaChatMessage>("new_message").await.unwrap();
                while let Some(new_message_event) = stream.next().await {
                    let message = new_message_event.payload;

                    let mut messages_cloned = (**latest_messages.current()).clone();
                    messages_cloned.push(message);

                    messages.set(messages_cloned);
                }
            });

            || {}
        });
    }

    {
        // Update sort
        let messages = messages.clone();
        let latest_messages = latest_messages;

        use_effect_with_deps(
            move |sort_asc| {
                let mut messages_cloned = (**latest_messages.current()).clone();
                messages_cloned.set_sort(*sort_asc);

                messages.set(messages_cloned);
            },
            props.sort_asc,
        );
    }

    {
        // Scroll to latest message
        let scroll_to_last_ref = scroll_to_last_ref.clone();
        let scroll_to_first_ref = scroll_to_first_ref.clone();

        use_effect_with_deps(
            move |scroll_list_deps| {
                let mut scroll_options = ScrollIntoViewOptions::default();
                scroll_options.behavior(ScrollBehavior::Smooth);

                if scroll_list_deps.scroll_asc {
                    scroll_list_deps
                        .scroll_to_last_ref
                        .cast::<HtmlElement>()
                        .unwrap()
                        .scroll_into_view_with_scroll_into_view_options(&scroll_options);
                } else {
                    scroll_list_deps
                        .scroll_to_first_ref
                        .cast::<HtmlElement>()
                        .unwrap()
                        .scroll_into_view_with_scroll_into_view_options(&scroll_options);
                }
            },
            ScrollListDependents {
                scroll_to_last_ref,
                scroll_to_first_ref,
                messages: messages.clone(),
                scroll_asc: props.sort_asc,
            },
        );
    }

    let container_style = css!(
        r#"
            background: transparent;
            padding: 5px 3px;
        "#,
    );

    html! {
        <div class={container_style}>
            <div ref={scroll_to_first_ref}></div>

            {
                messages.data.iter().map(|message| html! {
                    <TwitchChatMessage
                        key={message.message_id.clone()}
                        sender_name={message.sender.display_name.clone()}
                        sender_color={message.sender.color.clone()}
                        message={message.message_html.clone()}
                        message_id={message.message_id.clone()}
                        timestamp={message.timestamp} />
                }).collect::<Html>()
            }

            <div ref={scroll_to_last_ref}></div>
        </div>
    }
}
