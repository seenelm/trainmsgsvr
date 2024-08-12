use super::message_response::MessageResponse;
use super::user_response::UserResponse;
use database::model::Conversation;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::serde_helpers::serialize_object_id_as_hex_string;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConversationResponse {
    pub conversation_response: ConversationResponse,
    pub message_response: MessageResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationResponse {
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub name: String,
    pub owner: UserResponse,
    pub members: Vec<UserResponse>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<Conversation> for ConversationResponse {
    fn from(conversation: Conversation) -> Self {
        let members: Vec<UserResponse> = conversation
            .members
            .iter()
            .map(|user| UserResponse::from(user.clone()))
            .collect();

        Self {
            id: conversation._id,
            name: conversation.name,
            owner: UserResponse::from(conversation.owner.clone()),
            members,
            created_at: conversation.created_at,
            updated_at: Some(conversation.updated_at.unwrap_or_default()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationListResponse {
    pub conversations: Vec<ConversationResponse>,
}
