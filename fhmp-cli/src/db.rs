use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;

pub fn init_db(db_path: &str) -> Result<sqlite::Connection> {
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NoteData {
    Text(String),
    Card(Vec<String>)
}

#[derive(Serialize)]
pub struct DbNote {
    pub uuid: Uuid,
    // We store ctime as a RFC3339 formatted string with UTC timezone
    // in the hope that it will be easier to compare them and extract
    // parts of them.
    pub ctime: DateTime<Utc>,
    pub tags: String,
    pub data: NoteData,
}


pub fn insert_notes(
    db: &sqlite::Connection,
    notes: &[DbNote]
) -> Result<()> {
    let mut q = db.prepare("insert into notes values (?, ?, ?, ?)")?;
    for n in notes.iter() {
        q.reset()?;
        let data = serde_json::to_string(&n.data)?;
        q.bind(1, n.uuid.to_string().as_str())?;
        q.bind(2, n.ctime.to_rfc3339().as_str())?;
        q.bind(3, n.tags.as_str())?;
        q.bind(4, data.as_str())?;
        while let sqlite::State::Row = q.next()? { };
    }
    Ok(())
}
