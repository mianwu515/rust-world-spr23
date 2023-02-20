// ! A Rust Actix microserver music recommendation service with Spotify API 
// ! by providing a genre.

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

#[derive(Debug, Deserialize, Serialize)]
struct AccessTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u32,
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



