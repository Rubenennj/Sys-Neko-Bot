use crate::NekoClient;
use serenity::client::Context;
use serenity::model::channel::Message;

pub async fn command_handler(client: &NekoClient, ctx: &Context, msg: &Message) {
    if msg.author.bot || msg.guild_id.is_none() {
        return;
    }

    if let Some(prefix) = client
        .prefixes
        .iter()
        .find(|n| msg.content.starts_with(n.as_str()))
    {
        let mut args = msg.content[prefix.len()..]
            .split(' ')
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();

        if args.is_empty() {
            return;
        }

        let cmd = args.remove(0);

        match NekoClient::commands().iter().find(|c| c.name().eq(&cmd)) {
            Some(command) => {
                let _ = command.execute(client, ctx, msg, &args).await;
            }
            None => return,
        }
    } else {
        return;
    }
}
