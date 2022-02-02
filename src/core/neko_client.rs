use crate::commands::info::ping::PingCommand;
use crate::traits::command::Command;

#[derive(Default)]
pub struct NekoClient {
    prefixes: Vec<String>,
}

impl NekoClient {
    pub fn new(prefixes: Vec<String>) -> Self {
        Self {
            prefixes,
            ..Self::default()
        }
    }

    pub fn commands() -> Vec<Box<dyn Command>> {
        vec![
            Box::new(PingCommand)
        ]
    }
}
