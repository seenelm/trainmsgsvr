use super::base_dao::BaseDAO;
use crate::db_utils::init;
use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub chat_name: String,
    pub group_id: ObjectId,
    // pub participants: Vec<ObjectId>,
    // pub created_by: ObjectId,
    // pub created_at: chrono::DateTime<chrono::Utc>,
    // pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone)]
pub struct ChatDAO {
    pub collection: Collection<Chat>,
    pub db: Database,
}

impl ChatDAO {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        let db_uri = env::var("DB_URI").expect("DB_URI must be set");
        let db = init(&db_uri).await?;
        let collection = db.collection("chat");
        Ok(Self { collection, db })
    }
}

#[async_trait]
impl BaseDAO<Chat> for ChatDAO {
    async fn create(&self) -> Result<(), mongodb::error::Error> {
        self.db.create_collection("chat", None).await?;
        Ok(())
    }

    async fn insert_document(&self, document: Chat) -> Result<(), mongodb::error::Error> {
        println!("insert_document: {:?}", document);
        let filter = doc! { "group_id": &document.group_id };
        let result = self.collection.find_one(filter, None).await?;
        println!("result: {:?}", result);
        if result.is_some() {
            println!("group_id already exists in the collection");
        } else {
            println!("inserting document");
            self.collection.insert_one(document, None).await?;
            println!("inserted document");
        }
        Ok(())
    }
}
