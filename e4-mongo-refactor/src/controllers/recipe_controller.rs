use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use mongodb::bson::{doc, oid::ObjectId};

use crate::models::recipe::{CreateRecipeRequest, Recipe, UpdateRecipeRequest};
use crate::state::AppState;

#[get("/recipes")]
pub async fn get_all_recipes(data: web::Data<AppState>) -> impl Responder {
    let mut cursor = match data.recipes_collection.find(doc! {}).await {
        Ok(cursor) => cursor,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let mut recipes: Vec<Recipe> = Vec::new();

    while cursor.advance().await.unwrap_or(false) {
        match cursor.deserialize_current() {
            Ok(recipe) => recipes.push(recipe),
            Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
        }
    }

    HttpResponse::Ok().json(recipes)
}

#[get("/recipes/{id}")]
pub async fn get_recipe(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();

    let oid = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid id"),
    };

    match data.recipes_collection.find_one(doc! { "_id": oid }).await {
        Ok(Some(recipe)) => HttpResponse::Ok().json(recipe),
        Ok(None) => HttpResponse::NotFound().body("Recipe not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/recipes")]
pub async fn create_recipe(
    data: web::Data<AppState>,
    body: web::Json<CreateRecipeRequest>,
) -> impl Responder {
    let mut recipe = Recipe {
        id: None,
        title: body.title.clone(),
        description: body.description.clone(),
    };

    match data.recipes_collection.insert_one(&recipe).await {
        Ok(result) => {
            recipe.id = result.inserted_id.as_object_id();
            HttpResponse::Ok().json(recipe)
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/recipes/{id}")]
pub async fn update_recipe(
    data: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<UpdateRecipeRequest>,
) -> impl Responder {
    let id = path.into_inner();

    let oid = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid id"),
    };

    let update = doc! {
        "$set": {
            "title": body.title.clone(),
            "description": body.description.clone(),
        }
    };

    match data
        .recipes_collection
        .update_one(doc! { "_id": oid }, update)
        .await
    {
        Ok(result) if result.matched_count == 1 => {
            let updated = Recipe {
                id: Some(oid),
                title: body.title.clone(),
                description: body.description.clone(),
            };
            HttpResponse::Ok().json(updated)
        }
        Ok(_) => HttpResponse::NotFound().body("Recipe not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/recipes/{id}")]
pub async fn delete_recipe(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();

    let oid = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid id"),
    };

    match data.recipes_collection.delete_one(doc! { "_id": oid }).await {
        Ok(result) if result.deleted_count == 1 => HttpResponse::Ok().body("Deleted"),
        Ok(_) => HttpResponse::NotFound().body("Recipe not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
