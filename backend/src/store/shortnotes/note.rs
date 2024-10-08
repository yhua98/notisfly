use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, QueryBuilder};

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortNoteCreate {
    pub snid: String,
    pub title: String,
    pub content: Vec<u8>,
    pub tags: sqlx::types::Json<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortNoteUpdate {
    pub title: Option<String>,
    pub content: Option<Vec<u8>>,
    pub tags: Option<sqlx::types::Json<Vec<String>>>,
}

#[derive(Debug,Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct ShortNote {
    pub snid: String,
    pub user_id: i32,
    pub title: String,
    pub content: Vec<u8>,
    pub tags: sqlx::types::Json<Vec<String>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub async fn create_shortnote(
    user_id: String,
    create: ShortNoteCreate,
    db: &Pool<Postgres>,
) -> anyhow::Result<ShortNote> {
    // save it to the database
    Ok(sqlx::query_as::<_, ShortNote>("insert into shortnotes (snid, user_id, title, content, tags) values ($1,$2,$3,$4,$5) returning snid,user_id,title,content,tags;")
    .bind(create.snid)
    .bind(user_id)
    .bind(create.title)
    .bind(create.content)
    .bind(create.tags)
    .fetch_one(db).await?)
}

pub async fn get_shortnote(
    snid: String,
    db: &Pool<Postgres>,
) -> anyhow::Result<ShortNote> {
    Ok(sqlx::query_as::<_, ShortNote>("select snid,title,content,user_id,tags,created_at,updated_at from shortnotes where snid = $1;")
    .bind(snid)
    .fetch_one(db).await?)
}

pub async fn get_all_shortnotes(
    user_id: String,
    db: &Pool<Postgres>,
) -> anyhow::Result<Vec<ShortNote>> {
    Ok(sqlx::query_as::<_, ShortNote>("select snid,title,content,user_id,tags,created_at,updated_at from shortnotes where user_id = $1;")
    .bind(user_id)
    .fetch_all(db).await?.iter().map(|note| note.clone()).collect())
}

pub async fn update_shortnote(
    snid: String,
    update: ShortNoteUpdate,
    db: &Pool<Postgres>,
) -> anyhow::Result<ShortNote> {
    let mut query = QueryBuilder::new("update shortnotes set ");
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
    query.push(" where snid = ");
    query.push_bind(snid);
    query.push(" returning snid,title,content,tags,user_id,created_at,updated_at;");
    Ok(query.build_query_as().fetch_one(db).await?)
}
