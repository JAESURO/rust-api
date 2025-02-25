use actix_web::{web, App, HttpResponse, HttpServer, Responder, post, get};
use tera::{Tera, Context};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod db;

struct AppState {
    tera: Tera,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    email: String,
    password: String,
}

#[get("/")]
async fn index(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let tera = data.lock().unwrap();
    let mut ctx = Context::new();
    ctx.insert("title", "Welcome to E-Commerce Store");

    match tera.tera.render("index.ejs", &ctx) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(_) => HttpResponse::InternalServerError().body("Error rendering template"),
    }
}

#[get("/login")]
async fn login_page(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let tera = data.lock().unwrap();
    let mut ctx = Context::new();
    ctx.insert("title", "Login");

    match tera.tera.render("login.ejs", &ctx) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(_) => HttpResponse::InternalServerError().body("Error rendering template"),
    }
}

#[get("/register")]
async fn register(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let tera = data.lock().unwrap();
    let mut ctx = Context::new();
    ctx.insert("title", "Register");

    match tera.tera.render("register.ejs", &ctx) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(_) => HttpResponse::InternalServerError().body("Error rendering template"),
    }
}

#[post("/register")]
async fn register_user(form: web::Json<User>) -> impl Responder {
    println!("Received registration: {:?}", form);

    HttpResponse::Ok().json(format!("User {} registered successfully", form.username))
}

#[get("/users")]
async fn users_page(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let tera = data.lock().unwrap();
    let mut ctx = Context::new();
    ctx.insert("title", "Users");

    match tera.tera.render("dashboard.ejs", &ctx) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(_) => HttpResponse::InternalServerError().body("Error rendering template"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongo_connection = tokio::spawn(async {
        match db::mongo::connect_to_mongo().await {
            Ok(_) => println!("MongoDB connected successfully."),
            Err(e) => eprintln!("Error connecting to MongoDB: {}", e),
        }
    });

    let tera = Tera::new("src/views/**/*").expect("Failed to initialize Tera templates");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Mutex::new(AppState { tera: tera.clone() })))
            .service(index)
            .service(login_page)
            .service(register)
            .service(register_user)
            .service(users_page)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    mongo_connection.await?;

    Ok(())
}