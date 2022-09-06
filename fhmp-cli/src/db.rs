use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use sha3::{Shake128, digest::{Update, ExtendableOutput, XofReader}};

pub fn init_schema(db: &sqlite::Connection) -> Result<()> {
    db.execute("
        -- This table is a dictionary of note statuses.
        create table if not exists note_status(
            id integer primary key,
            label text
        );

        insert or ignore into note_status (id, label) values
            (1, 'active'),  -- is ready for review
            (2, 'retired'); -- was updated by a newer version or deleted

        -- This table holds our notes with some metadata.
        -- Notes are read-only. Hash of tags+data is used as primary key.
        -- When updating a note, a new version is created and the original one
        -- is marked as 'retired'.
        -- UUID is used to link versions together and track history of updates.
        create table if not exists notes(
            hash text primary key,
            uuid text not null,
            ctime text not null,
            mtime text,
            tags text not null,
            data json not null,
            status integer not null references note_status(id) default 1
        );
        create index if not exists notes_uuid_ix
            on notes(uuid);

        create view if not exists current_notes as
            select * from notes
                where status = 1;

        -- This table holds history of reviews.
        -- Each row references some note and contains review outcome.
        create table if not exists review(
            id integer primary key,
            note_id text not null references notes(uuid),
            ctime text not null,
            result text not null,  -- FIXME: dictionary
            decision json not null -- free form details of decision
        );

        -- Queue is used to select notes that are due for review.
        create table if not exists queue(
            note_id text not null unique references notes(uuid),
            next_review text not null
        );
        create index if not exists queue_next_review_ix
            on queue(next_review);
    ").map_err(|e| anyhow!(e))
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
    // ctime is stored as a RFC3339 formatted string with UTC timezone
    // in the hope that it will be easier to use and manipulate.
    pub ctime: DateTime<Utc>,
    pub tags: String,
    pub data: NoteData,
}

fn shake128(tags: &str, data: &str) -> String {
    let mut hasher = Shake128::default();
    hasher.update(tags.as_bytes());
    hasher.update(data.as_bytes());
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
        let hash = shake128(&n.tags, &data);
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
