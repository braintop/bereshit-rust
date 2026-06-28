mod controllers;
mod db;
mod models;
mod routes;
mod state;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;

use crate::models::recipe::Recipe;
use crate::models::student::Student;
use crate::models::todo::Todo;
use crate::models::user::User;
use crate::state::AppState;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    dotenv().ok();

    let client = db::connect().await?;

    let database = client.database("todo_db");
    let todos_collection = database.collection::<Todo>("todos");
    let recipes_collection = database.collection::<Recipe>("recipes");
    let users_collection = database.collection::<User>("users");
    let students_collection = database.collection::<Student>("students");

    let app_state = web::Data::new(AppState {
        todos_collection,
        recipes_collection,
        users_collection,
        students_collection,
    });

    println!("Server running on http://127.0.0.1:8080");

    HttpServer::new(move || {
        // Dev-only: allow any origin so the browser frontend can call the API.
        // Tighten this (e.g. .allowed_origin("http://localhost:5173")) for production.
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .configure(routes::general_routes::configure)
            .configure(routes::todo_routes::configure)
            .configure(routes::recipe_routes::configure)
            .configure(routes::user_routes::configure)
            .configure(routes::student_routes::configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
