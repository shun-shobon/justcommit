use std::{env, path::PathBuf, process::Command};

use anyhow::{Context, Result};
use config::{Config, File};
use serde::{Deserialize, Serialize};

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
            OpenAIToken::OnePassword { token: token_ref } => {
                let output = Command::new("op")
                    .arg("read")
                    .arg("--no-newline")
                    .arg(token_ref)
                    .output()?;
                String::from_utf8(output.stdout)?
            }
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
        let config_dir = get_config_base_dir_by_os()
            .or_else(|| {
                dirs::home_dir().map(|mut path| {
                    path.push(".config");
                    path
                })
            })
            .map(|p| p.join(APP_NAME))
            .context("Failed to get config base directory")?;
        let config_file = config_dir.join(CONFIG_FILE_NAME);

        let config = Config::builder()
            .add_source(File::from(config_file))
            .build()?;

        config.try_deserialize().map_err(Into::into)
    }
}

fn get_config_base_dir_by_os() -> Option<PathBuf> {
    #[cfg(any(unix, target_os = "redox"))]
    {
        env::var("XDG_CONFIG_HOME").map(PathBuf::from).ok()
    }

    #[cfg(windows)]
    {
        env::var("APPDATA").map(PathBuf::from).ok()
    }
}
