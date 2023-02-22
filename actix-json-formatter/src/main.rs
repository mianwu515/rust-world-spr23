/*An actix Microservice that has multiple routes:
A.  / that turns a hello world
B. /[json string] that returns the formatted json
C. /version that returns the version of the service
D. /{name} that returns a hello {name}
*/

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actixjsonformatter::read_from_string;

// A.  / that turns a hello world
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// B. /[json string] that returns the formatted json
#[get("/{obj}")]
async fn jsonformatter(obj: web::Path<String>) -> impl Responder {
    let result = read_from_string(obj.to_string());
    println!("Result: {result}");
    HttpResponse::Ok().body(result)
}

// C. /version that returns the version of the service
#[get("/version")]
async fn version() -> impl Responder {
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    HttpResponse::Ok().body("Version 1.0")
}

// D. /{name} that returns a hello {name}
#[get("/{name}")]
async fn hello_name(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {name}!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running the service");
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(jsonformatter)
            .service(version)
            .service(hello_name)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

