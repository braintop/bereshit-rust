use actix_web::web;

use crate::controllers::todo_controller::{create_todo, delete_todo, get_all_todos};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_todos)
        .service(create_todo)
        .service(delete_todo);
}
