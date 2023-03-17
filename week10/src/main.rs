use lambda_http::{
    aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse},
    http::header::HeaderMap,
    lambda_runtime::{self, Context},
    IntoResponse, Request, RequestExt, Response,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct DadJoke {
    id: String,
    joke: String,
}

async fn handler(
    _: Request,
    _: Context,
) -> Result<impl IntoResponse, lambda_http::Error> {
    let client = Client::new();

    let dadjoke: DadJoke = client
        .get("https://icanhazdadjoke.com/")
        .header("Accept", "application/json")
        .header(
            "User-Agent",
            "Rust Adventure Serverless Examples (https://github.com/rust-adventure/netlify-serverless-examples)",
        )
        .send()
        .await?
        .json()
        .await?;

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: serde_json::to_string(&dadjoke).unwrap().into(),
    })
}

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    lambda_runtime::run(handler).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::StatusCode;

    #[tokio::test]
    async fn test_handler() {
        let response = handler(Request::default(), Context::default())
            .await
            .unwrap()
            .into_response();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().concat2().await.unwrap();
        let dadjoke: DadJoke = serde_json::from_slice(&body).unwrap();
        assert!(!dadjoke.joke.is_empty());
    }
}
