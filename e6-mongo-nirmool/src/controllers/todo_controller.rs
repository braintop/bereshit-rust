use actix_web::{delete, get, post, web, HttpResponse, Responder};
use mongodb::bson::{doc, from_document, oid::ObjectId};

use crate::models::todo::{CreateTodoRequest, Todo, TodoPopulated, TodoPopulatedResponse};
use crate::state::AppState;

/// Returns every todo with its related student embedded via MongoDB `$lookup`
/// (the Rust/Mongo equivalent of Mongoose `.populate('student')`).
#[get("/todos")]
pub async fn get_all_todos(data: web::Data<AppState>) -> impl Responder {
    let pipeline = vec![
        doc! {
            "$lookup": {
                "from": "students",
                "localField": "student_id",
                "foreignField": "_id",
                "as": "student"
            }
        },
        doc! {
            "$unwind": {
                "path": "$student",
                "preserveNullAndEmptyArrays": true
            }
        },
    ];

    let mut cursor = match data.todos_collection.aggregate(pipeline).await {
        Ok(cursor) => cursor,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let mut todos: Vec<TodoPopulatedResponse> = Vec::new();

    while cursor.advance().await.unwrap_or(false) {
        match cursor.deserialize_current() {
            Ok(doc) => match from_document::<TodoPopulated>(doc) {
                Ok(todo) => todos.push(todo.into()),
                Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
            },
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
    let student_id = match ObjectId::parse_str(&body.student_id) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid student_id"),
    };

    match data
        .students_collection
        .find_one(doc! { "_id": student_id })
        .await
    {
        Ok(Some(_)) => {}
        Ok(None) => return HttpResponse::BadRequest().body("Student not found"),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    }

    let mut todo = Todo {
        id: None,
        message: body.message.clone(),
        done: false,
        student_id,
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
