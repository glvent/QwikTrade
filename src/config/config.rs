use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub window: WindowConfig
}

#[derive(Debug, Deserialize)]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub vsync: bool,
    pub resizable: bool,
}

pub fn load_config() -> Config {
    let json_path = Path::new("src/config/config.json");

    // TODO: implement logging and error handling
    let parsed_json = fs::read_to_string(json_path).unwrap_or_else(|_err| {
        String::new()
    });

    if !parsed_json.is_empty() {
        if let Ok(conf) = serde_json::from_str::<Config>(&parsed_json) {
            return conf;
        }
    }

    // Return a default config if something goes wrong...
    Config {
        window: WindowConfig {
            width: 1200,
            height: 800,
            title: "QwikTrade".to_owned(),
            vsync: true,
            resizable: true
        },
    }
}