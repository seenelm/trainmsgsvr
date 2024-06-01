use crate::DataError;

use super::base_dao::BaseDAO;
use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};

use mockall::{automock, predicate::*};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Conversation {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub owner_id: ObjectId,
    pub members: Vec<ObjectId>,
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

#[automock]
#[async_trait]
impl BaseDAO<Conversation> for ConversationDAO {
    async fn insert_document(&self, document: &Conversation) -> Result<(), DataError> {
        println!("insert_document: {:?}", document);
        self.collection.insert_one(document, None).await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_insert_conversation() {
        let mut conversation_dao = MockConversationDAO::new();

        let conversation = Conversation {
            id: None,
            name: "Test Conversation".to_string(),
            owner_id: ObjectId::new(),
            members: vec![ObjectId::new(), ObjectId::new()],
            created_at: chrono::Utc::now(),
            updated_at: None,
        };

        conversation_dao
            .expect_insert_document()
            .with(eq(conversation.clone()))
            .returning(|_| Ok(()));

        let result = conversation_dao.insert_document(&conversation).await;
        assert!(result.is_ok());
    }
}
