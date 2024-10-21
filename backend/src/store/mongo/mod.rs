pub mod types;

use futures::StreamExt;
use mongodb::bson::{doc, Bson};

pub async fn create_note(doc: types::Doc, mongo: &mongodb::Client) -> anyhow::Result<Bson> {
    let db = mongo.database("notes");
    let collection = db.collection::<types::Doc>("note");
    let result = collection.insert_one(doc).await?.inserted_id;
    Ok(result)
}

pub async fn update_note(
    id: String,
    doc: types::Doc,
    mongo: &mongodb::Client,
) -> anyhow::Result<()> {
    let db = mongo.database("notes");
    let collection = db.collection::<types::Doc>("note");
    collection
        .update_one(
            doc! {"meta.id": doc!{ "$eq": id }},
            doc! {
                "$set": doc! {
                    "meta": doc.meta,
                    "blocks": doc.blocks,
                }
            },
        )
        .await?;
    Ok(())
}

pub async fn get_note_by_id(id: String, mongo: &mongodb::Client) -> anyhow::Result<types::Doc> {
    let db = mongo.database("notes");
    let collection = db.collection::<types::Doc>("note");
    let result = collection.find_one(doc! {"meta.id": { "$eq": id }}).await?;
    match result {
        Some(doc) => Ok(doc),
        None => Err(anyhow::anyhow!("Not found")),
    }
}

pub async fn get_notes_meta(mongo: &mongodb::Client) -> anyhow::Result<Vec<types::Meta>> {
    let db = mongo.database("notes");
    let collection = db.collection::<types::Doc>("note");
    let mut cursor = collection.find(doc! {}).await?;
    let mut result = vec![];
    while let Some(doc) = cursor.next().await {
        result.push(doc?.meta);
    }
    Ok(result)
}
