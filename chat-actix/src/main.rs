mod handlers;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use reqwest::{header, Client};
use serde_json::json;
use serde_json::Value;
use std::env;
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct Conversation {
    text: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/chat")]
async fn chat(conversation: web::Json<Conversation>) -> impl Responder {
    let client = Client::new();

    // Use env variable OPENAI_API_KEY
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let url = "https://api.openai.com/v1/completions";

    let result = run_chat_loop(&client, &api_key, url, conversation.text.clone()).await;
    match result {
        Ok(text) => HttpResponse::Ok().json(Conversation { text }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn run_chat_loop(
    client: &Client,
    api_key: &str,
    url: &str,
    user_input: String,
) -> Result<String, reqwest::Error> {
    let mut conversation = String::from("The following is a conversation with an AI assistant. The assistant is helpful, creative, clever, and very friendly.\n");

    conversation.push_str("Human: ");
    conversation.push_str(&user_input);
    conversation.push_str("\nAI: ");

    let json = json!({
        "model": "text-davinci-003",
        "prompt": conversation,
        "temperature": 0.9,
        "max_tokens": 150,
        "top_p": 1.0,
        "frequency_penalty": 0.0,
        "presence_penalty": 0.6,
        "stop": [" Human:", " AI:"]
    });

    let body = call_api(client, api_key, url, json).await?;
    let ai_response = get_ai_response(&body);

    conversation.push_str(ai_response);
    conversation.push('\n');

    Ok(conversation)
}

async fn call_api(
    client: &Client,
    api_key: &str,
    url: &str,
    json: serde_json::Value,
) -> Result<Value, reqwest::Error> {
    let response = client
        .post(url)
        .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .header(header::CONTENT_TYPE, "application/json")
        .json(&json)
        .send()
        .await?;

    let body: Value = response.json().await?;
    Ok(body)
}

pub fn get_ai_response(body: &Value) -> &str {
    body["choices"][0]["text"].as_str().unwrap().trim()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", actix_web::web::get().to(handlers::index))
            .route("/chat", actix_web::web::post().to(handlers::chat))
    });

    server.bind("127.0.0.1:8000")?.run().await
}
