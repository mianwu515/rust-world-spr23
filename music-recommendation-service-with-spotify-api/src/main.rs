// ! A Rust Actix microserver music recommendation service with Spotify API 
// ! by providing a genre.

/*An actix Microservice that has multiple routes:
A.  / that turns a hello world
B. /possible-genres that returns a list of possible genres
C. /genre/{genre} that returns a list of recommendations for the genre
*/

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

#[derive(Debug, Deserialize, Serialize)]
struct AccessTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u32,
}

// A.  / that turns a hello world
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// B. /possible-genres that returns a list of possible genres
#[get("/possible-genres")]
async fn possible_genres() -> impl Responder {
    let access_token = get_access_token("369dbd7452cf4e5fac093191ef5e6538",
    "9eee9c5a46834c0b8319c5e0c798b4f4").unwrap();
    let genres = get_possible_genres(&access_token).unwrap();
    HttpResponse::Ok().body(format!("{:?}", genres));
}

// C. /{genre} that returns a list of recommendations for the genre
#[get("/genre/{genre}")]
async fn genre_recommendations(genre: web::Path<String>) -> impl Responder {
    let access_token = get_access_token("369dbd7452cf4e5fac093191ef5e6538",
    "9eee9c5a46834c0b8319c5e0c798b4f4").unwrap();

    let recommendations = get_recommendations(&access_token, &genre).unwrap();
    HttpResponse::Ok().body(format!("{:?}", recommendations));
}

fn get_access_token(client_id: &str, client_secret: &str) ->   Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let body = "grant_type=client_credentials";
    let basic_auth = general_purpose::STANDARD.encode(format!("{}:{}", client_id, client_secret));

    let response = client.post("https://accounts.spotify.com/api/token")
    .header(reqwest::header::AUTHORIZATION, format!("Basic {}", basic_auth))
    .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
    .body(body)
    .send()?
    .json::<AccessTokenResponse>()?;
    Ok(response.access_token)
}

#[derive(Debug, Deserialize, Serialize)]
struct RecommendationResponse {
    tracks: Vec<Track>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Track {
    name: String,
    artists: Vec<Artist>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Artist {
    name: String,
}

fn get_recommendations(access_token: &str, genre: &str) -> Result<Vec<Track>, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::AUTHORIZATION, format!("Bearer {}", access_token).parse().unwrap());

    let response = client.get("https://api.spotify.com/v1/recommendations")
        .headers(headers)
        .query(&[("seed_genres", genre)])
        .send()?
        .json::<RecommendationResponse>()?;

    Ok(response.tracks)
}

fn get_possible_genres(access_token: &str) -> Result<RecommendationResponse, Box<dyn std::error::Error>>{
    let client = reqwest::blocking::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::AUTHORIZATION, format!("Bearer {}", access_token).parse().unwrap());

    let response = client.get("	https://api.spotify.com/v1/recommendations/available-genre-seeds")
        .headers(headers)
        .send()?
        .json::<RecommendationResponse>()?;
    Ok(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(possible_genres)
            .service(genre_recommendations)
    })
    .bind("http://localhost:8888")? //0.0.0.0:8080
    .run()
    .await
}

/* 
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = "369dbd7452cf4e5fac093191ef5e6538";
    let client_secret = "9eee9c5a46834c0b8319c5e0c798b4f4";
    let access_token = get_access_token(client_id, client_secret)?;

    println!("Access token: {}", access_token);
    
    // print out the recommendations for the genre "dance"
    let recommendations = get_recommendations(&access_token, "dance")?;
    for track in recommendations {
        println!("{} by {}", track.name, track.artists[0].name);
    }
    Ok(())
}
*/
