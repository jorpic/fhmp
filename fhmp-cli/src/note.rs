// This file contains data structures to store notes and helper functions to manipulate them.
// We have different representations for notes in DB and notes not yet in DB.
use chrono::{DateTime, Utc, Local};
use serde::{Serialize, Deserialize};
use sha3::{Shake128, digest::{Update, ExtendableOutput, XofReader}};
use thiserror::Error;
use uuid::Uuid;


#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(PartialEq, Debug))]
#[serde(rename_all = "camelCase")]
pub enum NoteData {
    Text(String),
    Card(Vec<String>)
}

// DbNote represents a note as it is stored in the DB.
// It is used both for inserting new notes into the DB and selecting
// exisintg ones from the DB.
#[derive(Serialize, Clone)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct DbNote {
    pub uuid: Uuid,
    // ctime is stored as a RFC3339 formatted string with UTC timezone
    // in the hope that it will be easier to use and manipulate.
    pub ctime: DateTime<Utc>,
    pub tags: String,
    pub data: NoteData,
}

impl DbNote {
    pub fn data_as_json(&self) -> String {
        serde_json::to_string(&self.data)
            .expect("Serializing struct to JSON should always succeed.")
    }

    // This is used to store note into DB.
    pub fn hash_and_json(&self) -> (String, String) {
        let json = self.data_as_json();

        let mut hasher = Shake128::default();
        hasher.update(self.tags.as_bytes());
        hasher.update(json.as_bytes());
        let mut reader = hasher.finalize_xof();
        let mut hash = [0u8; 16];
        reader.read(&mut hash);
        (hex::encode(hash), json)
    }
}

// InputNote is used to deserialize draft notes from a YAML file.
// It has multiple optional fields to handle both notes dumped from the DB
// and new notes just composed by hand.
// Note format is supposed to be human-friendly rather than machine-friendly.
#[derive(Deserialize)]
pub struct InputNote {
    pub uuid: Option<Uuid>,
    pub ctime: Option<DateTime<Local>>,
    pub tags: String,
    pub data: NoteData
}

#[derive(Error, PartialEq, Debug)]
pub enum NoteParseError {
    #[error("`card` must have two or more elments")]
    InvalidCard,
}

impl InputNote {
    // Successful parse of YAML into InputNote does not guarantee that the note
    // is really wellformed.
    // Here we verify InputNote more throughly while converting it into DbNote.
    pub fn to_db_note(&self) -> Result<DbNote, NoteParseError> {
        let uuid = self.uuid.unwrap_or_else(Uuid::new_v4);
        let ctime = self.ctime.unwrap_or_else(Local::now).with_timezone(&Utc);
        let mut tags = self.tags
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .collect::<Vec<_>>();
        tags.sort();
        let tags = tags.join("\n"); // list of tags is \n delimited

        match &self.data {
            NoteData::Card(items) if items.len() < 2 =>
                Err(NoteParseError::InvalidCard),
            _ =>
                Ok(DbNote { uuid, ctime, tags, data: self.data.clone() })
        }
    }
}


#[cfg(test)]
mod tests {
    use anyhow::Result;
    use chrono::Duration;
    use indoc::indoc;
    use super::*;

    #[test]
    fn db_note_hash_depends_on_tags() {
        let note0 = DbNote {
            uuid: Uuid::new_v4(),
            ctime: Local::now().with_timezone(&Utc),
            tags: "hello\nworld".to_string(),
            data: NoteData::Text("first note".to_string())
        };
        let (h0, _) = note0.hash_and_json();

        let note1 = DbNote {
            tags: "hello".to_string(),
            ..note0.clone() };
        assert_ne!(note1.hash_and_json().0, h0);

        let note2 = DbNote {
            data: NoteData::Text("First note".to_string()),
            ..note0.clone() };
        assert_ne!(note2.hash_and_json().0, h0);

        let note3 = DbNote {
            uuid: Uuid::new_v4(),
            ..note0.clone() };
        assert_eq!(note3.hash_and_json().0, h0);

        let note4 = DbNote {
            ctime: note0.ctime + Duration::days(1),
            ..note0.clone() };
        assert_eq!(note4.hash_and_json().0, h0);
    }

    #[test]
    fn input_note_to_db_note_ok() -> Result<()> {
        let input_note: InputNote = serde_yaml::from_str(indoc!("
            tags: hello, world
            data: !text |
              hello, world!
        "))?;
        let db_note = input_note.to_db_note()?;
        assert_eq!(db_note.tags, "hello\nworld");
        assert_eq!(db_note.data, NoteData::Text("hello, world!\n".to_string()));

        let input_note: InputNote = serde_yaml::from_str(indoc!("
            tags: hello, world
            data: !card
              - hello
              - world
        "))?;
        let db_note = input_note.to_db_note()?;
        assert_eq!(db_note.tags, "hello\nworld");
        assert_eq!(db_note.data,
            NoteData::Card(vec!["hello".to_string(), "world".to_string()]));
        Ok(())
    }

    #[test]
    fn input_note_to_db_note_errs() -> Result<()> {
        let input_note: InputNote = serde_yaml::from_str(indoc!("
            tags: hello, world
            data: !card
              - hello
        "))?;
        assert_eq!(
            input_note.to_db_note(),
            Err(NoteParseError::InvalidCard)
        );
        Ok(())
    }

    // test to_db_note_preserves_ctime_and_uuid
}
