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
struct ImageRequest {
    prompt: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ImageResponse {
    image_url: String,
}

#[get("/")]
async fn index() -> impl Responder {
    let mut file = File::open("ui/index.html").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    HttpResponse::Ok().content_type("text/html").body(contents)
}

#[post("/generate-image")]
async fn generate_image(info: web::Json<ImageRequest>) -> impl Responder {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let url = "https://api.openai.com/v1/images/generations";
    let payload = json!({
        "prompt": info.prompt,
        "num_images": 1,
        "size": "256x256",
        "response_format": "url",
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
                    if let Some(image_url) = result["data"][0]["url"].as_str() {
                        let image_url = image_url.to_owned();
                        let response = ImageResponse { image_url };
                        HttpResponse::Ok().json(response)
                    } else {
                        HttpResponse::InternalServerError().body("Error: Unable to retrieve image URL")
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
            .service(generate_image)
            .service(Files::new("/static", "ui").index_file("index.html"))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
