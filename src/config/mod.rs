mod file;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::args::Args;

use self::file::{get_config_path, Format};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub openai_token: String,
}

impl Config {
    pub fn load(args: &Args) -> Result<Self> {
        let mut config_builder = config::Config::builder();

        if let Some(config_file) = get_config_path() {
            config_builder = config_builder.add_source(config::File::new(
                // `config::File::new()` が何故か `&Path` / `&PathBuf` を受け付けないので、`&str` に変換する
                config_file.to_str().unwrap(),
                Format,
            ));
        }

        let config = config_builder.build()?.try_deserialize()?;

        Ok(config)
    }
}
