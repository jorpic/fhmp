use std::{io, env, path::PathBuf};

use anyhow::{Context, Result};
use config::{Config, ConfigError, File, FileFormat};

use chrono::{DateTime, Local};
use serde::Deserialize;
use serde_yaml::Value;

#[derive(Deserialize)]
struct CliConfig {
    pub db_path: String,
}

#[derive(Debug, Deserialize)]
struct Note {
    tags: String,
    ctime: DateTime<Local>,
    card: Option<Vec<String>>,
    text: Option<String>
}


fn main() -> Result<()> {
    // read config
    let home = env::var("HOME")
        .context("While trying to get $HOME")?;
    let cfg_path: PathBuf = [&home, ".config", "fhmp", "config"].iter().collect();
    let cfg: CliConfig = Config::builder()
        .add_source(
            File::from(cfg_path)
                .format(FileFormat::Toml)
                .required(true))
        .build()?
        .try_deserialize()
        .context("While parsing config")?;


    // read stdin
    let val: Value = serde_yaml::from_reader(io::stdin())
        .context("While reading YAML from stdin")?;
    match val {
        Value::Sequence(_) => {
            let notes = Vec::<Note>::deserialize(val)
                .context("Whlie parsing a sequence of notes from YAML")?;
            println!("{:?}", notes);
        },
        Value::Mapping(_) => {
            let note = Note::deserialize(val)
                .context("While parsing a note from YAML")?;
            println!("{:?}", note);
        },
        _ => {
            anyhow::bail!("Object or sequence of objects expected.");
        }
    }
    Ok(())
}
