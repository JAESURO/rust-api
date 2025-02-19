mod api;
mod models;
mod repository;

use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer, http, get, HttpResponse, Responder};
use api::user_api::{create_user, get_user, update_user, delete_user, get_all_users}; //import the handler here
use repository::mongodb_repo::MongoRepo;
use std::env;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Rust API is running! 🚀")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port: u16 = port.parse().expect("Invalid PORT");

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .wrap(
                Cors::default()
                    .allowed_origin("https://rust-api-front.onrender.com")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
                    .max_age(3600),
            )
            .service(index)
            .service(create_user)
            .service(get_user)
            .service(update_user) 
            .service(delete_user) 
            .service(get_all_users)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}