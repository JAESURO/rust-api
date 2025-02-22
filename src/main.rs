use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_files::Files;
use tera::{Tera, Context};
use std::sync::Mutex;

struct AppState {
    tera: Tera,
}

async fn index(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let tera = data.lock().unwrap();
    let mut ctx = Context::new();
    ctx.insert("title", "Welcome to E-Commerce Store");

    let rendered = tera.tera.render("index.ejs", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("src/views/**/*").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Mutex::new(AppState { tera: tera.clone() })))
            .route("/", web::get().to(index))
            .service(Files::new("/static", "src/views/static")) // Serve static files
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}