use crate::error::ApiResult;
use crate::model::chat_model::{ConversationRequest, ConversationResponse};
use crate::service::chat_service::ChatService;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use database::dao::conversation_dao::Conversation;

pub struct ChatController {
    chat_service: ChatService,
}

impl ChatController {
    pub fn new(chat_service: ChatService) -> Self {
        Self { chat_service }
    }

    pub async fn create_conversation(
        &self,
        Json(req): Json<ConversationRequest>,
    ) -> ApiResult<(StatusCode, Json<ConversationResponse>)> {
        let conversation = Conversation::try_from(ConversationRequest {
            name: req.name.clone(),
            owner_id: req.owner_id.clone(),
            members: req.members.clone(),
            created_at: req.created_at.clone(),
        });

        match self.chat_service.insert_one(conversation).await {
            Ok(data) => (StatusCode::CREATED, Json(data)),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
