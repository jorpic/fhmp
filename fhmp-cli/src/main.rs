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

#[derive(Serialize,Deserialize)]
struct NoteData {
    card: Option<Vec<String>>,
    text: Option<String>
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    match &args[..] {
        [_, cmd] => match cmd.as_str() {
            "add" => cmd_add(),
            "rnd" => cmd_rnd(),
            "dump" => cmd_dump(),
            _ => anyhow::bail!("Invalid command '{}'", cmd),
        },
        _ => {
            println!("Usage:");
            println!("\tfhmp add − read YAML from stdin.");
            println!("\tfhmp rnd − review random note from DB.");
            anyhow::bail!("Invalid arguments.");
        }
    }
}

fn cmd_add() -> Result<()> {
    let cfg = read_config()
        .context("Reading config")?;
    let db = init_db(&cfg.db_path)
        .context("Opening database file")?;
    let notes = read_notes(io::stdin())
        .context("Reading notes from stdin")?;
    insert_notes(&db, &notes)
}

fn cmd_rnd() -> Result<()> {
    let cfg = read_config()
        .context("Reading config")?;
    let _db = init_db(&cfg.db_path)
        .context("Opening database file")?;
    // FIXME
    Ok(())
}

fn cmd_dump() -> Result<()> {
    let cfg = read_config()
        .context("Reading config")?;
    let _db = init_db(&cfg.db_path)
        .context("Opening database file")?;
    // FIXME
    Ok(())
}

fn read_config() -> Result<CliConfig> {
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

fn read_notes<T: std::io::Read>(r: T) -> Result<Vec<Note>> {
    let val: Value = serde_yaml::from_reader(r)?;
    match val {
        Value::Sequence(_) =>
            Vec::<Note>::deserialize(val)
                .context("Parsing a sequence of notes from YAML"),
        Value::Mapping(_) => Ok(vec![
            Note::deserialize(val)
                .context("Parsing a note from YAML")?

        ]),
        _ => anyhow::bail!("Object or sequence of objects expected.")
    }
}

fn init_db(db_path: &str) -> Result<sqlite::Connection> {
    let db = sqlite::open(db_path)?;
    db.execute(
        "create table if not exists notes(
            uuid text not null,
            ctime text not null,
            json text not null);
        "
    )?;
    Ok(db)
}


fn insert_notes(
    db: &sqlite::Connection,
    notes: &[Note]
) -> Result<()> {
    let mut q = db.prepare("insert into notes values (?, ?, ?)")?;
    for n in notes.iter() {
        q.reset()?;
        let id = Uuid::new_v4().to_string();
        let ctime = n.ctime
            .unwrap_or_else(|| Local::now())
            .to_rfc3339();
        let data = serde_json::to_string(&n)?;

        q.bind(1, id.as_str())?;
        q.bind(2, ctime.as_str())?;
        q.bind(3, data.as_str())?;
        while let sqlite::State::Row = q.next()? { };
    }
    Ok(())
}
