use mongodb::bson::oid::ObjectId;
use rust_socketio::Payload;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConversation {
    pub conversation_request: ConversationRequest,
    pub init_message_request: InitMessageRequest,
}

impl Into<Payload> for CreateConversation {
    fn into(self) -> Payload {
        // Add Error handling
        let json = serde_json::to_value(&self).unwrap();
        Payload::Text(vec![json])
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationRequest {
    pub name: Option<String>,
    pub owner_id: ObjectId,
    pub members: Vec<User>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitMessageRequest {
    pub sender_id: ObjectId,
    pub text: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: ObjectId,
    pub name: String,
}
