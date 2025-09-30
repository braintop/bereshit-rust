use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::{SqlitePool, Row};
use crate::models::worker::{Worker, CreateWorkerRequest, UpdateWorkerRequest, WorkerWithCity};

// Initialize workers table
pub async fn init_workers_table(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("📋 Creating workers table if not exists...");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS workers (
            worker_id INTEGER PRIMARY KEY AUTOINCREMENT,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            salary REAL NOT NULL,
            city_id INTEGER NOT NULL,
            FOREIGN KEY (city_id) REFERENCES cities (id)
        )
        "#,
    )
    .execute(pool)
    .await?;
    println!("✅ Workers table ready");
    
    Ok(())
}

#[post("/workers")]
pub async fn create_worker(pool: web::Data<SqlitePool>, worker: web::Json<CreateWorkerRequest>) -> impl Responder {
    // Check if city exists
    match sqlx::query("SELECT id FROM cities WHERE id = ?")
        .bind(&worker.city_id)
        .fetch_optional(&**pool)
        .await
    {
        Ok(Some(_)) => {
            // City exists, create worker
            match sqlx::query("INSERT INTO workers (first_name, last_name, salary, city_id) VALUES (?, ?, ?, ?)")
                .bind(&worker.first_name)
                .bind(&worker.last_name)
                .bind(&worker.salary)
                .bind(&worker.city_id)
                .execute(&**pool)
                .await
            {
                Ok(result) => {
                    let new_worker = Worker {
                        worker_id: result.last_insert_rowid(),
                        first_name: worker.first_name.clone(),
                        last_name: worker.last_name.clone(),
                        salary: worker.salary,
                        city_id: worker.city_id,
                    };
                    HttpResponse::Created().json(new_worker)
                }
                Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
            }
        }
        Ok(None) => HttpResponse::BadRequest().body("City with provided city_id does not exist"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[get("/workers")]
pub async fn get_workers(pool: web::Data<SqlitePool>) -> impl Responder {
    match sqlx::query(
        r#"
        SELECT w.worker_id, w.first_name, w.last_name, w.salary, w.city_id, c.name as city_name
        FROM workers w
        JOIN cities c ON w.city_id = c.id
        ORDER BY w.worker_id
        "#
    )
    .fetch_all(&**pool)
    .await
    {
        Ok(rows) => {
            let workers: Vec<WorkerWithCity> = rows
                .iter()
                .map(|row| WorkerWithCity {
                    worker_id: row.get("worker_id"),
                    first_name: row.get("first_name"),
                    last_name: row.get("last_name"),
                    salary: row.get("salary"),
                    city_id: row.get("city_id"),
                    city_name: row.get("city_name"),
                })
                .collect();
            HttpResponse::Ok().json(workers)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[get("/workers/{worker_id}")]
pub async fn get_worker(pool: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
    let worker_id = path.into_inner();
    
    match sqlx::query(
        r#"
        SELECT w.worker_id, w.first_name, w.last_name, w.salary, w.city_id, c.name as city_name
        FROM workers w
        JOIN cities c ON w.city_id = c.id
        WHERE w.worker_id = ?
        "#
    )
    .bind(&worker_id)
    .fetch_optional(&**pool)
    .await
    {
        Ok(Some(row)) => {
            let worker = WorkerWithCity {
                worker_id: row.get("worker_id"),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                salary: row.get("salary"),
                city_id: row.get("city_id"),
                city_name: row.get("city_name"),
            };
            HttpResponse::Ok().json(worker)
        }
        Ok(None) => HttpResponse::NotFound().body("Worker not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[put("/workers/{worker_id}")]
pub async fn update_worker(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
    updated: web::Json<UpdateWorkerRequest>,
) -> impl Responder {
    let worker_id = path.into_inner();
    
    // Check if city exists
    match sqlx::query("SELECT id FROM cities WHERE id = ?")
        .bind(&updated.city_id)
        .fetch_optional(&**pool)
        .await
    {
        Ok(Some(_)) => {
            // City exists, update worker
            match sqlx::query("UPDATE workers SET first_name = ?, last_name = ?, salary = ?, city_id = ? WHERE worker_id = ?")
                .bind(&updated.first_name)
                .bind(&updated.last_name)
                .bind(&updated.salary)
                .bind(&updated.city_id)
                .bind(&worker_id)
                .execute(&**pool)
                .await
            {
                Ok(result) => {
                    if result.rows_affected() > 0 {
                        let updated_worker = Worker {
                            worker_id: worker_id,
                            first_name: updated.first_name.clone(),
                            last_name: updated.last_name.clone(),
                            salary: updated.salary,
                            city_id: updated.city_id,
                        };
                        HttpResponse::Ok().json(updated_worker)
                    } else {
                        HttpResponse::NotFound().body("Worker not found")
                    }
                }
                Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
            }
        }
        Ok(None) => HttpResponse::BadRequest().body("City with provided city_id does not exist"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[delete("/workers/{worker_id}")]
pub async fn delete_worker(pool: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
    let worker_id = path.into_inner();
    
    match sqlx::query("DELETE FROM workers WHERE worker_id = ?")
        .bind(&worker_id)
        .execute(&**pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                HttpResponse::Ok().body("Worker deleted successfully")
            } else {
                HttpResponse::NotFound().body("Worker not found")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[get("/workers/city/{city_id}")]
pub async fn get_workers_by_city(pool: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
    let city_id = path.into_inner();
    
    match sqlx::query(
        r#"
        SELECT w.worker_id, w.first_name, w.last_name, w.salary, w.city_id, c.name as city_name
        FROM workers w
        JOIN cities c ON w.city_id = c.id
        WHERE w.city_id = ?
        ORDER BY w.worker_id
        "#
    )
    .bind(&city_id)
    .fetch_all(&**pool)
    .await
    {
        Ok(rows) => {
            let workers: Vec<WorkerWithCity> = rows
                .iter()
                .map(|row| WorkerWithCity {
                    worker_id: row.get("worker_id"),
                    first_name: row.get("first_name"),
                    last_name: row.get("last_name"),
                    salary: row.get("salary"),
                    city_id: row.get("city_id"),
                    city_name: row.get("city_name"),
                })
                .collect();
            HttpResponse::Ok().json(workers)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}
