use actix_web::{web, App, HttpServer};
use crate::controllers::item_controller::AppState;
use crate::routes::items::item_routes;

mod models;
mod controllers;
mod routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        items: std::sync::Mutex::new(vec![]),
    });

    println!("🚀 Server running at http://127.0.0.1:3002");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(item_routes())
    })
    .bind(("127.0.0.1", 3002))?
    .run()
    .await
}