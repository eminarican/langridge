use figment::{Figment, providers::{Format, Json, Env}};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Secrets {
    pub discord: String,
    pub deepl: String,
}

#[derive(Debug, Deserialize)]
pub struct Room {
    pub id: u64,
    pub lang: String,
    pub webhook: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub secrets: Secrets,
    pub groups: Vec<Vec<Room>>
}

impl Settings {
    pub fn new() -> figment::error::Result<Self> {
        Figment::new()
            .merge(Json::file("Settings.json"))
            .merge(Env::prefixed("LANGRIDGE_"))
            .extract()
    }
}
