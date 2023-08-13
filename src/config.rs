use std::path::Path;

use anyhow::Result;
use config::{Config, File};
use serde::{Deserialize, Serialize};
use xdg::BaseDirectories;

static APP_NAME: &str = env!("CARGO_PKG_NAME");
static CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Debug, Clone)]
pub struct ConfigData {
    pub openai_token: String,
}

impl ConfigData {
    pub fn load() -> Result<Self> {
        let raw_config = RawConfig::load()?;

        let token = match raw_config.openai_token {
            OpenAIToken::Plain { token } => token,
            OpenAIToken::OnePassword { .. } => todo!(),
        };

        Ok(Self {
            openai_token: token,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RawConfig {
    openai_token: OpenAIToken,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
enum OpenAIToken {
    Plain {
        token: String,
    },

    #[serde(rename = "1password")]
    OnePassword {
        token: String,
    },
}

impl RawConfig {
    fn load() -> Result<Self> {
        let xdg_dir = BaseDirectories::with_prefix(APP_NAME)?;
        let config_file = xdg_dir.get_config_file(Path::new(CONFIG_FILE_NAME));

        let config = Config::builder()
            .add_source(File::from(config_file))
            .build()?;

        config.try_deserialize().map_err(Into::into)
    }
}
