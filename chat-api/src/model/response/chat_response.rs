use database::dao::conversation_dao::Conversation;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationResponse {
    pub id: ObjectId,
    pub name: String,
    pub owner_id: ObjectId,
    pub members: Vec<ObjectId>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Conversation> for ConversationResponse {
    fn from(conversation: Conversation) -> Self {
        Self {
            id: conversation._id,
            name: conversation.name.unwrap_or_default(),
            owner_id: conversation.owner_id,
            members: conversation.members,
            created_at: conversation.created_at,
            updated_at: conversation.updated_at.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationListResponse {
    pub conversations: Vec<ConversationResponse>,
}
