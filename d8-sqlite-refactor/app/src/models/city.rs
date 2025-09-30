use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct City {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCityRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCityRequest {
    pub name: String,
}
