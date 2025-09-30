use actix_web::{web, Scope};
use crate::controllers::worker_controller;

pub fn worker_routes() -> Scope {
    web::scope("")
        .service(worker_controller::create_worker)
        .service(worker_controller::get_workers)
        .service(worker_controller::get_worker)
        .service(worker_controller::update_worker)
        .service(worker_controller::delete_worker)
        .service(worker_controller::get_workers_by_city)
}
