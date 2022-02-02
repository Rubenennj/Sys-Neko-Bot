use serenity::client::Context;
use serenity::model::guild::Member;
use serenity::model::id::{GuildId, UserId};
use serenity::Result;

pub async fn fetch_member(ctx: &Context, guild_id: GuildId, user_id: UserId) -> Result<Member> {
    match ctx.cache.member(&guild_id, &user_id).await {
        Some(m) => Ok(m),
        None => match ctx.http.get_member(guild_id.0, user_id.0).await {
            Ok(m) => {
                let mut guild = ctx.cache.guild(&guild_id).await.unwrap();

                guild.members.insert(user_id, m.clone());

                Ok(m)
            }
            Err(e) => Err(e),
        },
    }
}
