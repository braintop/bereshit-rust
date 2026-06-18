use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::models::serialize_oid_as_hex;

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_oid_as_hex"
    )]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateRecipeRequest {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRecipeRequest {
    pub title: String,
    pub description: String,
}
