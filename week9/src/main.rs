// write a Rust Lambda program that interacts with AWS DynamoDB using the rusoto_dynamodb crate
// https://docs.rs/rusoto_dynamodb/0.43.0/rusoto_dynamodb/index.html
// that can read and write data to a DynamoDB table
// get price of a coffee
// post a coffee order
// get a list of all coffee orders
// get total number of coffee orders

// Path: week9/src/main.rs
// Compare this snippet from week8/src/main.rs:
//     fact_list.facts[rng.gen_range(0..l)]
// }
//
// async fn build_success_response(fact: &'static str) -> Response<Body> {
//     json!({ "fact": fact }).into_response().await
// }
//
// async fn build_failure_response(error_message: &str) -> Response<Body> {
//     Response::builder()
//         .status(400)
//         .header("content-type", "application/json")
//         .body(Body::from(json!({ "error": error_message }).to_string()))
//         .expect("could not build the error response")
// }
//
// fn process_event(fact_list: &FactList) -> &'static str {
//     get_random_fact(fact_list)
// }
//
// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     let all_facts = &FactList::new();
//     let handler_func = |event: Request| async move {
//         let response = build_success_response(process_event(all_facts)).await;
//         Result::<Response<Body>, Error>::Ok(response)
//     };
//     run(service_fn(handler_func)).await?;
//     Ok(())
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]  
//     fn new_fact_list_test() {
//         let all_facts: FactList = FactList::new();
//         assert_eq!(5, all_facts.facts.len());
//     }
//
//     #[tokio::test]
//     async fn build_success_response_test() {
//         let test_fact = "This is a test fact.";
//         let result = build_success_response(test_fact).await;
//         let (parts, body) = result.into_parts();
//         assert_eq!(200, parts.status.as_u16());
//         assert_eq!(
//             "application/json",
//             parts.headers.get("content-type").unwrap()
//         );
//         assert_eq!(
//             "{\"fact\":\"This is a test fact.\"}",
//             String::from_utf8(body.to_ascii_lowercase()).unwrap()
//         );
//     }
//
//     #[tokio::test]
//     async fn build_failure_response_test() {
//         let result = build_failure_response("test error message").await;
//         let (parts, body) = result.into_parts();
//         assert_eq!(400, parts.status.as_u16());
//         assert_eq!(
//             "application/json",
//             parts.headers.get("content-type").unwrap()
//         );
//         assert_eq!(
//             "{\"error\":\"test error message\"}",
//             String::from_utf8(body.to_ascii_lowercase()).unwrap()
//         );
//     }
// }


use lambda_http::{
    aws_lambda_events::serde_json::json, run, service_fn, Body, Error, IntoResponse, Request,
    RequestExt, Response,
};

use serde::Serialize;


// connect to DynamoDB
// https://docs.rs/rusoto_dynamodb/0.43.0/rusoto_dynamodb/index.html
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, GetItemInput, PutItemInput};

// create a struct to hold the data we want to store in DynamoDB
// https://docs.rs/rusoto_dynamodb/0.43.0/rusoto_dynamodb/struct.PutItemInput.html
#[derive(Serialize)]
struct CoffeeOrder {
    id: String,
    name: String,
    size: String,
    price: String,
}

// create a function to get a coffee order from DynamoDB
// https://docs.rs/rusoto_dynamodb/0.43.0/rusoto_dynamodb/struct.GetItemInput.html

// create a function to put a coffee order into DynamoDB
// https://docs.rs/rusoto_dynamodb/0.43.0/rusoto_dynamodb/struct.PutItemInput.html

// create a function to get the price of a coffee from DynamoDB
// https://docs.rs/rusoto_dynamodb/0.43.0/rusoto_dynamodb/struct.GetItemInput.html

// create a function to get the total number of coffee orders from DynamoDB
// https://docs.rs/rusoto_dynamodb/0.43.0/rusoto_dynamodb/struct.GetItemInput.html

#[tokio::main]
async fn main() -> Result<(), Error> {
    let all_facts = &FactList::new();
    let handler_func = |event: Request| async move {
        let response = build_success_response(process_event(all_facts)).await;
        Result::<Response<Body>, Error>::Ok(response)
    };
    run(service_fn(handler_func)).await?;
    Ok(())
}

