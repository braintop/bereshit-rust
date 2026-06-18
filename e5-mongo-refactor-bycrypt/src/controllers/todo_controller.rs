use actix_web::{delete, get, post, web, HttpResponse, Responder};
use mongodb::bson::{doc, oid::ObjectId};

use crate::models::todo::{CreateTodoRequest, Todo};
use crate::state::AppState;

#[get("/todos")]
pub async fn get_all_todos(data: web::Data<AppState>) -> impl Responder {
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

#[post("/todos")]
pub async fn create_todo(
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

#[delete("/todos/{id}")]
pub async fn delete_todo(
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
