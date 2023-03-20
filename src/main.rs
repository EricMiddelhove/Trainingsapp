mod database_interactions;
mod dtos;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};

use std::env;

async fn index() -> impl Responder {
    "Hello index!"
}

async fn signup(info: web::Json<dtos::SignupUser>) -> impl Responder {
    let user = database_interactions::user::User::new(&info.name, &info.email, &info.password);

    let client = get_new_client().await;

    let inserted_user = user.database_insert(&client).await;

    HttpResponse::Ok().body(inserted_user.inserted_id.to_string())
}

async fn create_databases(client: &Client) {
    let new_user = database_interactions::user::User::new(
        &String::from("John"),
        &String::from("john@doe.com"),
        &String::from("password"),
    );

    let insert_result = new_user.database_insert(client).await;
    println!("Insert Result ID: {:?}", insert_result.inserted_id);
}

async fn get_new_client() -> Client {
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");

    let options =
        ClientOptions::parse_with_resolver_config(&mongo_uri, ResolverConfig::cloudflare())
            .await
            .expect("Failed to parse options");

    let client = Client::with_options(options).expect("Failed to initialize client.");
    println!("Connected to MongoDB!");

    client
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/signup", web::post().to(signup))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
