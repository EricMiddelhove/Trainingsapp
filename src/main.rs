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

async fn create_apparatus(info: web::Json<dtos::CreateApparatus>) -> impl Responder {
    let userid: String = info.userid.to_string();

    let apparatus = database_interactions::user::apparatus::Apparatus::new(
        &Some(info.name.to_string()),
        &Some(info.description.to_string()),
        &Some(info.repetitions),
        &Some(info.sets),
        &Some(info.notes.to_string()),
        None,
    );

    let client = get_new_client().await;

    let apparatus_id = apparatus.database_insert(&client, userid).await;

    HttpResponse::Ok().body(apparatus_id)
}

async fn patch_apparatus(info: web::Json<dtos::PatchApparatus>) -> impl Responder {
    let userid: String = info.userid.to_string();
    let apparatusid: String = info.apparatusid.to_string();

    let apparatus = database_interactions::user::apparatus::Apparatus::new(
        &info.name,
        &info.description,
        &info.repetitions,
        &info.sets,
        &info.notes,
        Some(apparatusid.clone()),
    );

    let client = get_new_client().await;

    let apparatus_id: String = apparatus
        .database_update(&client, userid, apparatusid)
        .await;

    HttpResponse::Ok().body(apparatus_id)
}

async fn get_new_client() -> Client {
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");

    let options =
        ClientOptions::parse_with_resolver_config(&mongo_uri, ResolverConfig::cloudflare())
            .await
            .expect("Failed to parse options");

    let client = Client::with_options(options).expect("Failed to initialize client.");

    client
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/user", web::post().to(signup))
            .route("/apparatus", web::post().to(create_apparatus))
            .route("/apparatus", web::patch().to(patch_apparatus))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
