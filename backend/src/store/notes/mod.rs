pub mod types;

// create note by NoteCreatePayload
pub async fn create_note(
    note: types::NoteCreatePayload,
    pool: &sqlx::PgPool,
) -> anyhow::Result<types::NoteResponse> {
    let note = sqlx::query_as::<_, types::Note>(
        "
        INSERT INTO notes (user_id, note_id, note_type, title, tags, content, note_version, is_latest)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *;
        ",
    )
    .bind(note.user_id)
    .bind(note.note_id)
    .bind(note.note_type)
    .bind(note.title)
    .bind(note.tags)
    .bind(note.content)
    .bind(1)
    .bind(true)
    .fetch_one(pool)
    .await?;

    Ok(types::NoteResponse { note })
}

// get note by note_id
pub async fn get_note(note_id: String, pool: &sqlx::PgPool) -> anyhow::Result<types::NoteResponse> {
    let note = sqlx::query_as::<_, types::Note>(
        "
        SELECT * FROM notes
        WHERE note_id = $1 AND is_latest = true;
        ",
    )
    .bind(note_id)
    .fetch_one(pool)
    .await?;

    Ok(types::NoteResponse { note })
}

// update note by note_id and NoteUpdatePayload
pub async fn update_note(
    note_id: String,
    update: types::NoteUpdatePayload,
    pool: &sqlx::PgPool,
) -> anyhow::Result<types::NoteResponse> {
    // update is_latest field of the current note by note_id and is_latest is true, and return the note
    let mut note = sqlx::query_as::<_, types::Note>(
        "
        UPDATE notes
        SET is_latest = false
        WHERE note_id = $1 AND is_latest = true 
        RETURNING *;
        ",
    )
    .bind(note_id)
    .fetch_one(pool)
    .await?;

    // update the note of return by NoteUpdatePayload
    if let Some(title) = update.title {
        note.title = title;
    }
    if let Some(tags) = update.tags {
        note.tags = tags;
    }
    if let Some(content) = update.content {
        note.content = content;
    }

    // create a new note with the updated note
    let note = sqlx::query_as::<_, types::Note>(
        "
        INSERT INTO notes (user_id, note_id, note_type, title, tags, content, note_version, is_latest)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *;
        ",
    )
    .bind(note.user_id)
    .bind(note.note_id)
    .bind(note.note_type)
    .bind(note.title)
    .bind(note.tags)
    .bind(note.content)
    .bind(note.note_version + 1)
    .bind(true)
    .fetch_one(pool)
    .await?;

    Ok(types::NoteResponse { note })
}

// get all notes by user_id
pub async fn get_notes_by_user_id(
    user_id: i32,
    pool: &sqlx::PgPool,
) -> anyhow::Result<Vec<types::Note>> {
    let notes = sqlx::query_as::<_, types::Note>(
        "
        SELECT * FROM notes
        WHERE user_id = $1 AND is_latest = true;
        ",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(notes)
}
