use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;

use sha3::{Shake128, digest::{Update, ExtendableOutput, XofReader}};

pub fn init_db(db_path: &str) -> Result<sqlite::Connection> {
    let db = sqlite::open(db_path)?;
    // Why we have two ids
    // Why datetimes are strings?
    db.execute("
        create table if not exists notes(
            hash text primary key,
            uuid text not null,
            ctime text not null,
            parent text references notes(hash),
            tags text not null,
            data json not null
        );
        create index if not exists notes_uuid_ix
            on notes(uuid);

        create table if not exists events(
            id text primary key,
            note_uuid text not null references notes(uuid),
            ctime text not null,
            data json not null
        );

        create table if not exists review_queue(
            note_hash text not null unique references notes(hash),
            event_id text not null references events,
            next_review text not null,
            tags text not null
        );
        create index if not exists review_queue_next_review_ix
            on review_queue(next_review);
    ")?;
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

fn shake128(s: &String) -> String {
    let mut hasher = Shake128::default();
    hasher.update(s.as_bytes());
    let mut reader = hasher.finalize_xof();
    let mut hash = [0u8; 16];
    reader.read(&mut hash);
    hex::encode(hash)
}

pub fn insert_notes(
    db: &sqlite::Connection,
    notes: &[DbNote]
) -> Result<()> {
    let mut q = db.prepare("
        insert into notes
          (hash, uuid, ctime, tags, data)
        values
          (?, ?, ?, ?, ?)
    ")?;
    for n in notes.iter() {
        q.reset()?;
        let data = serde_json::to_string(&n.data)?;
        let hash = shake128(&data);
        q.bind(1, hash.as_str())?;
        q.bind(2, n.uuid.to_string().as_str())?;
        q.bind(3, n.ctime.to_rfc3339().as_str())?;
        q.bind(4, n.tags.as_str())?;
        q.bind(5, data.as_str())?;
        while let sqlite::State::Row = q.next()? { };
    }
    Ok(())
}

pub fn select_notes_for_review(
    db: &sqlite::Connection
) -> Result<Vec<DbNote>> {
    let mut q = db.prepare(
        "select
            uuid, ctime, tags, data
            from notes
            order by random()
            limit 10"
    )?;

    let mut res = Vec::new();
    while let sqlite::State::Row = q.next()? {
        res.push(DbNote {
            uuid:
                Uuid::parse_str(q.read::<String>(0)?.as_str())?,
            ctime:
                DateTime::parse_from_rfc3339(q.read::<String>(1)?.as_str())?
                    .with_timezone(&Utc),
            tags:
                q.read::<String>(2)?,
            data:
                serde_json::from_str(q.read::<String>(3)?.as_str())?,
        });
    };

    Ok(res)
}
