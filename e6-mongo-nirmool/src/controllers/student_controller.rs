use actix_web::{get, post, web, HttpResponse, Responder};

use crate::models::student::{CreateStudentRequest, Student, StudentResponse};
use crate::state::AppState;

#[post("/students")]
pub async fn create_student(
    data: web::Data<AppState>,
    body: web::Json<CreateStudentRequest>,
) -> impl Responder {
    let mut student = Student {
        id: None,
        name: body.name.clone(),
    };

    match data.students_collection.insert_one(&student).await {
        Ok(result) => {
            student.id = result.inserted_id.as_object_id();
            HttpResponse::Ok().json(StudentResponse::from(&student))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/students")]
pub async fn get_all_students(data: web::Data<AppState>) -> impl Responder {
    let mut cursor = match data.students_collection.find(mongodb::bson::doc! {}).await {
        Ok(cursor) => cursor,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let mut students: Vec<StudentResponse> = Vec::new();

    while cursor.advance().await.unwrap_or(false) {
        match cursor.deserialize_current() {
            Ok(student) => students.push(StudentResponse::from(&student)),
            Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
        }
    }

    HttpResponse::Ok().json(students)
}
