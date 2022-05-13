use anyhow::Result;
use chrono::{DateTime, Local};
use serde::Serialize;
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


#[derive(Serialize)]
pub struct DbNote {
    pub uuid: Uuid,
    pub ctime: DateTime<Local>,
    pub tags: String,
    pub data: Value,
}

pub fn insert_notes(
    db: &sqlite::Connection,
    notes: &[DbNote]
) -> Result<()> {
    let mut q = db.prepare("insert into notes values (?, ?, ?)")?;
    for n in notes.iter() {
        q.reset()?;
        let data = serde_json::to_string(&n.data)?;
        q.bind(1, n.uuid.to_string().as_str())?;
        q.bind(2, n.ctime.to_rfc3339().as_str())?;
        q.bind(3, data.as_str())?;
        while let sqlite::State::Row = q.next()? { };
    }
    Ok(())
}
