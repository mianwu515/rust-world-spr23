// ! A Rust Actix microserver music recommendation service with Spotify API 
// ! by providing a theme and a number of recommendations to return.
// ! This is a simple example of using the Spotify API to get recommendations
// ! based on a theme and a number of recommendations to return.
// ! The Spotify API is a bit tricky to use, so this example is a bit
// ! more complex than it needs to be.

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use base64::Engine;

#[derive(Debug, Deserialize, Serialize)]
struct AccessTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u32,
}

fn get_access_token(client_id: &str, client_secret: &str) ->   Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let body = "grant_type=client_credentials";
    let basic_auth = Engine::encode(format!("{}:{}", client_id, client_secret));

    let response = client.post("https://accounts.spotify.com/api/token")
    .header(reqwest::header::AUTHORIZATION, format!("Basic {}", basic_auth))
    .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
    .body(body)
    .send()?
    .json::<AccessTokenResponse>()?;
    Ok(response.access_token)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = "YOUR_CLIENT_ID";
    let client_secret = "YOUR_CLIENT_SECRET";
    let access_token = get_access_token(client_id, client_secret)?;

    println!("Access token: {}", access_token);

    Ok(())
}



