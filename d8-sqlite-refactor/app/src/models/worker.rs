use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Worker {
    pub worker_id: i64,
    pub first_name: String,
    pub last_name: String,
    pub salary: f64,
    pub city_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateWorkerRequest {
    pub first_name: String,
    pub last_name: String,
    pub salary: f64,
    pub city_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWorkerRequest {
    pub first_name: String,
    pub last_name: String,
    pub salary: f64,
    pub city_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkerWithCity {
    pub worker_id: i64,
    pub first_name: String,
    pub last_name: String,
    pub salary: f64,
    pub city_id: i64,
    pub city_name: String,
}
