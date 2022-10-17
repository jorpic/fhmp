use anyhow::{anyhow, Context, Result};
use serde::ser::Serializer;
use serde::ser::SerializeSeq;
use crate::config::read_config;
use crate::db::active_notes;

pub fn exec() -> Result<()> {
    let cfg = read_config()
        .context("Reading config")?;
    let db = sqlite::open(&cfg.db_path)
        .context("Opening database file")?;

    let mut s = serde_yaml::Serializer::new(std::io::stdout());
    let mut ss = s.serialize_seq(None)?;
    for n in active_notes(&db)? {
        ss.serialize_element(&n)?;
    }
    ss.end().map_err(|e| anyhow!(e))
}
