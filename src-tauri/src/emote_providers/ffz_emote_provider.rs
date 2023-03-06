use std::collections::HashMap;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::state::saved_emotes_state::EmotesMap;

use super::EmoteProvider;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FfzEmote {
    pub id: i64,
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub public: bool,
    pub hidden: bool,
    pub modifier: bool,
    pub urls: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FfzEmoteSet {
    pub id: i64,
    pub _type: i32,
    pub title: String,
    pub emoticons: Vec<FfzEmote>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FfzEmotesSetResponse {
    pub default_sets: Vec<i64>,
    pub sets: HashMap<String, FfzEmoteSet>,
    //pub users
}

fn filter_hidden_emotes(emotes: Vec<FfzEmote>) -> Vec<FfzEmote> {
    emotes
        .into_iter()
        .filter(|emote| !emote.hidden)
        .collect::<Vec<FfzEmote>>()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FfzRoomResponse {
    //pub room
    pub sets: HashMap<String, FfzEmoteSet>,
}

pub struct FfzEmoteProvider {
    reqwest_client: reqwest::Client,
}

impl FfzEmoteProvider {
    pub fn new() -> Self {
        Self {
            reqwest_client: reqwest::Client::new(),
        }
    }
}

impl Default for FfzEmoteProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EmoteProvider for FfzEmoteProvider {
    fn get_name(&self) -> &'static str {
        "FFZ"
    }

    async fn get_global_emotes(&self) -> anyhow::Result<EmotesMap> {
        let client = self.reqwest_client.clone();

        let global_sets_response = client
            .get("https://api.frankerfacez.com/v1/set/global")
            .header("Accept", "application/json")
            .send()
            .await?
            .json::<FfzEmotesSetResponse>()
            .await?;

        let emotes = global_sets_response
            .default_sets
            .into_iter()
            .flat_map(|s| {
                let default_set = global_sets_response
                    .sets
                    .iter()
                    .find(|&(_, set)| s == set.id);

                if let Some((_, set)) = default_set {
                    filter_hidden_emotes(set.emoticons.clone())
                } else {
                    Vec::new()
                }
            })
            .flat_map(|emote| {
                let onex_url = emote.urls.get("1");

                if let Some(url) = onex_url {
                    Some((emote.name, url.to_string()))
                } else {
                    None
                }
            })
            .collect();

        Ok(emotes)
    }

    async fn get_channel_emotes<'a>(&self, channel_id: &'a str) -> anyhow::Result<EmotesMap> {
        let client = self.reqwest_client.clone();

        let room_response = client
            .get(&format!(
                "https://api.frankerfacez.com/v1/room/id/{}",
                channel_id
            ))
            .header("Accept", "application/json")
            .send()
            .await?
            .json::<FfzRoomResponse>()
            .await?;

        let emotes: EmotesMap = room_response
            .sets
            .into_iter()
            .flat_map(|(_, set)| filter_hidden_emotes(set.emoticons))
            .flat_map(|emote| {
                let onex_url = emote.urls.get("1");

                if let Some(url) = onex_url {
                    Some((emote.name, url.to_string()))
                } else {
                    None
                }
            })
            .collect();

        Ok(emotes)
    }
}
