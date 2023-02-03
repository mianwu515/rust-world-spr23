/*An actix Microservice that has multiple routes:
A.  / that turns a hello world
B. /rock that returns the result of the game
C. /paper that returns the result of the game
D. /scissor that returns the result of the game
E. /version that returns the version of the service
F. /{name} that returns a hello {name}
*/

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actixdocker::play;

// A.  / that turns a hello world
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// B. /rock that returns the result of the game
#[get("/rock")]
async fn rock() -> impl Responder {
    let result = play("rock".to_string());
    println!("Result: {result}");
    HttpResponse::Ok().body(result)
}

// C. /paper that returns the result of the game
#[get("/paper")]
async fn paper() -> impl Responder {
    let result = play("paper".to_string());
    println!("Result: {result}");
    HttpResponse::Ok().body(result)
}

// D. /scissor that returns the result of the game
#[get("/scissors")]
async fn scissor() -> impl Responder {
    let result = play("scissors".to_string());
    println!("Result: {result}");
    HttpResponse::Ok().body(result)
}

// E. /version that returns the version of the service
#[get("/version")]
async fn version() -> impl Responder {
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    HttpResponse::Ok().body("Version 1.0")
}

// F. /{name} that returns a hello {name}
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
            .service(rock)
            .service(paper)
            .service(scissor)
            .service(version)
            .service(hello_name)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
