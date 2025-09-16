use actix_web::{web, Scope};
use crate::controllers::product_controller::{
    create_product, get_products, get_product, delete_product, update_product
};

pub fn product_routes() -> Scope {
    web::scope("/products")
        .route("", web::post().to(create_product))
        .route("", web::get().to(get_products))
        .route("/{id}", web::get().to(get_product))
        .route("/{id}", web::put().to(update_product))
        .route("/{id}", web::delete().to(delete_product))
}
