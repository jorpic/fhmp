use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::note::DbNote;

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
        -- Notes are read-only. Hash of tags+data is used as a primary key.
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
        -- Each row references some note and contains a review outcome.
        create table if not exists review(
            id integer primary key,
            note_id text not null references notes(uuid),
            ctime text not null,
            result text not null,  -- FIXME: dictionary of possible results?
            decision json not null -- free form details of the decision
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

struct RetireNoteQuery<'l>(sqlite::Statement<'l>);

impl<'l> RetireNoteQuery<'l> {
    fn init(db: &'l sqlite::Connection) -> Result<Self> {
        db.prepare("
            update notes
              set
                status = 2,
                mtime = strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
              where uuid = ?
        ").map_err(|e| anyhow!(e)).map(Self)
    }

    fn exec(&mut self, uuid: &str) -> Result<()> {
        self.0.reset()?;
        self.0.bind(1, uuid)?;
        while let sqlite::State::Row = self.0.next()? { }
        Ok(())
    }
}

struct InsertNoteQuery<'l>(sqlite::Statement<'l>);

impl<'l> InsertNoteQuery<'l> {
    fn init(db: &'l sqlite::Connection) -> Result<Self> {
        db.prepare("
            insert into notes
              (hash, uuid, ctime, tags, data)
            values
              (?, ?, ?, ?, ?)
            on conflict (hash) do nothing
        ").map_err(|e| anyhow!(e)).map(Self)
    }

    fn exec(
        &mut self,
        hash: &str,
        uuid: &str,
        ctime: &DateTime<Utc>,
        tags: &str,
        json: &str
    ) -> Result<()> {
        self.0.reset()?;
        self.0.bind(1, hash)?;
        self.0.bind(2, uuid)?;
        self.0.bind(3, ctime.to_rfc3339().as_str())?;
        self.0.bind(4, tags)?;
        self.0.bind(5, json)?;
        while let sqlite::State::Row = self.0.next()? { }
        Ok(())
    }
}

// insert_notes must be idempotent (loading the same file again changes nothing).
// So when loading notes from a file it is ok to stop on the first error,
// fix that error and try to load the updated file again.
// We use hash(tags, note_data) to accomplish this.
pub fn insert_notes(
    db: &sqlite::Connection,
    notes: &[DbNote]
) -> Result<()> {
    let mut retire_note = RetireNoteQuery::init(db)?;
    let mut insert_note = InsertNoteQuery::init(db)?;

    for n in notes.iter() {
        db.execute("begin transaction")?;
        let uuid = n.uuid.to_string();
        retire_note.exec(&uuid)?;
        let (hash, json) = n.hash_and_json();
        insert_note.exec(&hash, &uuid, &n.ctime, &n.tags, &json)?;
        db.execute("commit transaction")?; // FIXME: try scopeguard
    }
    // FIXME: insert new notes into queue
    // FIXME: update references in the queue
    // update queue q
    //  set hash = (select hash from notes where uuid = q.uuid and status = 'active')
    // select from notes where status = 'retired'
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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::note::*;
    use chrono::{Local, Utc};
    use uuid::Uuid;

    #[test]
    fn can_init_schema() -> Result<()> {
        let db = sqlite::open(":memory:")?;
        init_schema(&db)
    }

    #[test]
    fn can_add_note() -> Result<()> {
        let db = sqlite::open(":memory:")?;
        init_schema(&db)?;
        let note = DbNote {
            uuid: Uuid::new_v4(),
            ctime: Local::now().with_timezone(&Utc),
            tags: "hello\nworld".to_string(),
            data: NoteData::Text("hello!".to_string())
        };
        let notes = vec![note.clone(), note];
        insert_notes(&db, &notes)
        // FIXME: check note exists
    }
}
