mod model;
//#[cfg(test)]
//mod test;

use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use model::User;
use mongodb::{bson::doc, options::ClientOptions, options::IndexOptions, Client, Collection, IndexModel};

const DB_NAME: &str = "myApp";
const COLL_NAME: &str = "users";

#[actix_web::main]
    async fn main() -> mongodb::error::Result<()> {
        let client_options = ClientOptions::parse(
            "mongodb+srv://mw515:<password>@cluster0.uymuwvl.mongodb.net/?retryWrites=true&w=majority",
        )
        .await?;
        let client = Client::with_options(client_options)?;

        
        let database = client.database("myApp");
        

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(client.clone()))
                //.service(add_user)
                //.service(get_user)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await;

        Ok(())
    }