use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;
use crate::models::product::{Product, ProductUpdateDTO};
use crate::controllers::item_controller::AppState;

pub async fn create_product(data: web::Data<AppState>, product: web::Json<Product>) -> impl Responder {
    let mut products = data.products.lock().unwrap();
    let product_data = product.into_inner();
    let new_product = Product::new(product_data.name, product_data.title, product_data.price, product_data.stock);
    products.push(new_product.clone());
    HttpResponse::Created().json(new_product)
}

pub async fn get_products(data: web::Data<AppState>) -> impl Responder {
    let products = data.products.lock().unwrap();
    HttpResponse::Ok().json(products.clone())
}

pub async fn get_product(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let products = data.products.lock().unwrap();
    let product = products.iter().find(|product| product.id == *path);
    match product {
        Some(p) => HttpResponse::Ok().json(p),
        None => HttpResponse::NotFound().body("Product not found")
    }
}

pub async fn delete_product(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    let mut products = data.products.lock().unwrap();
    if let Some(pos) = products.iter().position(|p| p.id == id) {
        products.remove(pos);
        return HttpResponse::Ok().body("Deleted");
    }
    HttpResponse::NotFound().body("Product not found")
}

pub async fn update_product(
    data: web::Data<AppState>,
    path: web::Path<String>,
    updated: web::Json<ProductUpdateDTO>,
) -> impl Responder {
    let id = path.into_inner();
    let mut products = data.products.lock().unwrap();
    if let Some(product) = products.iter_mut().find(|p| p.id == id) {
        product.name = updated.name.clone();
        product.title = updated.title.clone();
        product.price = updated.price;
        product.stock = updated.stock;
        return HttpResponse::Ok().json(product.clone());
    }
    HttpResponse::NotFound().body("Product not found")
}
