use mongodb::bson::{doc, Binary};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OssFile {
    #[serde(
        rename = "_id",
        deserialize_with = "bson::serde_helpers::deserialize_hex_string_from_object_id",
        serialize_with = "bson::serde_helpers::serialize_hex_string_as_object_id"
    )]
    pub id: String,
    pub key: String,
    pub size: i64,
    #[serde(rename = "type")]
    pub _type: String,
    pub data: Binary,
}

pub async fn create_file(file: OssFile, mongo: &mongodb::Client) -> anyhow::Result<()> {
    let db = mongo.database("oss");
    let collection = db.collection::<OssFile>("file");
    collection.insert_one(file).await?;
    Ok(())
}

pub async fn get_file_by_key(key: String, mongo: &mongodb::Client) -> anyhow::Result<OssFile> {
    let db = mongo.database("oss");
    let collection = db.collection::<OssFile>("file");
    let result = collection.find_one(doc! {"key": { "$eq": key }}).await?;
    match result {
        Some(file) => Ok(file),
        None => Err(anyhow::anyhow!("Not found")),
    }
}
