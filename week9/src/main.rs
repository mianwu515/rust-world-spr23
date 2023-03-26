
// create a function to get a coffee order from DynamoDB
// https://docs.rs/rusoto_dynamodb/0.43.0/rusoto_dynamodb/struct.GetItemInput.html

// create a function to put a coffee order into DynamoDB
// https://docs.rs/rusoto_dynamodb/0.43.0/rusoto_dynamodb/struct.PutItemInput.html

// create a function to get the price of a coffee from DynamoDB
// https://docs.rs/rusoto_dynamodb/0.43.0/rusoto_dynamodb/struct.GetItemInput.html

// create a function to get the total number of coffee orders from DynamoDB
// https://docs.rs/rusoto_dynamodb/0.43.0/rusoto_dynamodb/struct.GetItemInput.html

use mongodb::{bson::doc, options::ClientOptions, Client};
#[tokio::main]
    async fn main() -> mongodb::error::Result<()> {
        let client_options = ClientOptions::parse(
            "mongodb+srv://mw515:<password>@cluster0.uymuwvl.mongodb.net/?retryWrites=true&w=majority",
        )
        .await?;
        let client = Client::with_options(client_options)?;
        let database = client.database("testDB");
        Ok(())
    }

