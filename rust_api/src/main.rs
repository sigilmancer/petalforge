use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    message: String,
}

use actix_web::{post, web};
use serde::Deserialize;

#[derive(Deserialize)]
struct GreetRequest {
    name: String,
}

#[get("/hello")]
async fn hello() -> impl Responder {
    actix_web::web::Json(Message {
        message: "Hello from Rust in Docker!".to_string(),
    })
}

#[get("/goodbye")]
async fn goodbye() -> impl Responder {
    actix_web::web::Json(Message {
        message: "Goodbye from Rust!".to_string(),
    })
}

#[get("/greet")]
async fn greet(query: web::Query<GreetRequest>) -> impl Responder {
    let message = format!("Hello, {}! 👋", query.name);
    web::Json(Message { message })
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://0.0.0.0:8080");
    HttpServer::new(|| {
        App::new()
        .service(hello)
        .service(goodbye)
        .service(greet)
        .service(ping)
})
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
