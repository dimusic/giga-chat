use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::state::saved_emotes_state::EmotesMap;

use super::EmoteProvider;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BetterTtvEmote {
    pub id: String,
    pub code: String,
    #[serde(rename = "imageType")]
    pub image_type: String,
    pub animated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BetterTtvUserResponse {
    pub id: String,
    #[serde(skip_deserializing)]
    pub bots: Vec<String>,
    pub avatar: String,
    #[serde(rename = "channelEmotes")]
    pub channel_emotes: Vec<BetterTtvEmote>,
    #[serde(rename = "sharedEmotes")]
    pub shared_emotes: Vec<BetterTtvEmote>,
}

fn map_betterttv_emotes(emotes: Vec<BetterTtvEmote>) -> EmotesMap {
    emotes
        .into_iter()
        .map(|emote| {
            let url = format!("https://cdn.betterttv.net/emote/{}/1x.webp", emote.id);
            (emote.code, url)
        })
        .collect()
}

pub struct BetterTtvEmoteProvider {
    reqwest_client: reqwest::Client,
}

impl BetterTtvEmoteProvider {
    pub fn new() -> Self {
        Self {
            reqwest_client: reqwest::Client::new(),
        }
    }
}

impl Default for BetterTtvEmoteProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EmoteProvider for BetterTtvEmoteProvider {
    fn get_name(&self) -> &'static str {
        "BetterTTV"
    }

    async fn get_global_emotes(&self) -> anyhow::Result<EmotesMap> {
        let client = self.reqwest_client.clone();

        let emotes = client
            .get("https://api.betterttv.net/3/cached/emotes/global")
            .header("Accept", "application/json")
            .send()
            .await?
            .json::<Vec<BetterTtvEmote>>()
            .await?;

        Ok(map_betterttv_emotes(emotes))
    }

    async fn get_channel_emotes<'a>(&self, channel_id: &'a str) -> anyhow::Result<EmotesMap> {
        let client = self.reqwest_client.clone();

        let user_response = client
            .get(&format!(
                "https://api.betterttv.net/3/cached/users/twitch/{channel_id}"
            ))
            .header("Accept", "application/json")
            .send()
            .await?
            .json::<BetterTtvUserResponse>()
            .await?;

        let mut emotes = map_betterttv_emotes(user_response.shared_emotes);
        emotes.extend(map_betterttv_emotes(user_response.channel_emotes));

        Ok(emotes)
    }
}
