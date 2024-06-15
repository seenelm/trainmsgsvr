use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use database::dao::conversation_dao::Conversation;
use database::DataError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationRequest {
    pub name: Option<String>,
    pub owner_id: ObjectId,
    pub members: Vec<ObjectId>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationResponse {
    pub id: ObjectId,
    pub name: String,
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
