mod file;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{args::Args, APP_NAME};

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

        config_builder = config_builder
            .add_source(config::Environment::with_prefix(APP_NAME))
            .add_source(args.clone());

        let config = config_builder.build()?.try_deserialize()?;

        Ok(config)
    }
}

impl config::Source for Args {
    fn clone_into_box(&self) -> Box<dyn config::Source + Send + Sync> {
        Box::new(self.clone())
    }

    fn collect(
        &self,
    ) -> std::result::Result<config::Map<String, config::Value>, config::ConfigError> {
        let mut config = config::Map::new();

        if let Some(openai_token) = &self.openai_token {
            config.insert(
                "openai_token".to_owned(),
                config::Value::from(openai_token.clone()),
            );
        }

        Ok(config)
    }
}
