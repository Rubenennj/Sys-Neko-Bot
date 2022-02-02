#![allow(dead_code, unused, non_snake_case)]

mod commands;
mod core;
mod events;
mod functions;
mod handling;
mod structures;
mod tests;
mod traits;

use crate::core::event_handler::NekoEventHandler;
use crate::core::neko_client::NekoClient;
use serde_json::{from_str, Value};
use serenity::client::bridge::gateway::GatewayIntents;
use serenity::client::ClientBuilder;
use std::fs::{canonicalize, File};
use std::io::Read;

#[tokio::main]
async fn main() {
    let mut str = String::new();

    File::open(canonicalize("./src/config.json").unwrap())
        .unwrap()
        .read_to_string(&mut str);

    let json: &mut Value = &mut from_str(&str).unwrap();

    let token = json["token"].as_str().unwrap();

    let mut prefixes = json["prefixes"].as_array().unwrap().clone();

    let id = json["client_id"].as_u64().unwrap();

    prefixes.push(Value::String(format!("<@{}>", &id)));
    prefixes.push(Value::String(format!("<@!{}>", &id)));

    let mut builder = ClientBuilder::new(token)
        .application_id(id)
        .event_handler(NekoEventHandler::new(NekoClient::new(
            (*prefixes)
                .iter()
                .map(|n| n.as_str().unwrap().to_owned())
                .collect(),
        )))
        .intents(GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES)
        .await
        .unwrap();

    if let Err(err) = builder.start().await {
        println!("{}", err)
    }
}
