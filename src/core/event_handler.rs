use crate::core::neko_client::NekoClient;
use serenity::async_trait;
use serenity::client::EventHandler;

pub struct NekoEventHandler {
    pub client: NekoClient,
}

impl NekoEventHandler {
    pub fn new(client: NekoClient) -> Self {
        Self { client }
    }
}
