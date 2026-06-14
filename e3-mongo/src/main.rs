use actix_cors::Cors;
use actix_web::{get, delete, post, put, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;

use mongodb::{
    bson::{doc, oid::ObjectId},
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, Collection,
};
use serde::{Deserialize, Serialize};
use std::env;

// Serialize ObjectId to the HTTP/JSON response as a plain hex string
// (e.g. "507f1f77bcf86cd799439011") instead of {"$oid": "..."},
// so the frontend can pass it straight back to DELETE /todos/{id}.
fn serialize_oid_as_hex<S>(id: &Option<ObjectId>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match id {
        Some(oid) => s.serialize_str(&oid.to_hex()),
        None => s.serialize_none(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_oid_as_hex"
    )]
    id: Option<ObjectId>,
    message: String,
    done: bool,
}

#[derive(Debug, Deserialize)]
struct CreateTodoRequest {
    message: String,
}

#[derive(Debug, Deserialize)]
struct UpdateTodoRequest {
    message: String,
    done: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Recipe {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_oid_as_hex"
    )]
    id: Option<ObjectId>,
    title: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct CreateRecipeRequest {
    title: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct UpdateRecipeRequest {
    title: String,
    description: String,
}

struct AppState {
    todos_collection: Collection<Todo>,
    recipes_collection: Collection<Recipe>,
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Todo server is running")
}

#[get("/shmuel")]
async fn shmuel() -> impl Responder {
    HttpResponse::Ok().body("Smuael  is good man")
}

#[delete("/todos/{id}")]
async fn delete_todo(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();

    let oid = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid id"),
    };

    match data.todos_collection.delete_one(doc! { "_id": oid }).await {
        Ok(result) if result.deleted_count == 1 => HttpResponse::Ok().body("Deleted"),
        Ok(_) => HttpResponse::NotFound().body("Todo not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/todos")]
async fn create_todo(
    data: web::Data<AppState>,
    body: web::Json<CreateTodoRequest>,
) -> impl Responder {
    let mut todo = Todo {
        id: None,
        message: body.message.clone(),
        done: false,
    };

    match data.todos_collection.insert_one(&todo).await {
        Ok(result) => {
            todo.id = result.inserted_id.as_object_id();
            HttpResponse::Ok().json(todo)
        }
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

// ----- Recipes CRUD -----

#[get("/recipes")]
async fn get_all_recipes(data: web::Data<AppState>) -> impl Responder {
    let mut cursor = match data.recipes_collection.find(doc! {}).await {
        Ok(cursor) => cursor,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let mut recipes: Vec<Recipe> = Vec::new();

    while cursor.advance().await.unwrap_or(false) {
        match cursor.deserialize_current() {
            Ok(recipe) => recipes.push(recipe),
            Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
        }
    }

    HttpResponse::Ok().json(recipes)
}

#[get("/recipes/{id}")]
async fn get_recipe(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();

    let oid = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid id"),
    };

    match data.recipes_collection.find_one(doc! { "_id": oid }).await {
        Ok(Some(recipe)) => HttpResponse::Ok().json(recipe),
        Ok(None) => HttpResponse::NotFound().body("Recipe not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/recipes")]
async fn create_recipe(
    data: web::Data<AppState>,
    body: web::Json<CreateRecipeRequest>,
) -> impl Responder {
    let mut recipe = Recipe {
        id: None,
        title: body.title.clone(),
        description: body.description.clone(),
    };

    match data.recipes_collection.insert_one(&recipe).await {
        Ok(result) => {
            recipe.id = result.inserted_id.as_object_id();
            HttpResponse::Ok().json(recipe)
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/recipes/{id}")]
async fn update_recipe(
    data: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<UpdateRecipeRequest>,
) -> impl Responder {
    let id = path.into_inner();

    let oid = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid id"),
    };

    let update = doc! {
        "$set": {
            "title": body.title.clone(),
            "description": body.description.clone(),
        }
    };

    match data
        .recipes_collection
        .update_one(doc! { "_id": oid }, update)
        .await
    {
        Ok(result) if result.matched_count == 1 => {
            let updated = Recipe {
                id: Some(oid),
                title: body.title.clone(),
                description: body.description.clone(),
            };
            HttpResponse::Ok().json(updated)
        }
        Ok(_) => HttpResponse::NotFound().body("Recipe not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/recipes/{id}")]
async fn delete_recipe(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();

    let oid = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid id"),
    };

    match data.recipes_collection.delete_one(doc! { "_id": oid }).await {
        Ok(result) if result.deleted_count == 1 => HttpResponse::Ok().body("Deleted"),
        Ok(_) => HttpResponse::NotFound().body("Recipe not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
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
    let recipes_collection = database.collection::<Recipe>("recipes");

    let app_state = web::Data::new(AppState {
        todos_collection,
        recipes_collection,
    });

    println!("Server running on http://127.0.0.1:8080");

    HttpServer::new(move || {
        // Dev-only: allow any origin so the browser frontend can call the API.
        // Tighten this (e.g. .allowed_origin("http://localhost:5173")) for production.
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .service(home)
            .service(shmuel)
            .service(delete_todo)
            .service(create_todo)
            .service(get_all_todos)
            .service(get_all_recipes)
            .service(get_recipe)
            .service(create_recipe)
            .service(update_recipe)
            .service(delete_recipe)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}