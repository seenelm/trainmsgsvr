use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;
use database::dao::conversation_dao::Conversation;

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
impl TryFrom<&ConversationRequest> for Conversation {
    type Error = ApiError;

    fn try_from(value: &ConversationRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            name: value.name.to_owned(),
            owner_id: value.owner_id,
            members: value.members.to_owned(),
            created_at: value.created_at,
            updated_at: None,
        })
    }
}
