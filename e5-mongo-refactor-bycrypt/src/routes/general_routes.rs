use actix_web::web;

use crate::controllers::general_controller::{home, shmuel};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(home).service(shmuel);
}
