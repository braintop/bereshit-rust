use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub title: String,
    pub price: f64,
    pub stock: i32,
}

impl Product {
    pub fn new(name: String, title: String, price: f64, stock: i32) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            title,
            price,
            stock,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ProductUpdateDTO {
    pub name: String,
    pub title: String,
    pub price: f64,
    pub stock: i32,
}
