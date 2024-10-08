use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, QueryBuilder};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct DiaryNote {
    pub id: i32,
    pub dnid: String,
    pub title: String,
    pub content: Vec<u8>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct DiaryNoteUpdate {
    pub title: Option<String>,
    pub content: Option<Vec<u8>>,
    pub tags: Option<sqlx::types::Json<Vec<String>>>,
}

pub async fn create_diarynote(
    dnid: String,
    user_id: i32,
    db: &Pool<Postgres>,
) -> anyhow::Result<DiaryNote> {
    Ok(sqlx::query_as::<_, DiaryNote>(
        "insert into diarynotes (dnid, user_id, title) values ($1,$2,$3) returning id,dnid,user_id,title,content,created_at,updated_at;",
    )
    .bind(dnid)
    .bind(user_id)
    .bind("New Diary")
    .fetch_one(db)
    .await?)
}

pub async fn update_diarynote_by_dnid(
    dnid: String,
    update: DiaryNoteUpdate,
    db: &Pool<Postgres>,
) -> anyhow::Result<DiaryNote> {
    let mut query = QueryBuilder::new("update diarynotes set ");
    let mut count = 0;
    if let Some(title) = update.title {
        query.push("title = ");
        query.push_bind(title);
        count += 1;
    }
    if let Some(content) = update.content {
        if count > 0 {
            query.push(", ");
        }
        query.push("content = ");
        query.push_bind(content);
        count += 1;
    }
    if let Some(tags) = update.tags {
        if count > 0 {
            query.push(", ");
        }
        query.push("tags = ");
        query.push_bind(tags);
    }
    if count == 0 {
        // nothing to update
        return Err(anyhow!(sqlx::Error::RowNotFound));
    }
    query.push(" where dnid = ");
    query.push_bind(dnid);
    query.push(" returning id,dnid,title,content,tags,user_id,created_at,updated_at;");
    Ok(query.build_query_as().fetch_one(db).await?)
}

pub async fn get_diarynote_by_date(
    user_id: i32,
    dnid: String,
    db: &Pool<Postgres>,
) -> anyhow::Result<DiaryNote> {
    Ok(sqlx::query_as::<_, DiaryNote>(
        "select id,dnid,user_id,title,content,created_at,updated_at from diarynotes where user_id = $1 and dnid = $2;",
    )
    .bind(user_id)
    .bind(dnid)
    .fetch_one(db)
    .await?)
}
