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

        create trigger if not exists retire_updated_notes
            before insert on notes
            begin
                update notes
                    set status = 2,
                        mtime = strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
                    where true
                      and hash <> new.hash
                      and uuid = new.uuid
                      and status = 1;
            end;

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

    fn exec(&mut self, n: &DbNote) -> Result<()> {
        self.0.reset()?;
        let (hash, json) = n.hash_and_json();
        self.0.bind(1, hash.as_str())?;
        self.0.bind(2, n.uuid.to_string().as_str())?;
        self.0.bind(3, n.ctime.to_rfc3339().as_str())?;
        self.0.bind(4, n.tags.as_str())?;
        self.0.bind(5, json.as_str())?;
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
    let mut insert_note = InsertNoteQuery::init(db)?;

    for n in notes.iter() {
        insert_note.exec(&n)?;
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
        res.push(db_note_from_row(&q)?);
    }

    Ok(res)
}

#[cfg(test)]
// Returns active notes
fn dump_notes(
    db: &sqlite::Connection
) -> Result<DbNotes> {
    let q = db.prepare(
        "select uuid, ctime, tags, data from notes where status = 1"
    )?;
    Ok(DbNotes(q))
}

pub struct DbNotes<'a>(sqlite::Statement<'a>);

impl<'a> Iterator for DbNotes<'a> {
    type Item = DbNote;
    fn next(&mut self) -> Option<DbNote> {
        match self.0.next() {
            Ok(sqlite::State::Row) => db_note_from_row(&self.0).ok(),
            _ => None
        }
    }
}

// Assumes that q starts like "select uuid, ctime, tags, data ..".
fn db_note_from_row(q: &sqlite::Statement) -> Result<DbNote> {
    Ok(DbNote {
        uuid:
            Uuid::parse_str(q.read::<String>(0)?.as_str())?,
        ctime:
            DateTime::parse_from_rfc3339(q.read::<String>(1)?.as_str())?
                .with_timezone(&Utc),
        tags:
            q.read::<String>(2)?,
        data:
            serde_json::from_str(q.read::<String>(3)?.as_str())?,
    })
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
    fn can_add_single_note() -> Result<()> {
        let db = sqlite::open(":memory:")?;
        init_schema(&db)?;
        let note = text_note("hello\nworld", "hello!");
        insert_notes(&db, &vec![note.clone()])?;

        let mut iter = dump_notes(&db)?;
        assert_eq!(Some(note), iter.next());
        assert_eq!(None, iter.next());
        Ok(())
    }

    #[test]
    fn insert_skips_duplicates() -> Result<()> {
        let db = sqlite::open(":memory:")?;
        init_schema(&db)?;
        let note1 = text_note("hello\nworld", "hello!");
        let note2 = text_note("bye\nworld", "bye!");
        let notes = vec![note1.clone(), note2.clone(), note1.clone()];
        insert_notes(&db, &notes)?;
        let notes = vec![note2.clone(), note2.clone()];
        insert_notes(&db, &notes)?;

        let mut n1 = 0;
        let mut n2 = 0;
        for n in dump_notes(&db)? {
            if n == note1 { n1 += 1 }
            else if n == note2 { n2 += 1 }
        }
        assert_eq!(1, n1);
        assert_eq!(1, n2);
        Ok(())
    }

    #[test]
    fn can_update_note() -> Result<()> {
        let db = sqlite::open(":memory:")?;
        init_schema(&db)?;
        let note1 = text_note("hello\nworld", "hello!");
        let note2 = DbNote {
            uuid: note1.uuid,
            ..text_note("hello\nworld", "bye!")
        };
        let notes = vec![note1, note2.clone()];
        insert_notes(&db, &notes)?;

        let mut iter = dump_notes(&db)?;
        assert_eq!(Some(note2), iter.next());
        assert_eq!(None, iter.next());
        Ok(())
    }

    // helper function for tests
    fn text_note(tags: &str, text: &str) -> DbNote {
        DbNote {
            uuid: Uuid::new_v4(),
            ctime: Local::now().with_timezone(&Utc),
            tags: tags.to_string(),
            data: NoteData::Text(text.to_string())
        }
    }
}
