use database::dao::conversation_dao::{Conversation, User};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConversationResponse {
    pub conversation_response: ConversationResponse,
    pub message_response: MessageResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: ObjectId,
    pub name: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationResponse {
    pub id: ObjectId,
    pub name: String,
    pub owner_id: ObjectId,
    pub owner_name: String,
    pub members: Vec<UserResponse>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    pub sender_id: ObjectId,
    pub conversation_id: ObjectId,
    pub text: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMessageResponse {
    pub sender_id: ObjectId,
    pub conversation_id: ObjectId,
    pub text: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
