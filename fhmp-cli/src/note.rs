use chrono::{DateTime, Utc, Local};
use serde::{Serialize, Deserialize};
use sha3::{Shake128, digest::{Update, ExtendableOutput, XofReader}};
use thiserror::Error;
use uuid::Uuid;


#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum NoteData {
    Text(String),
    Card(Vec<String>)
}

#[derive(Serialize, Clone)]
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


#[derive(Deserialize)]
pub struct InputNote {
    pub uuid: Option<Uuid>,
    pub ctime: Option<DateTime<Local>>,
    pub tags: String,
    pub card: Option<Vec<String>>,
    pub text: Option<String>
}

#[derive(Error, Debug)]
pub enum NoteParseError {
    #[error("no `card` or `text` keys are found")]
    KeysNotFound,
    #[error("both `card` and `text` keys are present")]
    TooManyKeys,
    #[error("`card` must have two or more elments")]
    InvalidCard,
}

impl InputNote {
    pub fn to_db_note(&self) -> Result<DbNote, NoteParseError> {
        let uuid = self.uuid.unwrap_or_else(Uuid::new_v4);
        let ctime = self.ctime.unwrap_or_else(Local::now).with_timezone(&Utc);
        let mut tags = self.tags
            .split(",")
            .map(|s| s.trim().to_lowercase())
            .collect::<Vec<_>>();
        tags.sort();
        let tags = tags.join("\n");

        if self.card != None && self.text != None {
            Err(NoteParseError::TooManyKeys)
        } else if let Some(card) = &self.card {
            if card.len() < 2 {
                Err(NoteParseError::InvalidCard)
            } else {
                Ok(DbNote {
                    uuid, ctime, tags,
                    data: NoteData::Card(card.clone())
                })
            }
        } else if let Some(text) = &self.text {
            Ok(DbNote {
                uuid, ctime, tags,
                data: NoteData::Text(text.clone())
            })
        } else {
            Err(NoteParseError::KeysNotFound)
        }
    }
}
