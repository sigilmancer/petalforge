use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use crate::models::task::Task;
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;


#[get("/tasks")]
pub async fn get_tasks(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Task>("SELECT * FROM tasks ORDER BY created_at DESC")
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(e) => {
            eprintln!("⚠️ Error fetching tasks: {}", e);
            HttpResponse::InternalServerError().body("Failed to retrieve tasks")
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub completed: Option<bool>,
}

#[post("/tasks")]
pub async fn create_task(
    pool: web::Data<PgPool>,
    payload: web::Json<CreateTask>,
) -> impl Responder {
    let result = sqlx::query_as::<_, Task>(
        r#"
        INSERT INTO tasks (id, title, completed, created_at)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(&payload.title)
    .bind(payload.completed.unwrap_or(false))
    .bind(Utc::now())
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(task) => HttpResponse::Created().json(task),
        Err(e) => {
            eprintln!("❌ Failed to insert task: {}", e);
            HttpResponse::InternalServerError().body("Failed to create task")
        }
    }
}
