use serenity::model::id::UserId;
use serenity::model::user::User;
use serenity::prelude::Context;
use serenity::Result;

pub async fn fetch_user(ctx: &Context, id: UserId) -> Result<User> {
    match ctx.cache.user(&id).await {
        Some(u) => Ok(u),
        None => match ctx.http.get_user(id.0).await {
            Ok(u) => {
                ctx.cache.users.write().await.insert(id, u.clone());

                Ok(u)
            }
            Err(e) => Err(e),
        },
    }
}
