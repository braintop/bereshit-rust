use actix_web::{web, App, HttpServer};
use crate::controllers::city_controller;
use crate::routes::cities::city_routes;
use crate::routes::workers::worker_routes;

mod models;
mod controllers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize database
    let pool = city_controller::init_db().await.expect("Failed to initialize database");
    
    println!("🚀 Server running at http://127.0.0.1:3004");
    println!("📊 SQLite database initialized at src/mydb.db");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(city_routes())
            .service(worker_routes())
    })
    .bind(("127.0.0.1", 3007))?
    .run()
    .await
}