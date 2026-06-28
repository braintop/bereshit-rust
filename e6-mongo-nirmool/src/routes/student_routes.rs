use actix_web::web;

use crate::controllers::student_controller::{create_student, get_all_students};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(create_student).service(get_all_students);
}
