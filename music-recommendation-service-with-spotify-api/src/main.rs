// ! A Rust Actix microserver music recommendation service with Spotify API 
// ! by providing a genre.

/*An actix Microservice that has multiple routes:
A.  / that turns a hello world
B. /possible-genres that returns a list of possible genres
C. /genre/{genre} that returns a list of recommendations for the genre
*/

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};

#[derive(Debug, Deserialize, Serialize)]
struct AccessTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u32,
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


async fn get_access_token(client_id: &str, client_secret: &str) ->   Result<String, Box<dyn std::error::Error>> {
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

async fn get_recommendations(access_token:  &str, genre_name:  &str) -> Result<Vec<Track>, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::AUTHORIZATION, format!("Bearer {}", access_token).parse().unwrap());

    let response = client.get("https://api.spotify.com/v1/recommendations")
    .headers(headers)
    .query(&[("seed_genres", genre_name)])
    .send()?
    .json::<RecommendationResponse>()?;

    Ok(response.tracks)
}

async fn get_possible_genres(access_token:  &str) -> Result<RecommendationResponse, Box<dyn std::error::Error>>{
    let client = reqwest::blocking::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::AUTHORIZATION, format!("Bearer {}", access_token).parse().unwrap());

    let response = client.get("	https://api.spotify.com/v1/recommendations/available-genre-seeds")
        .headers(headers)
        .send()?
        .json::<RecommendationResponse>()?;
    Ok(response)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/possible-genres")]
async fn possible_genres() -> impl Responder {
    let access_token = get_access_token("369dbd7452cf4e5fac093191ef5e6538", "12125765ddf14ff086d8b3252969f7a6").await.unwrap();
    let genres = get_possible_genres(&access_token).await.unwrap();
    HttpResponse::Ok().body(serde_json::to_string(&genres).unwrap())
}

#[get("/{genre}")]
async fn recommendations(genre: web::Path<String>) -> actix_web::Result<impl Responder> {
    let access_token = get_access_token("369dbd7452cf4e5fac093191ef5e6538", "12125765ddf14ff086d8b3252969f7a6").await.unwrap();
    let recommendations = get_recommendations(&access_token, &genre).await.unwrap();
    for track in recommendations {
        println!("{} by {}", track.name, track.artists[0].name);
    }
    Ok(HttpResponse::Ok())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(possible_genres)
            .service(recommendations)
    })
    .bind("localhost:6662")?
    .run()
    .await
}




