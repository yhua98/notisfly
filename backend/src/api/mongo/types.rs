use std::{cell::RefCell, collections::HashMap, rc::Rc};

use bson::Bson;
use serde::{Deserialize, Serialize};

use crate::store;

#[derive(Debug, Serialize, Deserialize)]
pub struct Doc {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub _type: String,
    pub meta: Meta,
    pub blocks: Block,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoDoc {
    #[serde(
        rename = "_id",
        deserialize_with = "bson::serde_helpers::deserialize_hex_string_from_object_id",
        serialize_with = "bson::serde_helpers::serialize_hex_string_as_object_id"
    )]
    pub id: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub meta: Meta,
    pub blocks: TodoBlock,
    pub top_block_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoBlock {
    #[serde(rename = "type")]
    pub _type: String,
    pub id: String,
    pub flavour: String,
    pub props: Props,
    pub version: i32,
    pub parent: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocCreate {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub _type: String,
    pub meta: Meta,
    pub blocks: BlockCreate,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Meta {
    pub id: String,
    pub title: String,
    pub tags: Vec<String>,
    #[serde(rename = "createDate")]
    pub create_date: i64,
}

impl Meta {
    pub fn to_store_meta(&self) -> store::mongo::types::Meta {
        store::mongo::types::Meta {
            id: self.id.clone(),
            title: self.title.clone(),
            tags: self.tags.clone(),
            create_date: self.create_date,
        }
    }

    pub fn from_store_meta(meta: store::mongo::types::Meta) -> Meta {
        Meta {
            id: meta.id,
            title: meta.title,
            tags: meta.tags,
            create_date: meta.create_date,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockCreate {
    #[serde(rename = "type")]
    pub _type: String,
    pub id: String,
    pub flavour: String,
    pub props: Props,
    pub version: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<BlockCreate>>,
}

impl BlockCreate {
    pub fn to_store_blocks(self, parent_id: String) -> Vec<store::mongo::types::Block> {
        let mut blocks = vec![];

        blocks.push(store::mongo::types::Block {
            _type: self._type.clone(),
            id: self.id.clone(),
            flavour: self.flavour.clone(),
            props: self.props.clone(),
            version: self.version,
            parent: parent_id.clone(),
        });

        if let Some(children) = self.children {
            for child in children {
                blocks.append(&mut child.to_store_blocks(self.id.clone()));
            }
        }

        blocks
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
    pub children: Vec<Rc<RefCell<Block>>>,
}

impl Block {
    pub fn from_store_blocks(top_id: String, blocks: Vec<store::mongo::types::Block>) -> Block {
        let mut map: HashMap<String, Rc<RefCell<Block>>> = HashMap::new();

        for block in blocks {
            if let Some(_block) = map.get_mut(&block.id) {
                // If the block is already in the map, update the block
                let mut _block = _block.borrow_mut();
                _block.id = block.id.clone();
                _block._type = block._type.clone();
                _block.flavour = block.flavour.clone();
                _block.props = block.props.clone();
                _block.version = block.version;
            } else {
                // If the block is not in the map, create a new block
                let _block = Rc::new(RefCell::new(Block {
                    _type: block._type.clone(),
                    id: block.id.clone(),
                    flavour: block.flavour.clone(),
                    props: block.props.clone(),
                    version: block.version,
                    children: vec![],
                }));
                map.insert(block.id.clone(), _block);
            }

            let _block = map.get(&block.id).unwrap().clone();

            if let Some(parent) = map.get(&block.parent) {
                // If the parent block is already in the map, add the child block
                let mut parent = parent.borrow_mut();
                // if parent.children.is_none() {
                //     parent.children = Some(vec![]);
                // }
                parent.children.push(_block.clone());
            } else {
                // If the parent block is not in the map, create a new block
                let parent = Rc::new(RefCell::new(Block {
                    _type: "".to_string(),
                    id: block.parent.clone(),
                    flavour: "".to_string(),
                    props: Bson::Null,
                    version: 0,
                    children: vec![_block.clone()],
                }));
                map.insert(block.parent.clone(), parent);
            }
        }

        let top_block = map.get(&top_id).unwrap().clone();
        top_block.to_owned().borrow().clone()
    }
}

pub type Props = Bson;
