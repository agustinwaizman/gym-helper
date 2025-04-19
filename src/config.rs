use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
    pub api_bind: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_file(path: &str) -> Self {
        let config_str = fs::read_to_string(Path::new(path))
            .expect("Failed to read env.json");
        serde_json::from_str(&config_str)
            .expect("Failed to parse env.json")
    }
}