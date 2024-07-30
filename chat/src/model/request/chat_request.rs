use crate::error::ChatError;
use database::dao::conversation_dao::{Conversation, User};
use database::dao::message_dao::Message;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRequest {
    pub id: ObjectId,
    pub name: String,
}

impl From<UserRequest> for User {
    fn from(user_request: UserRequest) -> Self {
        Self {
            id: user_request.id,
            name: user_request.name,
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
    pub name: Option<String>,
    pub owner_id: ObjectId,
    pub owner_name: String,
    pub members: Vec<UserRequest>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitMessageRequest {
    pub sender_id: ObjectId,
    pub text: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageRequest {
    pub sender_id: ObjectId,
    pub conversation_id: ObjectId,
    pub text: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl MessageRequest {
    pub fn new(
        sender_id: ObjectId,
        conversation_id: ObjectId,
        text: String,
        created_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            sender_id,
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
            sender_id: req.sender_id,
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

        // let member_ids: Vec<ObjectId> = req.members.iter().map(|user| user.id).collect();

        let members: Vec<User> = req
            .members
            .iter()
            .map(|user_request| User::from(user_request.clone()))
            .collect();

        Ok(Self {
            _id: ObjectId::new(),
            name: Some(conversation_name),
            owner_id: req.owner_id,
            owner_name: req.owner_name.clone(),
            members,
            created_at: req.created_at,
            updated_at: None,
        })
    }
}
