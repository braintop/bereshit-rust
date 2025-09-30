use actix_web::{web, Scope};
use crate::controllers::city_controller;

pub fn city_routes() -> Scope {
    web::scope("")
        .service(city_controller::create_city)
        .service(city_controller::get_cities)
        .service(city_controller::get_city)
        .service(city_controller::update_city)
        .service(city_controller::delete_city)
}
