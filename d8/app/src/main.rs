use actix_web::{get, post, put, delete, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Row};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct City {
    id: i64,
    name: String,
}

#[derive(Debug, Deserialize)]
struct CreateCityRequest {
    name: String,
}

#[derive(Debug, Deserialize)]
struct UpdateCityRequest {
    name: String,
}

// Initialize database connection
async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    let database_url = "sqlite:src/mydb.db";
    let pool = SqlitePool::connect(database_url).await?;
    
    // Check if cities table exists and create if needed
    match sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name='cities'")
        .fetch_optional(&pool)
        .await?
    {
        Some(_) => {
            println!("✅ Cities table already exists in database");
        }
        None => {
            println!("📋 Creating cities table...");
            sqlx::query(
                r#"
                CREATE TABLE cities (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL
                )
                "#,
            )
            .execute(&pool)
            .await?;
            println!("✅ Cities table created successfully");
        }
    }
    
    Ok(pool)
}

#[post("/cities")]
async fn create_city(pool: web::Data<SqlitePool>, city: web::Json<CreateCityRequest>) -> impl Responder {
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
async fn get_cities(pool: web::Data<SqlitePool>) -> impl Responder {
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
async fn get_city(pool: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
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
async fn update_city(
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
async fn delete_city(pool: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize database
    let pool = init_db().await.expect("Failed to initialize database");
    
    println!("🚀 Server running at http://127.0.0.1:3004");
    println!("📊 SQLite database initialized at src/mydb.db");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(create_city)
            .service(get_cities)
            .service(get_city)
            .service(delete_city)
            .service(update_city)
    })
    .bind(("127.0.0.1", 3004))?
    .run()
    .await
}
