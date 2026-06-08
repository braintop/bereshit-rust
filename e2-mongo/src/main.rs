use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use mongodb::{
    bson::{doc, Document},
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, Collection,
};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    message: String,
    done: bool,
}

#[derive(Debug, Deserialize)]
struct CreateTodoRequest {
    message: String,
}

struct AppState {
    todos_collection: Collection<Todo>,
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Todo server is running")
}

#[get("/shmuel")]
async fn shmuel() -> impl Responder {
    HttpResponse::Ok().body("Smuael  is good man")
}


#[post("/todos")]
async fn create_todo(
    data: web::Data<AppState>,
    body: web::Json<CreateTodoRequest>,
) -> impl Responder {
    let todo = Todo {
        message: body.message.clone(),
        done: false,
    };

    let result = data.todos_collection.insert_one(&todo).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/todos")]
async fn get_all_todos(data: web::Data<AppState>) -> impl Responder {
    let mut cursor = match data.todos_collection.find(doc! {}).await {
        Ok(cursor) => cursor,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let mut todos: Vec<Todo> = Vec::new();

    while cursor.advance().await.unwrap_or(false) {
        match cursor.deserialize_current() {
            Ok(todo) => todos.push(todo),
            Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
        }
    }

    HttpResponse::Ok().json(todos)
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    dotenv().ok();

    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");

    let mut client_options = ClientOptions::parse(mongo_uri).await?;

    let server_api = ServerApi::builder()
        .version(ServerApiVersion::V1)
        .build();

    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options)?;

    client
        .database("cluster0")
        .run_command(doc! { "ping": 1 })
        .await?;

    println!("Connected to MongoDB");

    let database = client.database("todo_db");
    let todos_collection = database.collection::<Todo>("todos");

    let app_state = web::Data::new(AppState {
        todos_collection,
    });

    println!("Server running on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(home)
            .service(shmuel)
            .service(create_todo)
            .service(get_all_todos)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}