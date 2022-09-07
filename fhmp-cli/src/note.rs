use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Local};
use uuid::Uuid;
use sha3::{Shake128, digest::{Update, ExtendableOutput, XofReader}};


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
    // FIXME: tags must be normalized (sorted & LF delimited)
    pub tags: String,
    pub data: NoteData,
}

impl DbNote {
    // FIXME: When we add note to DB, self.data serialized to JSON twice:
    // to calculate hash and to insert JSON into DB.
    pub fn hash(&self) -> String {
        let mut hasher = Shake128::default();
        hasher.update(self.tags.as_bytes());
        hasher.update(self.data_as_json().as_bytes());
        let mut reader = hasher.finalize_xof();
        let mut hash = [0u8; 16];
        reader.read(&mut hash);
        hex::encode(hash)
    }

    pub fn data_as_json(&self) -> String {
        serde_json::to_string(&self.data)
            .expect("Serializing struct to JSON should always succeed.")
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

impl InputNote {
    pub fn to_db_note(&self) -> Result<DbNote, String> {
        let uuid = self.uuid.unwrap_or_else(Uuid::new_v4);
        let ctime = self.ctime.unwrap_or_else(Local::now).with_timezone(&Utc);
        let tags = self.tags.clone(); // FIXME: sort

        if self.card != None && self.text != None {
            Err("both `card` and `text` are present".to_string())
        } else if let Some(card) = &self.card {
           if card.len() < 2 {
               Err("`card` must have two or more elments".to_string())
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
            Err("`card` or `text` are not found".to_string())
        }
    }
}
