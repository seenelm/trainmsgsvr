use axum::{routing::get, Router};
use database::dao::conversation_dao::ConversationDAO;
use database::dao::MessageDAO;
use mongodb::Database;

use crate::controller::chat_controller::ChatController;
use crate::service::chat_service::ChatService;

use std::sync::Arc;

pub fn create_router(db: &Database) -> Router {
    // TODO: Add error handling
    let conversation_dao = ConversationDAO::new(&db).unwrap();
    let message_dao = MessageDAO::new(&db).unwrap();
    let chat_service = ChatService::new(conversation_dao, message_dao);
    let chat_controller = Arc::new(ChatController::new(chat_service));

    Router::new()
        .route(
            "/chat/api/conversations/:user_id",
            get({
                let chat_controller = Arc::clone(&chat_controller);
                |user_id| async move { chat_controller.fetch_all_conversations(user_id).await }
            }),
        )
        .route(
            "/chat/api/messages/:conversation_id",
            get({
                let chat_controller = Arc::clone(&chat_controller);
                |conversation_id| async move {
                    chat_controller.fetch_all_messages(conversation_id).await
                }
            }),
        )
        .route(
            "/chat/api/conversation",
            get({
                let chat_controller = Arc::clone(&chat_controller);
                |find_conversation_request| async move {
                    chat_controller
                        .find_conversation(find_conversation_request)
                        .await
                }
            }),
        )
}
