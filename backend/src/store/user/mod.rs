use sqlx::{Pool, Postgres};

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow, Clone)]
pub struct UserCreate {
    pub name: String,
    pub password: String,
    pub email: String,
}

#[allow(unused)]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserUpdate {
    pub name: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
}

#[allow(unused)]
pub async fn create_user(
    user_create: UserCreate,
    db: &Pool<Postgres>,
) -> anyhow::Result<User> {
    Ok(sqlx::query_as::<_, User>("insert into users (name, password, email) values ($1,$2,$3) returning id,name,email,password,created_at,updated_at;")
        .bind(user_create.name)
        .bind(user_create.password)
        .bind(user_create.email)
        .fetch_one(db).await?)
}

#[allow(unused)]
pub async fn get_user_by_emial(
    email: String,
    db: &Pool<Postgres>,
) -> anyhow::Result<User> {
    Ok(sqlx::query_as::<_, User>("select id,name,email,password,created_at,updated_at from users where email = $1;")
        .bind(email)
        .fetch_one(db).await?)
}

#[allow(unused)]
pub async fn get_user_by_id(
    id: i32,
    db: &Pool<Postgres>,
) -> anyhow::Result<User> {
    Ok(sqlx::query_as::<_, User>("select id,name,email,password,created_at,updated_at from users where id = $1;")
        .bind(id)
        .fetch_one(db).await?)
}

#[allow(unused)]
pub async fn get_password_by_email(
    email: String,
    db: &Pool<Postgres>,
) -> anyhow::Result<String> {
    Ok(sqlx::query_as::<_, (String,)>("select password from users where email = $1;")
        .bind(email)
        .fetch_one(db).await?.0)
}

#[allow(unused)]
pub async fn get_password_by_id(
    id: i32,
    db: &Pool<Postgres>,
) -> anyhow::Result<String> {
    Ok(sqlx::query_as::<_, (String,)>("select password from users where id = $1;")
        .bind(id)
        .fetch_one(db).await?.0)
}