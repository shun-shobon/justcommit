use std::{env, path::PathBuf, process::Command};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[cfg(not(target_os = "windows"))]
static CONFIG_HOME_VAR: &str = "XDG_CONFIG_HOME";
#[cfg(target_os = "windows")]
static CONFIG_HOME_VAR: &str = "APPDATA";

static APP_NAME: &str = env!("CARGO_PKG_NAME");
static CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Debug, Clone, Copy)]
pub struct Format;

impl config::Format for Format {
    fn parse(
        &self,
        _uri: Option<&String>,
        text: &str,
    ) -> std::result::Result<
        config::Map<String, config::Value>,
        Box<dyn std::error::Error + Send + Sync>,
    > {
        let mut config = config::Map::new();

        let file_config = toml::from_str::<ConfigFile>(text)?;
        let openai_token = file_config.openai_token.read()?;

        config.insert("openai_token".to_owned(), config::Value::from(openai_token));

        Ok(config)
    }
}

impl config::FileStoredFormat for Format {
    fn file_extensions(&self) -> &'static [&'static str] {
        &["toml"]
    }
}

pub(super) fn get_config_path() -> Option<PathBuf> {
    env::var_os(CONFIG_HOME_VAR)
        .map(PathBuf::from)
        .or_else(|| {
            dirs::home_dir().map(|mut p| {
                p.push(".config");
                p
            })
        })
        .map(|p| p.join(APP_NAME).join(CONFIG_FILE_NAME))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct ConfigFile {
    pub(super) openai_token: OpenAIToken,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub(super) enum OpenAIToken {
    Plain {
        token: String,
    },

    #[serde(rename = "1password")]
    OnePassword {
        token: String,
    },
}

impl OpenAIToken {
    pub(super) fn read(&self) -> Result<String> {
        match self {
            Self::Plain { token } => Ok(token.clone()),
            Self::OnePassword { token: token_ref } => {
                let output = Command::new("op")
                    .args(["read", "--no-newline", token_ref])
                    .output()
                    .context("Failed to run 1Password CLI")?;

                if !output.status.success() {
                    anyhow::bail!(
                        "Failed to get OpenAI API token from 1Password: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }

                let token = String::from_utf8(output.stdout)
                    .context("Failed to parse 1Password output as UTF-8")?;

                Ok(token)
            }
        }
    }
}
