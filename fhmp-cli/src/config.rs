use std::{env, path::PathBuf};
use anyhow::{Context, Result};
use config::{Config, File, FileFormat};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CliConfig {
    pub db_path: String,
}

pub fn read_config() -> Result<CliConfig> {
    let home = env::var("HOME")
        .context("Trying to get $HOME")?;
    let cfg_path: PathBuf = [&home, ".config", "fhmp", "config"].iter().collect();
    Config::builder()
        .add_source(
            File::from(cfg_path)
                .format(FileFormat::Toml)
                .required(true))
        .build()?
        .try_deserialize()
        .context("Parsing config")
}
