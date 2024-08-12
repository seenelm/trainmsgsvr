use super::user_response::UserResponse;
use database::model::Message;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::serde_helpers::serialize_object_id_as_hex_string;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    pub sender: UserResponse,
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub conversation_id: ObjectId,
    pub text: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<Message> for MessageResponse {
    fn from(message: Message) -> Self {
        Self {
            sender: UserResponse::from(message.sender),
            conversation_id: message.conversation_id,
            text: message.text,
            created_at: message.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageListResponse {
    pub messages: Vec<MessageResponse>,
}
