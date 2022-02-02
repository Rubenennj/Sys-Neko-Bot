use crate::traits::command::Command;
use crate::NekoClient;
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;
use std::error::Error;
use std::ops::Sub;
use std::time::SystemTime;

pub struct PingCommand;

#[async_trait]
impl Command for PingCommand {
    fn name(&self) -> &'static str {
        "ping"
    }

    fn category(&self) -> &'static str {
        "info"
    }

    async fn execute(
        &self,
        client: &NekoClient,
        ctx: &Context,
        message: &Message,
    ) -> Result<(), String> {
        match message
            .channel_id
            .send_message(&ctx, |m| m.content("Pinging..."))
            .await {
            Ok(mut msg) => {
                let timestamp = msg.timestamp;

                match msg.edit(&ctx, |m| {
                    m.content(format!(
                        "Pong! {}ms",
                        SystemTime::now()
                            .duration_since(SystemTime::from(timestamp))
                            .unwrap()
                            .as_millis()
                    ))
                }).await {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e.to_string())
                }
            },
            Err(e) => Err(e.to_string())
        }
    }
}
