use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;
use database::dao::conversation_dao::Conversation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationListResponse {
    pub conversations: Vec<Conversation>,
}
