use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::{SqlitePool, Row};
use crate::models::city::{City, CreateCityRequest, UpdateCityRequest};
use crate::controllers::worker_controller;

// Initialize database connection
pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    let database_url = "sqlite:src/mydb.db";
    let pool = SqlitePool::connect(database_url).await?;
    
    // Create cities table if it doesn't exist
    println!("📋 Creating cities table if not exists...");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS cities (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await?;
    println!("✅ Cities table ready");
    
    // Initialize workers table
    worker_controller::init_workers_table(&pool).await?;
    
    Ok(pool)
}

#[post("/cities")]
pub async fn create_city(pool: web::Data<SqlitePool>, city: web::Json<CreateCityRequest>) -> impl Responder {
    match sqlx::query("INSERT INTO cities (name) VALUES (?)")
        .bind(&city.name)
        .execute(&**pool)
        .await
    {
        Ok(result) => {
            let new_city = City {
                id: result.last_insert_rowid(),
                name: city.name.clone(),
            };
            HttpResponse::Created().json(new_city)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[get("/cities")]
pub async fn get_cities(pool: web::Data<SqlitePool>) -> impl Responder {
    match sqlx::query("SELECT id, name FROM cities ORDER BY id")
        .fetch_all(&**pool)
        .await
    {
        Ok(rows) => {
            let cities: Vec<City> = rows
                .iter()
                .map(|row| City {
                    id: row.get("id"),
                    name: row.get("name"),
                })
                .collect();
            HttpResponse::Ok().json(cities)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[get("/cities/{id}")]
pub async fn get_city(pool: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    
    match sqlx::query("SELECT id, name FROM cities WHERE id = ?")
        .bind(&id)
        .fetch_optional(&**pool)
        .await
    {
        Ok(Some(row)) => {
            let city = City {
                id: row.get("id"),
                name: row.get("name"),
            };
            HttpResponse::Ok().json(city)
        }
        Ok(None) => HttpResponse::NotFound().body("City not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[put("/cities/{id}")]
pub async fn update_city(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
    updated: web::Json<UpdateCityRequest>,
) -> impl Responder {
    let id = path.into_inner();
    
    match sqlx::query("UPDATE cities SET name = ? WHERE id = ?")
        .bind(&updated.name)
        .bind(&id)
        .execute(&**pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                let updated_city = City {
                    id: id,
                    name: updated.name.clone(),
                };
                HttpResponse::Ok().json(updated_city)
            } else {
                HttpResponse::NotFound().body("City not found")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[delete("/cities/{id}")]
pub async fn delete_city(pool: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    
    match sqlx::query("DELETE FROM cities WHERE id = ?")
        .bind(&id)
        .execute(&**pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                HttpResponse::Ok().body("Deleted")
            } else {
                HttpResponse::NotFound().body("City not found")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}
