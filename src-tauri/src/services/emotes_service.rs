use std::collections::HashMap;

use futures::{future::join_all, Future};
use twitch_irc::message::PrivmsgMessage;

use crate::{
    emote_providers::EmoteProvider,
    state::saved_emotes_state::{EmotesMap, SavedEmotes},
};

#[derive(Clone, PartialEq, Eq)]
enum MessageTag {
    Emote,
    PlainText,
}

async fn get_all_provider_emotes<'a, Fut>(
    providers: &'a [Box<dyn EmoteProvider + 'static>],
    request_fn: impl Fn(&'a dyn EmoteProvider) -> Fut,
) -> EmotesMap
where
    Fut: Future<Output = Result<EmotesMap, anyhow::Error>> + Send,
{
    let callback_ref = &request_fn;

    let requests = providers.iter().map(|provider| async move {
        let provider_emotes = callback_ref(&**provider).await;
        if provider_emotes.is_err() {
            error!(
                "Failed to fetch emotes from {}: {:?}",
                provider.get_name(),
                provider_emotes
            );

            return None;
        }

        let provider_emotes = provider_emotes.unwrap();

        info!(
            "Fetched {} emotes from {}",
            provider_emotes.len(),
            provider.get_name()
        );
        trace!("Emotes list: {:?}", provider_emotes);

        Some(provider_emotes)
    });

    join_all(requests)
        .await
        .into_iter()
        .flatten()
        .flatten()
        .collect::<EmotesMap>()

    // The code below should work as well?? but produces higher-ranked lifetime error
    // in places where this function is used
    // More info: https://github.com/rust-lang/rust/issues/102211
    // and https://github.com/rust-lang/rust/issues/71671
    //
    // futures::stream::iter(providers)
    //     .map(|provider| async move {
    //         let provider_emotes = callback_ref(provider).await;
    //
    //         if provider_emotes.is_err() {
    //             error!("Failed to fetch global emotes from {}", provider.get_name());
    //             return None;
    //         }
    //
    //         let provider_emotes = provider_emotes.unwrap();
    //
    //         info!(
    //             "Fetched {} global emotes from {}",
    //             provider_emotes.len(),
    //             provider.get_name()
    //         );
    //         trace!("Emotes list: {:?}", provider_emotes);
    //
    //         Some(provider_emotes)
    //     })
    //     .buffer_unordered(10)
    //     .fold(HashMap::new(), |mut acc, provider_emotes| async move {
    //         if let Some(provider_emotes) = provider_emotes {
    //             acc.extend(provider_emotes);
    //         }
    //
    //         acc
    //     })
    //     .await
}

pub struct EmotesService {}

impl EmotesService {
    pub async fn get_global_emotes(
        providers: &[Box<dyn EmoteProvider + 'static>],
    ) -> HashMap<String, String> {
        info!("Fetching all global emotes");

        get_all_provider_emotes(providers, |provider: &dyn EmoteProvider| {
            provider.get_global_emotes()
        })
        .await
    }

    pub async fn get_channel_emotes(
        providers: &[Box<dyn EmoteProvider + 'static>],
        channel_id: &str,
    ) -> HashMap<String, String> {
        info!("Fetching channel emotes for {}", channel_id);

        get_all_provider_emotes(providers, |provider: &dyn EmoteProvider| {
            provider.get_channel_emotes(channel_id)
        })
        .await
    }

    pub fn parse_twitch_message_emotes(msg: &PrivmsgMessage) -> EmotesMap {
        msg.emotes
            .iter()
            .map(|emote| {
                let emote_url = format!(
                    "https://static-cdn.jtvnw.net/emoticons/v2/{}/default/dark/1.0",
                    emote.id.clone()
                );

                (emote.code.clone(), emote_url)
            })
            .collect()
    }

    // Returns message html with replaced empote images
    pub fn replace_message_emotes(emotes_map: &SavedEmotes, message: &str) -> String {
        let mut html_str = "".to_string();
        let mut last_tag: Option<MessageTag> = None;

        message.split_whitespace().for_each(|word| {
            let mut found_emote = emotes_map.global_emotes.get(word);

            if found_emote.is_none() {
                found_emote = emotes_map.channel_emotes.get(word);
            }

            if let Some(emote_url) = found_emote {
                let pre_str: &str = last_tag.clone().map_or("", |t| {
                    if t == MessageTag::PlainText {
                        " </span>"
                    } else {
                        "<span> </span>"
                    }
                });

                html_str.push_str(&format!(
                    "{}<div class=\"d-inline\"><img src=\"{}\" alt=\"{}\" /></div>",
                    pre_str, emote_url, word
                ));

                last_tag = Some(MessageTag::Emote);
            } else {
                let mut pre_str: String = last_tag.clone().map_or("<span>".to_string(), |t| {
                    if t == MessageTag::Emote {
                        "<span>".to_string()
                    } else {
                        "".to_string()
                    }
                });

                if last_tag.is_some() {
                    pre_str.push(' ');
                }

                html_str.push_str(&format!("{}{}", pre_str, word));

                last_tag = Some(MessageTag::PlainText);
            }
        });

        if let Some(MessageTag::PlainText) = last_tag {
            html_str.push_str("</span>");
        }

        html_str
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn get_emotes_map() -> EmotesMap {
        let mut emotes_map = HashMap::new();
        emotes_map.insert("Kappa".to_string(), "_kappa_url_".to_string());
        emotes_map.insert("Pog".to_string(), "_pog_url_".to_string());

        emotes_map
    }

    fn get_saved_emotes() -> SavedEmotes {
        SavedEmotes {
            global_emotes: get_emotes_map(),
            channel_emotes: HashMap::new(),
        }
    }

    #[test]
    fn test_replace_message_emotes_plain_text_with_emote_after() {
        let message = "hello there Kappa";
        let emotes_map = get_saved_emotes();

        let result = EmotesService::replace_message_emotes(&emotes_map, message);

        assert_eq!(
            result,
            "<span>hello there </span><div class=\"d-inline\"><img src=\"_kappa_url_\" alt=\"Kappa\" /></div>"
        );
    }

    #[test]
    fn test_replace_message_emotes_emote_only() {
        let emotes_map = get_saved_emotes();

        let message = "Kappa";

        let result = EmotesService::replace_message_emotes(&emotes_map, message);

        assert_eq!(
            result,
            "<div class=\"d-inline\"><img src=\"_kappa_url_\" alt=\"Kappa\" /></div>"
        );
    }

    #[test]
    fn test_replace_message_emotes_plain_text_with_emote_before() {
        let emotes_map = get_saved_emotes();

        let message = "Kappa hello there";

        let result = EmotesService::replace_message_emotes(&emotes_map, message);

        assert_eq!(
            result,
            "<div class=\"d-inline\"><img src=\"_kappa_url_\" alt=\"Kappa\" /></div><span> hello there</span>"
        );
    }

    #[test]
    fn test_replace_message_emotes_multiple_emotes() {
        let emotes_map = get_saved_emotes();

        let message = "Kappa Pog";

        let result = EmotesService::replace_message_emotes(&emotes_map, message);

        assert_eq!(
            result,
            "<div class=\"d-inline\"><img src=\"_kappa_url_\" alt=\"Kappa\" /></div><span> </span><div class=\"d-inline\"><img src=\"_pog_url_\" alt=\"Pog\" /></div>"
        );
    }
}
