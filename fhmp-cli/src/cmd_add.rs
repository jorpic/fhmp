use std::io;
use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

use crate::config::read_config;
use crate::note::{InputNote, DbNote};
use crate::db::{init_schema, insert_notes};

pub fn cmd_add() -> Result<()> {
    let cfg = read_config()
        .context("Reading config")?;

    let db = sqlite::open(&cfg.db_path)
        .context("Opening database file")?;
    init_schema(&db)
        .context("Initializing database schema")?;
    let notes = read_notes(io::stdin())
        .context("Reading notes from stdin")?;
    let notes = transform_notes(&notes)
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

fn transform_notes(notes: &[InputNote]) -> Result<Vec<DbNote>>
{
    let mut res = Vec::new();
    let mut err = Vec::new();

    for r in notes.iter().map(InputNote::to_db_note) {
        match r {
            Ok(n) => res.push(n),
            Err(e) => err.push(e),
        }
    }

    if err.is_empty() {
        Ok(res)
    } else {
        let err: String = err.iter()
            .map(|s| "- ".to_string() + &s.to_string() + "\n")
            .collect();
        Err(anyhow!(err))
    }
}
