use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::models::serialize_oid_as_hex;
use crate::models::student::{Student, StudentResponse};

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
    /// Reference to the owning student (many todos → one student).
    pub student_id: ObjectId,
}

/// Raw aggregation result after `$lookup` + `$unwind` (MongoDB "populate").
#[derive(Debug, Deserialize)]
pub struct TodoPopulated {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub message: String,
    pub done: bool,
    pub student_id: ObjectId,
    pub student: Student,
}

/// JSON response with the related student embedded — like Mongoose `.populate('student')`.
#[derive(Debug, Serialize)]
pub struct TodoPopulatedResponse {
    pub id: Option<String>,
    pub message: String,
    pub done: bool,
    pub student_id: String,
    pub student: StudentResponse,
}

impl From<TodoPopulated> for TodoPopulatedResponse {
    fn from(todo: TodoPopulated) -> Self {
        TodoPopulatedResponse {
            id: todo.id.map(|oid| oid.to_hex()),
            message: todo.message,
            done: todo.done,
            student_id: todo.student_id.to_hex(),
            student: StudentResponse::from(&todo.student),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub message: String,
    pub student_id: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoRequest {
    pub message: String,
    pub done: bool,
}
