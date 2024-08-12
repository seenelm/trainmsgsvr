use crate::error::ApiResult;
use crate::service::chat_service::ChatService;
use axum::{
    extract::{Json, Path},
    http::StatusCode,
};
use mongodb::bson::oid::ObjectId;

use common::conversation_request::FindConversationRequest;
use common::conversation_response::{ConversationListResponse, ConversationResponse};
use common::message_response::MessageListResponse;

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
        match self.chat_service.fetch_all_conversations(&user_id).await {
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

    pub async fn find_conversation(
        &self,
        Json(request): Json<FindConversationRequest>,
    ) -> ApiResult<(StatusCode, Json<ConversationResponse>)> {
        match self.chat_service.find_conversation(&request).await {
            Ok(conversation) => {
                println!("conversation: {:?}", conversation);
                Ok((StatusCode::ACCEPTED, Json(conversation)))
            }
            Err(err) => Err(err),
        }
    }
}
