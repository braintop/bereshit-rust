use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: String,
    pub title: String,
    pub description: String,
}

impl Item {
    pub fn new(title: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
        }
    }
}
