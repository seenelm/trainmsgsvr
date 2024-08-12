use database::model::User;
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
    pub username: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            username: user.username,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationResponse {
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub name: String,
    pub owner: UserResponse,
    pub members: Vec<UserResponse>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    pub sender: UserResponse,
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub conversation_id: ObjectId,
    pub text: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
