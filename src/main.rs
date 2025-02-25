use actix_web::{web, App, HttpResponse, HttpServer, Responder, post, get};
use tera::{Tera, Context};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod db;

struct AppState {
    tera: Tera,
}

// Структура для данных пользователя
#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    email: String,
    password: String,
}

// 📌 Функция для рендеринга главной страницы
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

// 📌 Функция для рендеринга страницы входа
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

// 📌 Функция для рендеринга страницы регистрации (форма)
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

// 📌 Функция для обработки POST-запроса регистрации
#[post("/register")]
async fn register_user(form: web::Json<User>) -> impl Responder {
    println!("Received registration: {:?}", form);

    // Здесь можно добавить сохранение пользователя в базу данных
    HttpResponse::Ok().json(format!("User {} registered successfully", form.username))
}

// 📌 Функция для рендеринга страницы пользователей
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

// 📌 Главная функция для запуска сервера и подключения к MongoDB
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Подключение к MongoDB
    let mongo_connection = tokio::spawn(async {
        match db::mongo::connect_to_mongo().await {
            Ok(_) => println!("MongoDB connected successfully."),
            Err(e) => eprintln!("Error connecting to MongoDB: {}", e),
        }
    });

    // Инициализация Tera шаблонов
    let tera = Tera::new("src/views/**/*").expect("Failed to initialize Tera templates");

    // Запуск Actix веб-сервера
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Mutex::new(AppState { tera: tera.clone() })))
            .service(index)
            .service(login_page)
            .service(register)        // Страница регистрации (GET)
            .service(register_user)   // Регистрация пользователя (POST)
            .service(users_page)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    // Ждем завершения подключения к MongoDB
    mongo_connection.await?;

    Ok(())
}