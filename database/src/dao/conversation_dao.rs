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
    pub _id: ObjectId,
    pub name: Option<String>,
    pub owner_id: ObjectId,
    pub members: Vec<ObjectId>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationRequest {
    pub name: Option<String>,
    pub owner_id: ObjectId,
    pub members: Vec<ObjectId>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// Convert from ConversationRequest to Conversation
impl TryFrom<ConversationRequest> for Conversation {
    type Error = DataError;

    fn try_from(value: ConversationRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            name: value.name,
            owner_id: value.owner_id,
            members: value.members,
            created_at: value.created_at,
            updated_at: None,
        })
    }
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
    async fn insert_document(&self, document: &Conversation) -> Result<ObjectId, DataError> {
        println!("insert_document: {:?}", document);
        let result = self.collection.insert_one(document, None).await;
        match result {
            Ok(insert_result) => match insert_result.inserted_id.as_object_id() {
                Some(inserted_id) => Ok(inserted_id),
                None => Err(DataError::InsertFailed),
            },
            Err(e) => Err(DataError::Database(e)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_insert_conversation() {
        let mut conversation_dao = MockConversationDAO::new();
        let mock_id = ObjectId::new();

        let conversation = Conversation {
            _id: ObjectId::new(),
            name: Some("Test Conversation".to_string()),
            owner_id: ObjectId::new(),
            members: vec![ObjectId::new(), ObjectId::new()],
            created_at: chrono::Utc::now(),
            updated_at: None,
        };

        conversation_dao
            .expect_insert_document()
            .with(eq(conversation.clone()))
            .returning(move |_| Ok(mock_id.clone()));

        let result = conversation_dao.insert_document(&conversation).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), mock_id);
    }
}
