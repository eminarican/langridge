use config::{Config, ConfigError, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Secrets {
    pub discord: String,
    pub deepl: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Room {
    pub id: u64,
    pub lang: String,
    pub webhook: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub secrets: Secrets,
    pub groups: Vec<Vec<Room>>
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("Settings.json"))
            .build()?;

        config.try_deserialize()
    }
}
