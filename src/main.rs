use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use tera::{Tera, Context};
use std::sync::Mutex;

struct AppState {
    tera: Tera,
}

// Function to render the index page
async fn index(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let tera = data.lock().unwrap();
    let mut ctx = Context::new();
    ctx.insert("title", "Welcome to E-Commerce Store");

    match tera.tera.render("index.ejs", &ctx) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(_) => HttpResponse::InternalServerError().body("Error rendering template"),
    }
}

// Function to render the login page
async fn login_page(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let tera = data.lock().unwrap();
    let mut ctx = Context::new();
    ctx.insert("title", "Login");

    match tera.tera.render("login.ejs", &ctx) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(_) => HttpResponse::InternalServerError().body("Error rendering template"),
    }
}

// Function to render the registration page
async fn register(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let tera = data.lock().unwrap();
    let mut ctx = Context::new();
    ctx.insert("title", "Register");

    match tera.tera.render("register.ejs", &ctx) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(_) => HttpResponse::InternalServerError().body("Error rendering template"),
    }
}

// Function to render the users dashboard
async fn users_page(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let tera = data.lock().unwrap();
    let mut ctx = Context::new();
    ctx.insert("title", "Users");

    match tera.tera.render("dashboard.ejs", &ctx) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(_) => HttpResponse::InternalServerError().body("Error rendering template"),
    }
}

// Main function to start the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("src/views/**/*").expect("Failed to initialize Tera templates");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Mutex::new(AppState { tera: tera.clone() })))
            .route("/", web::get().to(index))
            .route("/login", web::get().to(login_page))
            .route("/register", web::get().to(register))
            .route("/users", web::get().to(users_page))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}