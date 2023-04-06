use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use openai::{Completion, OpenAI, Prompt, Temperature};

use std::env;

async fn index() -> impl Responder {
    let html = r#"
        <html>
            <head>
                <title>Text Summarizer</title>
            </head>
            <body>
                <h1>Text Summarizer</h1>
                <form action="/summarize" method="post">
                    <label for="text">Text:</label>
                    <br>
                    <textarea id="text" name="text" rows="10" cols="50"></textarea>
                    <br>
                    <input type="submit" value="Summarize">
                </form>
                <div id="summary"></div>
            </body>
        </html>
    "#;
    HttpResponse::Ok().body(html)
}

async fn summarize(params: web::Form<FormData>) -> impl Responder {
    let openai_api_key = env::var("OPENAI_API_KEY").unwrap();

    let client = OpenAI::new(openai_api_key);
    let prompt = Prompt::new(&format!(
        "Summarize this text for a second-grade student:\n\n{}",
        params.text
    ));
    let temperature = Temperature::new(0.7);
    let response = client
        .complete(&prompt)
        .max_tokens(256)
        .temperature(temperature)
        .send()
        .await
        .unwrap();

    let summary = response.choices[0].text.clone();
    let html = format!(
        r#"
        <html>
            <head>
                <title>Text Summarizer</title>
            </head>
            <body>
                <h1>Text Summarizer</h1>
                <form action="/summarize" method="post">
                    <label for="text">Text:</label>
                    <br>
                    <textarea id="text" name="text" rows="10" cols="50">{}</textarea>
                    <br>
                    <input type="submit" value="Summarize">
                </form>
                <div id="summary">
                    <h2>Summary:</h2>
                    <p>{}</p>
                </div>
            </body>
        </html>
    "#,
        params.text, summary
    );

    HttpResponse::Ok().body(html)
}

#[derive(Deserialize)]
struct FormData {
    text: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/summarize", web::post().to(summarize))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
