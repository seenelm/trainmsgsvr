use crate::DataError;

use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Message {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub sender_id: ObjectId,
    pub conversation_id: ObjectId,
    pub text: String,
    // pub media_url: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct MessageDAO {
    pub collection: Collection<Message>,
}

impl MessageDAO {
    pub fn new(db: &Database) -> Result<Self, DataError> {
        let collection = db.collection("message");
        Ok(Self { collection })
    }

    pub async fn insert_document(&self, document: &Message) -> Result<ObjectId, DataError> {
        println!("insert_document: {:?}", document);
        let result = self.collection.insert_one(document, None).await;
        match result {
            Ok(insert_result) => match insert_result.inserted_id.as_object_id() {
                Some(inserted_id) => Ok(inserted_id),
                None => Err(DataError::InsertError(
                    "Insert ID is not an ObjectId".to_string(),
                )),
            },
            Err(e) => Err(DataError::QueryError(e)),
        }
    }

    pub async fn find_all(&self, conversation_id: &ObjectId) -> Result<Vec<Message>, DataError> {
        let filter = doc! {
            "conversation_id": conversation_id
        };

        let mut cursor = self.collection.find(filter, None).await?;
        let mut messages: Vec<Message> = Vec::new();

        while let Ok(result) = cursor.try_next().await {
            match result {
                Some(message) => messages.push(message),
                None => break,
            }
        }

        println!("Messages 1: {:?}", messages);

        Ok(messages)
    }
}
