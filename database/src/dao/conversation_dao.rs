use crate::DataError;

use crate::model::conversation::Conversation;
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{Collection, Database};

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
    pub async fn find_one(&self, owner_id: &ObjectId) -> Result<Option<Conversation>, DataError> {
        let filter = doc! {
            "$or": [
                { "owner.user.id": owner_id },
                { "members.user.id": owner_id }
            ]
        };

        let result = self.collection.find_one(filter, None).await?;

        Ok(result)
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
                    // Check if conversation is a group.
                    if conversation.members.len() > 1 {
                        conversations.push(conversation)
                    } else {
                        if conversation.owner.id == *user_id {
                            conversation.name = conversation.members[0].name.clone();
                        } else {
                            let owner_name = conversation.owner.name.clone();
                            conversation.name = owner_name;
                        }
                        conversations.push(conversation);
                    }
                }
                None => break,
            }
        }

        Ok(conversations)
    }
}
