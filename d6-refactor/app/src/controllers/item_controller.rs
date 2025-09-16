use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;
use crate::models::item::Item;
use crate::models::product::Product;

pub struct AppState {
    pub items: Mutex<Vec<Item>>,
    pub products: Mutex<Vec<Product>>,
}

pub async fn create_item(data: web::Data<AppState>, item: web::Json<Item>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    let item_data = item.into_inner();
    let new_item = Item::new(item_data.title, item_data.description);
    items.push(new_item.clone());
    HttpResponse::Created().json(new_item)
}

pub async fn get_items(data: web::Data<AppState>) -> impl Responder {
    let items = data.items.lock().unwrap();
    HttpResponse::Ok().json(items.clone())
}

pub async fn get_item(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let items = data.items.lock().unwrap();
    let item = items.iter().find(|item| item.id == *path);
    HttpResponse::Ok().json(item)
}

pub async fn delete_item(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    let mut items = data.items.lock().unwrap();
    if let Some(pos) = items.iter().position(|i| i.id == id) {
        items.remove(pos);
        return HttpResponse::Ok().body("Deleted");
    }
    HttpResponse::NotFound().body("Item not found")
}

pub async fn update_item(
    data: web::Data<AppState>,
    path: web::Path<String>,
    updated: web::Json<Item>,
) -> impl Responder {
    let id = path.into_inner();
    let mut items = data.items.lock().unwrap();
    if let Some(item) = items.iter_mut().find(|i| i.id == id) {
        item.title = updated.title.clone();
        item.description = updated.description.clone();
        return HttpResponse::Ok().json(item.clone());
    }
    HttpResponse::NotFound().body("Item not found")
}
