use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;
use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct SummaryRequest {
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SummaryResponse {
    summary: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the summary service!")
}

#[post("/summarize")]
async fn summarize(info: web::Json<SummaryRequest>) -> impl Responder {
    let api_key = env::var("OPENAI_API_KEY").unwrap();
    let url = "https://api.openai.com/v1/engines/text-davinci-003/completions";
    let prompt = format!("Summarize this for a second-grade student:\n\n{}", info.text);
    let payload = json!({
        "prompt": prompt,
        "temperature": 0.7,
        "max_tokens": 256,
        "top_p": 1,
        "frequency_penalty": 0,
        "presence_penalty": 0
    });

    let client = reqwest::Client::new();
    let res = client.post(url)
        .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .header(header::CONTENT_TYPE, "application/json")
        .json(&payload)
        .send()
        .await;

    match res {
        Ok(mut r) => {
            let result: Value = r.json().await.unwrap();
            let summary = result["choices"][0]["text"].as_str().unwrap().to_owned();
            let response = SummaryResponse { summary };
            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(summarize)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
