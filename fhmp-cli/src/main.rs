use std::{io, env, path::PathBuf};

use anyhow::{anyhow, Context, Result};
use config::{Config, File, FileFormat};

use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};
use serde_json::json;

use uuid::Uuid;

#[derive(Deserialize)]
struct CliConfig {
    pub db_path: String,
}

#[derive(Serialize,Deserialize)]
struct InputNote {
    tags: String,
    ctime: Option<DateTime<Local>>,
    card: Option<Vec<String>>,
    text: Option<String>
}

#[derive(Serialize)]
struct DbNote {
    uuid: String,
    ctime: DateTime<Local>,
    tags: String,
    data: serde_json::Value
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
    let (_errors, notes) = check_notes(&notes);
    // FIXME: check errors
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

fn read_notes<T: std::io::Read>(r: T) -> Result<Vec<InputNote>> {
    let val = serde_yaml::from_reader(r)?;
    match val {
        serde_yaml::Value::Sequence(_) =>
            Vec::<InputNote>::deserialize(val)
                .context("Parsing a sequence of notes from YAML"),
        serde_yaml::Value::Mapping(_) => Ok(vec![
            InputNote::deserialize(val)
                .context("Parsing a note from YAML")?

        ]),
        _ => anyhow::bail!("Object or sequence of objects expected.")
    }
}

fn check_notes(notes: &[InputNote]) -> (Vec<anyhow::Error>, Vec<DbNote>)
{
    let mut db_notes = Vec::new();
    let mut errors = Vec::new();

    for n in notes.iter() {
        let mut data = serde_json::Map::new();

        if n.card == None && n.text == None {
            errors.push(
                anyhow!("`card` or `text` must present")
            );
        } else if n.card != None && n.text != None {
            errors.push(
                anyhow!("both `card` and `text` are present")
            );
        } else if let Some(card) = &n.card {
            data.insert("card".to_string(), json!(card));
        } else if let Some(text) = &n.text {
            data.insert("text".to_string(), json!(text));
        }

        // FIXME: update record if existing uuid provided
        db_notes.push(
            DbNote {
                uuid: Uuid::new_v4().to_string(),
                tags: "FIXME".to_string(),
                ctime: n.ctime.unwrap_or_else(|| Local::now()),
                data: json!(data),
            }
        )
    }

    (errors, db_notes)
}

fn init_db(db_path: &str) -> Result<sqlite::Connection> {
    let db = sqlite::open(db_path)?;
    db.execute(
        "create table if not exists notes(
            uuid text not null,
            ctime text not null,
            tags text not null,
            data json not null);
        "
    )?;
    Ok(db)
}


fn insert_notes(
    db: &sqlite::Connection,
    notes: &[DbNote]
) -> Result<()> {
    let mut q = db.prepare("insert into notes values (?, ?, ?)")?;
    for n in notes.iter() {
        q.reset()?;
        let data = serde_json::to_string(&n.data)?;
        q.bind(1, n.uuid.as_str())?;
        q.bind(2, n.ctime.to_rfc3339().as_str())?;
        q.bind(3, data.as_str())?;
        while let sqlite::State::Row = q.next()? { };
    }
    Ok(())
}
