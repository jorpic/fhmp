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
    let notes = check_notes(&notes)
        .context("Invalid note format")?;
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

fn check_notes(notes: &[InputNote]) -> Result<Vec<DbNote>>
{
    let mut res = Vec::new();
    let mut err = Vec::new();

    for n in notes.iter() {
        let mut data = serde_json::Map::new();

        if n.card == None && n.text == None {
            err.push("required fields `card` or `text` are not found");
        } else if n.card != None && n.text != None {
            err.push("both `card` and `text` are present");
        } else {
            if let Some(card) = &n.card {
                data.insert("card".to_string(), json!(card));
            } else if let Some(text) = &n.text {
                data.insert("text".to_string(), json!(text));
            }

            res.push(
                DbNote {
                    uuid: Uuid::new_v4(),
                    tags: n.tags.clone(), // FIXME: normalize tags somehow?
                    ctime: n.ctime.unwrap_or_else(Local::now),
                    data: json!(data),
                }
            )
        }
    }

    if err.is_empty() {
        Ok(res)
    } else {
        let err: String = err.iter()
            .map(|s| "- ".to_string() + s + "\n")
            .collect();
        Err(anyhow!(err))
    }
}
