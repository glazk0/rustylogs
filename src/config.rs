use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub discord: Discord,
    pub openai: OpenAI,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Discord {
    pub token: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OpenAI {
    pub api_key: String,
    pub prompt: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            discord: Discord {
                token: String::new(),
            },
            openai: OpenAI {
                api_key: String::new(),
                prompt: String::new(),
            },
        }
    }
}

impl Config {
    const FILE_NAME: &str = "config.toml";

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        if let Ok(file) = fs::read_to_string(Self::FILE_NAME) {
            let config: Config = toml::from_str(&file)?;
            Ok(config)
        } else {
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(Self::FILE_NAME, toml::to_string_pretty(self)?)?;
        Ok(())
    }
}
