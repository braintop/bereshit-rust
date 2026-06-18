use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::models::serialize_oid_as_hex;

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_oid_as_hex"
    )]
    pub id: Option<ObjectId>,
    pub message: String,
    pub done: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoRequest {
    pub message: String,
    pub done: bool,
}
