use crate::model::response::chat_response::ConversationListResponse;
use crate::service::chat_service::ChatService;
use crate::{error::ApiResult, model::response::chat_response::MessageListResponse};
use axum::{extract::Path, http::StatusCode, response::Json};
use mongodb::bson::oid::ObjectId;

pub struct ChatController {
    chat_service: ChatService,
}

impl ChatController {
    pub fn new(chat_service: ChatService) -> Self {
        Self { chat_service }
    }

    pub async fn fetch_all_conversations(
        &self,
        Path(user_id): Path<ObjectId>,
    ) -> ApiResult<(StatusCode, Json<ConversationListResponse>)> {
        match self.chat_service.fetch_all(&user_id).await {
            Ok(conversations) => {
                println!("conversations: {:?}", conversations);
                Ok((StatusCode::ACCEPTED, Json(conversations)))
            }
            Err(err) => Err(err),
        }
    }

    pub async fn fetch_all_messages(
        &self,
        Path(conversation_id): Path<ObjectId>,
    ) -> ApiResult<(StatusCode, Json<MessageListResponse>)> {
        match self.chat_service.fetch_all_messages(&conversation_id).await {
            Ok(messages) => {
                println!("messages: {:?}", messages);
                Ok((StatusCode::ACCEPTED, Json(messages)))
            }
            Err(err) => Err(err),
        }
    }
}
