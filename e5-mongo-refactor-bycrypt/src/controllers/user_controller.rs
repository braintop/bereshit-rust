use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use mongodb::bson::doc;

use crate::models::user::{AuthRequest, User, UserResponse};
use crate::state::AppState;

#[post("/register")]
pub async fn register(
    data: web::Data<AppState>,
    body: web::Json<AuthRequest>,
) -> impl Responder {
    // Reject if the email is already taken.
    match data
        .users_collection
        .find_one(doc! { "email": &body.email })
        .await
    {
        Ok(Some(_)) => return HttpResponse::Conflict().body("Email already registered"),
        Ok(None) => {}
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    }

    // Hash the plaintext password; the salt is generated and embedded automatically.
    let hashed = match hash(&body.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let mut user = User {
        id: None,
        email: body.email.clone(),
        password: hashed,
    };

    match data.users_collection.insert_one(&user).await {
        Ok(result) => {
            user.id = result.inserted_id.as_object_id();
            HttpResponse::Ok().json(UserResponse::from(&user))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/login")]
pub async fn login(
    data: web::Data<AppState>,
    body: web::Json<AuthRequest>,
) -> impl Responder {
    let user = match data
        .users_collection
        .find_one(doc! { "email": &body.email })
        .await
    {
        Ok(Some(user)) => user,
        // Same message whether the email is missing or the password is wrong,
        // so we don't leak which emails exist.
        Ok(None) => return HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    match verify(&body.password, &user.password) {
        Ok(true) => HttpResponse::Ok().json(UserResponse::from(&user)),
        Ok(false) => HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
