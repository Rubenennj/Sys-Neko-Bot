use crate::core::event_handler::NekoEventHandler;
use crate::NekoClient;
use serde_json::Value;
use serenity::async_trait;
use serenity::client::bridge::gateway::event::ShardStageUpdateEvent;
use serenity::client::{Context, EventHandler};
use serenity::model::channel::{
    Channel, ChannelCategory, GuildChannel, Message, PartialGuildChannel, Reaction, StageInstance,
};
use serenity::model::event::{
    ChannelPinsUpdateEvent, GuildMemberUpdateEvent, GuildMembersChunkEvent, InviteCreateEvent,
    InviteDeleteEvent, MessageUpdateEvent, PresenceUpdateEvent, ResumedEvent, ThreadListSyncEvent,
    ThreadMembersUpdateEvent, TypingStartEvent, VoiceServerUpdateEvent,
};
use serenity::model::gateway::{Presence, Ready};
use serenity::model::guild::{
    Emoji, Guild, GuildUnavailable, Integration, Member, PartialGuild, Role, ThreadMember,
};
use serenity::model::id::{
    ApplicationId, ChannelId, EmojiId, GuildId, IntegrationId, MessageId, RoleId,
};
use serenity::model::prelude::{CurrentUser, User, VoiceState};
use std::collections::HashMap;
use crate::handling::command_handler::command_handler;

#[async_trait]
impl EventHandler for NekoEventHandler {
    async fn ready(&self, _ctx: Context, data: Ready) {
        println!(
            "Ready on client {} and loaded {} commands",
            data.user.tag(),
            NekoClient::commands().len()
        );
    }

    async fn message(&self, ctx: Context, msg: Message) {
        command_handler(&self.client, &ctx, &msg).await;
    }
}
