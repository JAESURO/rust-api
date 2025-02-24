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
            .route("/login", web::get().to(login_page))  // Добавлен маршрут для логина
            .route("/register", web::get().to(register))  // Добавлен маршрут для логина
            .route("/users", web::get().to(users_page))  // Добавлен маршрут для логина
            .service(Files::new("/static", "src/views/static"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
async fn login_page(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let tera = data.lock().unwrap();
    let mut ctx = Context::new();
    ctx.insert("title", "Login");

    let rendered = tera.tera.render("login.ejs", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

async fn register(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let tera = data.lock().unwrap();
    let mut ctx = Context::new();
    ctx.insert("title", "Register");

    let rendered = tera.tera.render("register.ejs", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

async fn users_page(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let tera = data.lock().unwrap();
    let mut ctx = Context::new();
    ctx.insert("title", "Users");

    let rendered = tera.tera.render("dashboard.ejs", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}