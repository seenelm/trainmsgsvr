use crate::error::ApiResult;
use crate::model::chat_model::{ConversationRequest, ConversationResponse};
use crate::service::chat_service::ChatService;
use axum::{http::StatusCode, response::Json};
use database::dao::conversation_dao::ConversationDAO;

pub struct ChatController {
    chat_service: ChatService<ConversationDAO>,
}

impl ChatController {
    pub fn new(chat_service: ChatService<ConversationDAO>) -> Self {
        Self { chat_service }
    }

    pub async fn create_conversation(
        &self,
        Json(req): Json<ConversationRequest>,
    ) -> ApiResult<(StatusCode, Json<ConversationResponse>)> {
        match self.chat_service.insert_one(&req).await {
            Ok(conversation) => Ok((StatusCode::CREATED, Json(conversation))),
            Err(err) => Err(err),
        }
    }
}
