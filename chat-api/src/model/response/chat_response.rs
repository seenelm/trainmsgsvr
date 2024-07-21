use database::dao::conversation_dao::Conversation;
use database::utils::serialize_object_id_vec_as_hex_string;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::serde_helpers::serialize_object_id_as_hex_string;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationResponse {
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub name: String,
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub owner_id: ObjectId,
    #[serde(serialize_with = "serialize_object_id_vec_as_hex_string")]
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
