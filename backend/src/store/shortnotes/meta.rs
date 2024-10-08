use anyhow::Ok;
use sqlx::{Pool, Postgres};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Meta {
    pub snid: String,
    pub title: String,
    pub tags: sqlx::types::Json<Vec<String>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct MetaCreate {
    pub snid: String,
    pub title: String,
    pub tags: sqlx::types::Json<Vec<String>>,
}

pub async fn create_meta(
    user_id: i32,
    meta_create: MetaCreate,
    db: &Pool<Postgres>,
) -> anyhow::Result<Meta> {
    Ok(sqlx::query_as::<_, Meta>("insert into shortnotes (snid, user_id, title, content, tags) values ($1,$2,$3,$4,$5) returning snid,title,tags,created_at;")
    .bind(meta_create.snid)
    .bind(user_id)
    .bind(meta_create.title)
    .bind::<Vec<u8>>(vec![])
    .bind(meta_create.tags)
    .fetch_one(db).await?)
}

pub async fn get_metas(user_id: i32, db: &Pool<Postgres>) -> anyhow::Result<Vec<Meta>> {
    Ok(sqlx::query_as::<_, Meta>(
        "select snid,title,tags,created_at from shortnotes where user_id = $1;",
    )
    .bind(user_id)
    .fetch_all(db)
    .await?
    .iter()
    .map(|meta| meta.clone())
    .collect())
}
