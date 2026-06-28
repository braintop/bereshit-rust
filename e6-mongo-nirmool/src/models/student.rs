use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::models::serialize_oid_as_hex;

#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_oid_as_hex"
    )]
    pub id: Option<ObjectId>,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateStudentRequest {
    pub name: String,
}

/// Safe view of a student for HTTP responses.
#[derive(Debug, Serialize, Clone)]
pub struct StudentResponse {
    pub id: Option<String>,
    pub name: String,
}

impl From<&Student> for StudentResponse {
    fn from(student: &Student) -> Self {
        StudentResponse {
            id: student.id.map(|oid| oid.to_hex()),
            name: student.name.clone(),
        }
    }
}
