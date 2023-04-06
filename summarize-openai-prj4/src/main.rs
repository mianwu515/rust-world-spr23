use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;
use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::json;
use std::fs::File;
use std::io::prelude::*;
use actix_files::Files;

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
    let mut file = File::open("ui/index.html").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    HttpResponse::Ok().content_type("text/html").body(contents)
}

#[post("/summarize")]
async fn summarize(info: web::Json<SummaryRequest>) -> impl Responder {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
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
            Ok(r) => {
                match r.json::<serde_json::Value>().await {
                    Ok(result) => {
                        println!("Result JSON: {:?}", result);
                        if let Some(summary) = result["choices"][0]["text"].as_str() {
                            let summary = summary.to_owned();
                            let response = SummaryResponse { summary };
                            HttpResponse::Ok().json(response)
                        } else {
                            HttpResponse::InternalServerError().body("Error: Unable to retrieve summary")
                        }
                    }
                    Err(e) => HttpResponse::InternalServerError().body(format!("Error parsing JSON: {:?}", e)),
                }
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
            //.service(fs::Files::new("/", "./static").index_file("ui/index.html"))
            .service(Files::new("/static", "ui").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}