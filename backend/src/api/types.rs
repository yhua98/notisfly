use serde::{Serialize,Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct ResponsePayload<T> {
    pub status: u16,
    pub message: String,
    pub data: Option<T>
}