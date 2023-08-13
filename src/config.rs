use std::path::Path;

use anyhow::Result;
use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};
use xdg::BaseDirectories;

static APP_NAME: &str = env!("CARGO_PKG_NAME");
static CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Debug, Clone)]
pub struct ConfigData {
    pub token: String,
}

impl ConfigData {
    pub fn load() -> Result<Self> {
        let raw_config = RawConfig::load()?;

        let token = match raw_config.token_type {
            TokenType::Plain => raw_config.token,
            TokenType::OnePassword => todo!(),
        };

        Ok(Self { token })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RawConfig {
    token: String,

    #[serde(default, rename = "type")]
    token_type: TokenType,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum TokenType {
    #[default]
    Plain,

    #[serde(rename = "1password")]
    OnePassword,
}

impl RawConfig {
    fn load() -> Result<Self> {
        let xdg_dir = BaseDirectories::with_prefix(APP_NAME)?;
        let config_file = xdg_dir.get_config_file(Path::new(CONFIG_FILE_NAME));

        let config = Config::builder()
            .add_source(File::from(config_file))
            .add_source(Environment::with_prefix(APP_NAME.to_uppercase().as_str()))
            .build()?;

        config.try_deserialize().map_err(Into::into)
    }
}
