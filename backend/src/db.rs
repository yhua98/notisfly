use sqlx::postgres::PgPoolOptions;

pub async fn connect(db_url: String) -> sqlx::Result<sqlx::PgPool> {
    PgPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .connect(&db_url)
        .await
}

pub async fn connect_mongo(db_uri: String) -> mongodb::Client {
    let client = mongodb::Client::with_uri_str(db_uri).await.unwrap();
    client
}
