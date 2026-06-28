use actix_web::web;

use crate::controllers::user_controller::{login, register};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(register).service(login);
}
