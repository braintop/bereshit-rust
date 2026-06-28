use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn home() -> impl Responder {
    HttpResponse::Ok().body("Todo server is running")
}

#[get("/shmuel")]
pub async fn shmuel() -> impl Responder {
    HttpResponse::Ok().body("Smuael  is good man")
}
