use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub language: Language,
    pub colors: Colorscheme,
}

#[derive(Serialize, Deserialize)]
pub enum Language {
    English,
    Spanish,
}

#[derive(Serialize, Deserialize)]
pub struct Colorscheme {
    pub borders: String,
    pub titles: String,
    pub text: String,
    pub accent: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: String::from("John Doe"),
            language: Language::English,
            colors: Colorscheme {
                borders: String::from("#a6e3a1"),
                titles: String::from("#a6e3a1"),
                text: String::from("#cdd6f4"),
                accent: String::from("#f5c2e7"),
            },
        }
    }
}

pub fn load_config() -> Result<Config> {
    let cfg: Config = confy::load("tasker", "tasker_tui")?;

    Ok(cfg)
}
