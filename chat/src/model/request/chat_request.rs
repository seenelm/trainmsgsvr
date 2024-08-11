use crate::error::ChatError;
use database::model::Message;
use database::model::{Conversation, User};

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRequest {
    pub id: ObjectId,
    pub name: String,
    pub username: String,
}

impl From<UserRequest> for User {
    fn from(user_request: UserRequest) -> Self {
        Self {
            id: user_request.id,
            name: user_request.name,
            username: user_request.username,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConversation {
    pub conversation_request: ConversationRequest,
    pub init_message_request: InitMessageRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationRequest {
    pub name: String,
    pub owner: UserRequest,
    pub members: Vec<UserRequest>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

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

impl TryFrom<MessageRequest> for Message {
    type Error = ChatError;

    fn try_from(req: MessageRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ObjectId::new(),
            sender: User::from(req.sender),
            conversation_id: req.conversation_id,
            text: req.text,
            created_at: req.created_at,
        })
    }
}

// Convert from ConversationRequest to Conversation
impl TryFrom<&ConversationRequest> for Conversation {
    type Error = ChatError;

    fn try_from(req: &ConversationRequest) -> Result<Self, Self::Error> {
        let members: Vec<User> = req
            .members
            .iter()
            .map(|user_request| User::from(user_request.clone()))
            .collect();

        Ok(Self {
            _id: ObjectId::new(),
            name: req.name.clone(),
            owner: User::from(req.owner.clone()),
            members,
            created_at: req.created_at,
            updated_at: None,
        })
    }
}
