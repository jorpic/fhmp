use std::io;

use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Local};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::config::read_config;
use crate::db::{init_db, insert_notes, DbNote};

#[derive(Deserialize)]
struct InputNote {
    tags: String,
    ctime: Option<DateTime<Local>>,
    card: Option<Vec<String>>,
    text: Option<String>
}

pub fn cmd_add() -> Result<()> {
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

fn read_notes<T: io::Read>(r: T) -> Result<Vec<InputNote>> {
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
                uuid: Uuid::new_v4(),
                tags: "FIXME".to_string(),
                ctime: n.ctime.unwrap_or_else(Local::now),
                data: json!(data),
            }
        )
    }

    (errors, db_notes)
}

