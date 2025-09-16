use actix_web::{web, Scope};
use crate::controllers::item_controller;

pub fn item_routes() -> Scope {
    web::scope("/items")
        .route("", web::post().to(item_controller::create_item))
        .route("", web::get().to(item_controller::get_items))
        .route("/{id}", web::get().to(item_controller::get_item))
        .route("/{id}", web::put().to(item_controller::update_item))
        .route("/{id}", web::delete().to(item_controller::delete_item))
}
