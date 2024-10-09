use crate::store;

// create note request payload
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct NoteCreate {
    pub note_type: String,
}

// update note request payload
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct NoteUpdate {
    pub title: Option<String>,
    pub content: Option<Vec<u8>>,
    pub tags: Option<Vec<String>>,
}

// note response payload
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Note {
    pub note_id: String,
    pub version: i32,
    pub title: String,
    pub content: Vec<u8>,
    pub tags: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// meta of note
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct NoteMeta {
    pub note_id: String,
    pub version: i32,
    pub is_latest: bool,
    pub note_type: String,
    pub title: String,
    pub tags: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// note response payload
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct NoteResponse {
    pub note: Note,
}

impl Note {
    pub fn from_store(note: store::notes::types::Note) -> Self {
        Self {
            note_id: note.note_id,
            version: note.note_version,
            title: note.title,
            content: note.content,
            tags: note.tags.to_vec(),
            created_at: note.created_at,
        }
    }
}

impl NoteMeta {
    pub fn from_store(note: store::notes::types::Note) -> Self {
        Self {
            note_id: note.note_id,
            version: note.note_version,
            is_latest: note.is_latest,
            title: note.title,
            tags: note.tags.to_vec(),
            created_at: note.created_at,
            note_type: note.note_type,
        }
    }
}
