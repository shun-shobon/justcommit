use std::env;
use std::path::PathBuf;

use serde::Deserialize;

use crate::APP_NAME;

#[cfg(not(target_os = "windows"))]
static CONFIG_HOME_VAR: &str = "XDG_CONFIG_HOME";
#[cfg(target_os = "windows")]
static CONFIG_HOME_VAR: &str = "APPDATA";

static CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Debug, Deserialize)]
pub struct Config {
    pub openai_token: String,
}

impl Config {
    pub fn load() -> Option<Self> {
        let mut builder = config::Config::builder();

        if let Some(home_dir) = dirs::home_dir() {
            let config_base_dir = home_dir.join(".config").join(APP_NAME);
            let config_file = config_base_dir.join(CONFIG_FILE_NAME);

            builder = builder.add_source(config::File::from(config_file));
        }

        if let Some(config_home_dir) = env::var_os(CONFIG_HOME_VAR) {
            let config_base_dir = PathBuf::from(config_home_dir).join(APP_NAME);
            let config_file = config_base_dir.join(CONFIG_FILE_NAME);

            builder = builder.add_source(config::File::from(config_file));
        }

        builder.build().ok()?.try_deserialize().ok()
    }
}
