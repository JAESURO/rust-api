use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{oid::ObjectId, doc},
    results::{InsertOneResult, UpdateResult, DeleteResult},
    Client, Collection,
};
use futures::stream::TryStreamExt;
use crate::models::user_model::User;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = env::var("MONGOURI").expect("MONGOURI must be set");
        let client = Client::with_uri_str(&uri).await.expect("Failed to initialize MongoDB client");
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, mongodb::error::Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        self.col.insert_one(new_doc, None).await
    }

    pub async fn get_user(&self, id: &String) -> Result<Option<User>, mongodb::error::Error> {
        let obj_id = ObjectId::parse_str(id)?;
        let filter = doc! { "_id": obj_id };
        self.col.find_one(filter, None).await
    }

    pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, mongodb::error::Error> {
        let obj_id = ObjectId::parse_str(id)?;
        let filter = doc! { "_id": obj_id };
        let new_doc = doc! {
            "$set": {
                "name": new_user.name,
                "location": new_user.location,
                "title": new_user.title,
            },
        };
        self.col.update_one(filter, new_doc, None).await
    }

    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, mongodb::error::Error> {
        let obj_id = ObjectId::parse_str(id)?;
        let filter = doc! { "_id": obj_id };
        self.col.delete_one(filter, None).await
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, mongodb::error::Error> {
        let mut cursors = self.col.find(None, None).await?;
        let mut users = Vec::new();
        while let Some(user) = cursors.try_next().await? {
            users.push(user);
        }
        Ok(users)
    }
}