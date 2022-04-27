use std::{io, env, path::PathBuf};

use anyhow::{Context, Result};
use config::{Config, File, FileFormat};

use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};
use serde_yaml::Value;

use uuid::Uuid;

#[derive(Deserialize)]
struct CliConfig {
    pub db_path: String,
}

#[derive(Serialize,Deserialize)]
struct Note {
    tags: String,
    ctime: Option<DateTime<Local>>,
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
    let notes = match val {
        Value::Sequence(_) =>
            Vec::<Note>::deserialize(val)
                .context("Whlie parsing a sequence of notes from YAML")?,
        Value::Mapping(_) => vec![
            Note::deserialize(val)
                .context("While parsing a note from YAML")?

        ],
        _ => anyhow::bail!("Object or sequence of objects expected.")
    };

    // init db
    let db = sqlite::open(&cfg.db_path)?;
    db.execute(
        "create table if not exists notes(
            uuid text not null,
            ctime text not null,
            json text not null);
        "
    )?;

    // insert notes
    let mut q = db.prepare("insert into notes values (?, ?, ?)")?;
    for n in notes.iter() {
        q.reset()?;
        let id = Uuid::new_v4().to_string();
        let ctime = match n.ctime {
            Some(ctime) => ctime.to_rfc3339(),
            None => Local::now().to_rfc3339(),
        };
        let data = serde_json::to_string(&n)?;

        q.bind(1, id.as_str())?;
        q.bind(2, ctime.as_str())?;
        q.bind(3, data.as_str())?;
        while let sqlite::State::Row = q.next()? { };
    }
    Ok(())
}
