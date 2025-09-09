use actix_web::{get, post, put, delete, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Item {
    id: String,
    name: String,
}

struct AppState {
    items: Mutex<Vec<Item>>,
}

#[post("/items")]
async fn create_item(data: web::Data<AppState>, item: web::Json<Item>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    let mut new_item = item.into_inner();
    new_item.id = Uuid::new_v4().to_string();
    items.push(new_item.clone());
    HttpResponse::Created().json(new_item)
}

#[get("/items")]
async fn get_items(data: web::Data<AppState>) -> impl Responder {
    let items = data.items.lock().unwrap();
    HttpResponse::Ok().json(items.clone())
}

#[get("/items/{id}")]
async fn get_item(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    println!("{:?}", path);
    let items = data.items.lock().unwrap();
    let item = items.iter().find(|item| item.id == *path);
    HttpResponse::Ok().json(item)
}
#[delete("/items/{id}")]
async fn delete_item(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    let mut items = data.items.lock().unwrap();
    if let Some(pos) = items.iter().position(|i| i.id == id) {
        items.remove(pos);
        return HttpResponse::Ok().body("Deleted");
    }
    HttpResponse::NotFound().body("Item not found")
}

#[put("/items/{id}")]
async fn update_item(
    data: web::Data<AppState>,
    path: web::Path<String>,
    updated: web::Json<Item>,
) -> impl Responder {
    let id = path.into_inner();
    let mut items = data.items.lock().unwrap();
    if let Some(item) = items.iter_mut().find(|i| i.id == id) {
        item.name = updated.name.clone();
        return HttpResponse::Ok().json(item.clone());
    }
    HttpResponse::NotFound().body("Item not found")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        items: Mutex::new(vec![]),
    });

    println!("🚀 Server running at http://127.0.0.1:3002");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(create_item)
            .service(get_items)
            .service(get_item)
            .service(delete_item)
            .service(update_item)
    })
    .bind(("127.0.0.1", 3002))?
    .run()
    .await
}