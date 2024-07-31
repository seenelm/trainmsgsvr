use crate::DataError;

use async_trait::async_trait;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct User {
    pub id: ObjectId,
    pub name: String,
}

// Make Conversation name a String.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Conversation {
    pub _id: ObjectId,
    pub name: Option<String>,
    pub owner_id: ObjectId,
    pub owner_name: String,
    pub members: Vec<User>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct ConversationDAO {
    pub collection: Collection<Conversation>,
}

impl ConversationDAO {
    pub fn new(db: &Database) -> Result<Self, DataError> {
        let collection = db.collection("conversation");
        Ok(Self { collection })
    }
}

impl ConversationDAO {
    pub async fn insert_document(&self, document: &Conversation) -> Result<ObjectId, DataError> {
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

    // Find conversation by owner_id and members id.
    pub async fn find_one(
        &self,
        owner_id: &ObjectId,
        name: &str,
    ) -> Result<Conversation, DataError> {
        let filter = doc! { "owner_id": owner_id, "name": name };
        let result = self.collection.find_one(filter, None).await?;

        match result {
            Some(conversation) => {
                println!("Conversation exists");
                Ok(conversation)
            }
            None => {
                println!("Conversation does not exist");
                return Err(DataError::NotFoundError(
                    "Conversation not found".to_string(),
                ));
            }
        }
    }

    pub async fn find_all(&self, user_id: &ObjectId) -> Result<Vec<Conversation>, DataError> {
        let filter = doc! {
            "$or": [
                { "owner_id": user_id },
                { "members.id": user_id }
            ]
        };

        let mut cursor = self.collection.find(filter, None).await?;
        let mut conversations: Vec<Conversation> = Vec::new();

        while let Ok(result) = cursor.try_next().await {
            match result {
                Some(mut conversation) => {
                    println!("conversation: {:?}", conversation);
                    // Check if conversation is a group.
                    if conversation.members.len() > 1 {
                        conversations.push(conversation)
                    } else {
                        println!("Not a group");
                        if conversation.owner_id == *user_id {
                            conversation.name = Some(conversation.members[0].name.clone());
                        } else {
                            let owner_name = conversation.owner_name.clone();
                            conversation.name = Some(owner_name);
                        }
                        conversations.push(conversation);
                    }
                }
                None => break,
            }
        }

        println!("conversations 1: {:?}", conversations);

        Ok(conversations)
    }
}
