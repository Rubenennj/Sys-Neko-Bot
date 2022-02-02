use crate::NekoClient;
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;
use std::error::Error;

#[async_trait]
pub trait Command: Send + Sync {
    fn name(&self) -> &'static str;

    fn category(&self) -> &'static str;

    async fn execute(
        &self,
        client: &NekoClient,
        ctx: &Context,
        message: &Message,
        args: &Vec<String>,
    ) -> Result<(), String> {
        todo!()
    }
}
