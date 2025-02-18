mod api;
mod models;
mod repository;

use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer, http};
use api::user_api::{create_user, get_user, update_user, delete_user, get_all_users}; //import the handler here
use repository::mongodb_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .wrap(
                Cors::default()
                    .allowed_origin("http://127.0.0.1:5500") // Allow frontend requests
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
                    .allow_any_origin()
                    .max_age(3600),
            )
            .service(create_user)
            .service(get_user)
            .service(update_user) 
            .service(delete_user) 
            .service(get_all_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}