use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::models::serialize_oid_as_hex;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_oid_as_hex"
    )]
    pub id: Option<ObjectId>,
    pub email: String,
    // Stores the bcrypt hash. This IS serialized so it gets written to mongo.
    // It must never be sent to the client — return `UserResponse` from handlers
    // instead of this struct.
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}

/// Safe view of a user for HTTP responses — no password hash.
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Option<String>,
    pub email: String,
}

impl From<&User> for UserResponse {
    fn from(user: &User) -> Self {
        UserResponse {
            id: user.id.map(|oid| oid.to_hex()),
            email: user.email.clone(),
        }
    }
}
