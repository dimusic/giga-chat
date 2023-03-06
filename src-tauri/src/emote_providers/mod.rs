use async_trait::async_trait;

use crate::state::saved_emotes_state::EmotesMap;

pub mod better_ttv_emote_provider;
pub mod ffz_emote_provider;
pub mod seven_tv_emote_provider;

#[async_trait]
pub trait EmoteProvider: Send + Sync {
    fn get_name(&self) -> &'static str;

    async fn get_global_emotes(&self) -> anyhow::Result<EmotesMap>;

    async fn get_channel_emotes<'a>(&self, twitch_user_id: &'a str) -> anyhow::Result<EmotesMap>;
}
