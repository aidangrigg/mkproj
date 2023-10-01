use anyhow::{Context, Result};
use home::home_dir;

const CONFIG_PATH: &str = ".config/mkproj/config.toml";

fn main() -> Result<()> {
    let mut config_path = home_dir().context("Could not find users home dir")?;
    config_path.push(CONFIG_PATH);

    mkproj::run(&config_path)?;
    Ok(())
}
