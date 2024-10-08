use sqlx::postgres::PgPoolOptions;

pub async fn connect(db_url:String) -> sqlx::Result<sqlx::PgPool> {
    PgPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .connect(&db_url)
        .await
}