use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use mongodb::{bson::doc, options::FindOptions, Client, options::ClientOptions};
use std::collections::HashMap;
use tabulate::tabulate;
use tabulate::Table;
use futures::stream::StreamExt;

const EARTH_RADIUS: f64 = 6371.0;
const MDB_URL: &str = "mongodb+srv://readonly:readonly@covid-19.hip2i.mongodb.net/covid19";

use mongodb::bson::Document;


#[get("/query")]
async fn query(params: web::Query<HashMap<String, String>>) -> impl Responder {
    let query = params.get("query").unwrap_or(&"".to_string());

    let client_options = ClientOptions::parse(MDB_URL).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("covid19");
    let stats = db.collection("global_and_us");

    let filter = Document::parse(&query).unwrap();
    let mut cursor = stats.find(filter, None).await.unwrap();
    let mut results = Vec::new();
    while let Some(doc) = cursor.next().await? {
        if let Ok(doc) = doc {
            results.push(doc);
        }
    }

    let headers = vec!["date", "confirmed", "deaths"];
    let records = results.iter().map(|doc| extract_tuple(doc, &headers));
    let table = Table::from_rows(records);
    let table_html = table.render_html();

    HttpResponse::Ok().body(table_html)
}


fn extract_tuple(mapping: &mongodb::bson::document::Document, keys: &[&str]) -> Vec<String> {
    /*
    Extract a tuple from a mapping by requesting a sequence of keys.

    Missing keys will result in `None` values in the resulting tuple.
    */
    let values: Vec<String> = keys.iter()
                                  .map(|key| mapping.get(key).and_then(|v| v.to_string()).unwrap_or_else(|| "None".to_string()))
                                  .collect();
    values
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(r#"
        <html>
            <body>
                <h1>MongoDB Query</h1>
                <form method="post" action="/query">
                    <label for="query">Query:</label>
                    <input type="text" name="query">
                    <br><br>
                    <button type="submit">Submit</button>
                </form>
            </body>
        </html>
    "#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(query)
            .route("/", web::get().to(index))
            .service(actix_files::Files::new("/static", "./static"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
