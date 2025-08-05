mod db;
mod models;
use dotenv::dotenv;
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
mod handlers;
use handlers::task_handler::get_tasks;
use handlers::task_handler::create_task;

use sqlx::PgPool;
use db::connect;

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("I'm Good!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool: PgPool = connect().await;

     println!("🚀 Server running at http://localhost:8080");
     println!("📡 Connected to Postgres at {}", std::env::var("DATABASE_URL").unwrap());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Share the database pool
            .service(health_check)
            .service(get_tasks)
            .service(create_task)
            .service(db_check)
    })
    .bind(("0.0.0.0", 8080))?
    .run().await
}   

#[get("/db-check")]
pub async fn db_check(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query("SELECT 1").execute(pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body("✅ Connected to DB!"),
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("❌ DB connection failed: {}", e)),
    }
}
