use mongodb::bson::{doc, Bson};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Doc {
    #[serde(
        rename = "_id",
        deserialize_with = "bson::serde_helpers::deserialize_hex_string_from_object_id",
        serialize_with = "bson::serde_helpers::serialize_hex_string_as_object_id"
    )]
    pub id: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub meta: Meta,
    pub blocks: Vec<Block>,
    pub top_block_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Meta {
    pub id: String,
    pub title: String,
    pub tags: Vec<String>,
    #[serde(rename = "createDate")]
    pub create_date: i64,
}

impl Into<Bson> for Meta {
    fn into(self) -> Bson {
        Bson::Document(doc! {
            "id": self.id,
            "title": self.title,
            "tags": self.tags,
            "createDate": self.create_date,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    #[serde(rename = "type")]
    pub _type: String,
    pub id: String,
    pub flavour: String,
    pub props: Props,
    pub version: i32,
    pub parent: String,
}

impl Into<Bson> for Block {
    fn into(self) -> Bson {
        Bson::Document(doc! {
            "type": self._type,
            "id": self.id,
            "flavour": self.flavour,
            "props": self.props,
            "version": self.version,
            "parent": self.parent,
        })
    }
}

pub type Props = Bson;
