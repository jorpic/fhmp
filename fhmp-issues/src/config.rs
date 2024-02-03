use anyhow::{Context, Result};
use config::{Config, File, FileFormat};
use serde::Deserialize;
use std::{env, path::PathBuf};

#[derive(Deserialize)]
pub struct IssuesConfig {
    pub issues_path: PathBuf,
}

pub fn read_config() -> Result<IssuesConfig> {
    let home = env::var("HOME").context("Get $HOME env var")?;
    let cfg_path: PathBuf = [&home, ".config", "fhmp", "config"].iter().collect();
    Config::builder()
        .add_source(File::from(cfg_path).format(FileFormat::Toml).required(true))
        .build()?
        .try_deserialize()
        .context("Parse config file")
}
