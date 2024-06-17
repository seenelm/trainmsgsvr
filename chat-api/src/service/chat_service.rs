use crate::error::{ApiError, ApiResult};
use crate::model::chat_model::{ConversationRequest, ConversationResponse};
use database::dao::conversation_dao::{Conversation, ConversationDAO};
use database::dao::BaseDAO;

pub struct ChatService {
    conversation_dao: ConversationDAO,
}

impl ChatService {
    pub fn new(conversation_dao: ConversationDAO) -> Self {
        Self { conversation_dao }
    }

    pub async fn insert_one(&self, data: &ConversationRequest) -> ApiResult<ConversationResponse> {
        let conversation =
            Conversation::try_from(data).map_err(|err| ApiError::BadRequest(err.to_string()))?;

        let id = self.conversation_dao.insert_document(&conversation).await?;

        Ok(ConversationResponse {
            id,
            name: conversation.name.unwrap_or_default(),
            owner_id: conversation.owner_id,
            members: conversation.members,
            created_at: conversation.created_at,
        })
    }
}
