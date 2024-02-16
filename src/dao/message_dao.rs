use super::base_dao::BaseDAO;
use crate::db_utils::init;
use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub chat_id: ObjectId,
    pub sender_id: ObjectId,
    pub message: String,
    // pub created_at: chrono::DateTime<chrono::Utc>,
    // pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone)]
pub struct MessageDAO {
    pub collection: Collection<Message>,
    pub db: Database,
}

impl MessageDAO {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        let db_uri = env::var("DB_URI").expect("DB_URI must be set");
        let db = init(&db_uri).await?;
        let collection = db.collection("message");
        Ok(Self { collection, db })
    }
}

#[async_trait]
impl BaseDAO<Message> for MessageDAO {
    async fn create(&self) -> Result<(), mongodb::error::Error> {
        self.db.create_collection("message", None).await?;
        Ok(())
    }

    async fn insert_document(&self, document: Message) -> Result<(), mongodb::error::Error> {
        println!("insert_document: {:?}", document);
        self.collection.insert_one(document, None).await?;
        Ok(())
    }
}
