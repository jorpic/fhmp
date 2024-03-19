use anyhow::{Context, Result};
use config::{File, FileFormat};
use serde::Deserialize;
use std::env;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Config {
    pub issues_path: PathBuf,
}

pub fn read_config() -> Result<Config> {
    let home = env::var("HOME").context("Get $HOME env var")?;
    let cfg_path: PathBuf =
        [&home, ".config", "fhmp", "config.toml"].iter().collect();
    let cfg_file = File::from(cfg_path).format(FileFormat::Toml).required(true);
    config::Config::builder()
        .add_source(cfg_file)
        .build()?
        .try_deserialize()
        .context("Read config file")
}
