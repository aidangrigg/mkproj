use std::fs;

use anyhow::{Context, Result};
use home::home_dir;
use serde::{Deserialize, Serialize};

const CONFIG_FILE_PATH: &str = ".config/mkproj/config.toml";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub template_dir: Option<String>,
}

impl Config {
    fn default() -> Self {
        Self { template_dir: None }
    }

    pub fn load() -> Result<Config> {
        let mut config_path = home_dir().context("Impossible to get your home dir!")?;
        config_path.push(CONFIG_FILE_PATH);

        if config_path.exists() {
            let content = fs::read_to_string(config_path).context("Could not read config file!")?;
            let config = toml::from_str(&content).context("Could not parse config file!")?;

            return Ok(config);
        }

        let config = Config::default();
        config.save()?;

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let mut config_path = home_dir().context("Impossible to get your home dir!")?;
        config_path.push(CONFIG_FILE_PATH);

        if !config_path.parent().unwrap().exists() {
            fs::create_dir_all(config_path.parent().unwrap())
                .context("Could not create config file")?;
        }

        let toml = toml::to_string(self).context("Failed to convert toml to string")?;

        fs::write(config_path, toml).context("Could not create config file")?;
        Ok(())
    }
}
