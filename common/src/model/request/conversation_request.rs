use super::message_request::InitMessageRequest;
use super::user_request::UserRequest;
use database::model::{Conversation, User};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConversationRequest {
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

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct FindConversationRequest {
//     pub sender: UserRequest,
//     pub members: Vec<UserRequest>,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindConversationRequest {
    pub sender: ObjectId,
    pub members: Vec<ObjectId>,
}

// Convert from ConversationRequest to Conversation
impl From<&ConversationRequest> for Conversation {
    fn from(req: &ConversationRequest) -> Self {
        let members: Vec<User> = req
            .members
            .iter()
            .map(|user_request| User::from(user_request.clone()))
            .collect();

        Self {
            _id: ObjectId::new(),
            name: req.name.clone(),
            owner: User::from(req.owner.clone()),
            members,
            created_at: req.created_at,
            updated_at: None,
        }
    }
}
