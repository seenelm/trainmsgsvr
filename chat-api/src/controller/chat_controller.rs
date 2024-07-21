use crate::error::ApiResult;
use crate::model::response::chat_response::ConversationListResponse;
use crate::service::chat_service::ChatService;
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
}
