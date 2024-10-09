// define the types for the notes store
#[derive(serde::Deserialize, serde::Serialize, Debug, sqlx::FromRow, Clone)]
pub struct Note {
    pub id: i32,
    pub note_id: String,
    pub user_id: i32,

    pub note_type: String,
    pub note_version: i32,
    pub is_latest: bool,

    pub title: String,
    pub tags: sqlx::types::Json<Vec<String>>,
    pub content: Vec<u8>,

    pub created_at: chrono::DateTime<chrono::Utc>,
}

// notes create payload
#[derive(serde::Deserialize, serde::Serialize, Debug, sqlx::FromRow, Clone)]
pub struct NoteCreatePayload {
    pub user_id: i32,
    pub note_id: String,

    pub note_type: String,

    pub title: String,
    pub tags: sqlx::types::Json<Vec<String>>,
    pub content: Vec<u8>,
}

// notes update payload
#[derive(serde::Deserialize, serde::Serialize, Debug, sqlx::FromRow, Clone)]
pub struct NoteUpdatePayload {
    pub title: Option<String>,
    pub tags: Option<sqlx::types::Json<Vec<String>>>,
    pub content: Option<Vec<u8>>,
}

// notes response
#[derive(serde::Deserialize, serde::Serialize, Debug, sqlx::FromRow, Clone)]
pub struct NoteResponse {
    pub note: Note,
}
