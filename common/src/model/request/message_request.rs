use super::user_request::UserRequest;
use database::model::{Message, User};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitMessageRequest {
    pub sender: UserRequest,
    pub text: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageRequest {
    pub sender: UserRequest,
    pub conversation_id: ObjectId,
    pub text: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl MessageRequest {
    pub fn new(
        sender: UserRequest,
        conversation_id: ObjectId,
        text: String,
        created_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            sender,
            conversation_id,
            text,
            created_at,
        }
    }
}

impl From<MessageRequest> for Message {
    fn from(req: MessageRequest) -> Self {
        Self {
            id: ObjectId::new(),
            sender: User::from(req.sender),
            conversation_id: req.conversation_id,
            text: req.text,
            created_at: req.created_at,
        }
    }
}
