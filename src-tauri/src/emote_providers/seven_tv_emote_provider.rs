use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::state::saved_emotes_state::EmotesMap;

use super::EmoteProvider;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SevenTvEmoteOwnerRole {
    id: String,
    name: String,
    position: i32,
    color: i32,
    allowed: i32,
    denied: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SevenTvEmoteOwner {
    id: String,
    twitch_id: String,
    login: String,
    display_name: String,
    role: SevenTvEmoteOwnerRole,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SevenTvEmote {
    id: String,
    name: String,
    owner: SevenTvEmoteOwner,
    visibility: i32,
    visibility_simple: Vec<String>,
    mime: String,
    status: i32,
    tags: Vec<String>,
    width: Vec<i32>,
    height: Vec<i32>,
    urls: Vec<Vec<String>>,
}

fn map_seventv_emotes(emotes: Vec<SevenTvEmote>) -> EmotesMap {
    emotes
        .into_iter()
        .map(|emote| {
            let url = emote
                .urls
                .get(0)
                .expect("Failed to get at least one emote url");
            let url = url.get(1).expect("Failed to get 1x emote url");

            (emote.name, url.to_string())
        })
        .collect()
}

pub struct SevenTvEmoteProvider {
    reqwest_client: reqwest::Client,
}

impl SevenTvEmoteProvider {
    pub fn new() -> Self {
        Self {
            reqwest_client: reqwest::Client::new(),
        }
    }
}
impl Default for SevenTvEmoteProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EmoteProvider for SevenTvEmoteProvider {
    fn get_name(&self) -> &'static str {
        "7tv"
    }

    async fn get_channel_emotes<'a>(&self, twitch_user_id: &'a str) -> anyhow::Result<EmotesMap> {
        let client = self.reqwest_client.clone();

        let emotes = client
            .get(format!(
                "https://api.7tv.app/v2/users/{}/emotes",
                twitch_user_id
            ))
            .header("Accept", "application/json")
            .send()
            .await?
            .json::<Vec<SevenTvEmote>>()
            .await?;

        Ok(map_seventv_emotes(emotes))
    }

    async fn get_global_emotes(&self) -> anyhow::Result<EmotesMap> {
        let client = self.reqwest_client.clone();

        let emotes = client
            .get("https://api.7tv.app/v2/emotes/global")
            .header("Accept", "application/json")
            .send()
            .await?
            .json::<Vec<SevenTvEmote>>()
            .await?;

        Ok(map_seventv_emotes(emotes))
    }
}
