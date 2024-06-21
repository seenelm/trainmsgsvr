use crate::error::ChatError;
use database::dao::conversation_dao::Conversation;
use database::dao::message_dao::Message;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConversation {
    pub conversation_request: ConversationRequest,
    pub message_request: MessageRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConversationResponse {
    pub conversation_response: ConversationResponse,
    pub message_response: MessageResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: ObjectId,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationRequest {
    pub name: Option<String>,
    pub owner_id: ObjectId,
    pub members: Vec<User>,
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
impl TryFrom<ConversationRequest> for Conversation {
    type Error = ChatError;

    fn try_from(req: ConversationRequest) -> Result<Self, Self::Error> {
        let conversation_name = match &req.name {
            Some(name) => name.to_owned(),
            None => {
                let member_names: Vec<String> = req
                    .members
                    .iter()
                    .map(|user| user.name.to_owned())
                    .collect();
                member_names.join(", ")
            }
        };

        let member_ids: Vec<ObjectId> = req.members.iter().map(|user| user.id).collect();

        Ok(Self {
            _id: ObjectId::new(),
            name: Some(conversation_name),
            owner_id: req.owner_id,
            members: member_ids,
            created_at: req.created_at,
            updated_at: None,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageRequest {
    pub sender_id: ObjectId,
    pub conversation_id: ObjectId,
    pub text: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    pub text: String,
}

impl TryFrom<MessageRequest> for Message {
    type Error = ChatError;

    fn try_from(req: MessageRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ObjectId::new(),
            sender_id: req.sender_id,
            conversation_id: req.conversation_id,
            text: req.text,
            created_at: req.created_at,
        })
    }
}
