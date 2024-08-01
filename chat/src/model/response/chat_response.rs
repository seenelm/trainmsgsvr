use database::dao::conversation_dao::{Conversation, User};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::serde_helpers::serialize_object_id_as_hex_string;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConversationResponse {
    pub conversation_response: ConversationResponse,
    pub message_response: MessageResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
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
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub name: String,
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub owner_id: ObjectId,
    pub owner_name: String,
    pub members: Vec<UserResponse>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub sender_id: ObjectId,
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
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
