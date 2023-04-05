use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ChatRequest {
    message: String,
}

pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

pub async fn chat(data: web::Json<ChatRequest>) -> HttpResponse {
    let message = &data.message;
    HttpResponse::Ok().body(format!("Received message: {}", message))
}
