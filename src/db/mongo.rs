use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client};
use std::env;
use dotenv::dotenv;

pub async fn connect_to_mongo() -> mongodb::error::Result<Client> {
    dotenv().ok(); // Загружаем переменные окружения из .env (если локально)
    let mongo_uri = env::var("MONGOURI").expect("MONGOURI must be set in environment variables");

    println!("🔌 Connecting to MongoDB...");

    let mut client_options = ClientOptions::parse(&mongo_uri).await?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options)?;

    // Проверяем подключение (ping)
    client.database("admin").run_command(doc! {"ping": 1}, None).await?;
    println!("✅ Successfully connected to MongoDB!");

    Ok(client)
}