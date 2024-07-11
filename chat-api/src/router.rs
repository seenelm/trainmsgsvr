use axum::{routing::get, Router};
use database::dao::conversation_dao::ConversationDAO;
use mongodb::Database;

use crate::controller::chat_controller::ChatController;
use crate::service::chat_service::ChatService;

use std::sync::Arc;

// pub fn create_router(chat_controller: Arc<ChatController>) -> Router {
//     // TODO: Add error handling
//     // let conversation_dao = ConversationDAO::new(&db).unwrap();
//     // let chat_service = ChatService::new(&conversation_dao);
//     // let chat_controller = ChatController::new(&chat_service);

//     Router::new().route(
//         "/chat/api/:user_id/conversations",
//         get({
//             let chat_controller = Arc::clone(&chat_controller);
//             |user_id| async move { chat_controller.fetch_all_conversations(user_id).await }
//         }),
//     )
// }

pub fn create_router(db: &Database) -> Router {
    // TODO: Add error handling
    let conversation_dao = ConversationDAO::new(&db).unwrap();
    let chat_service = ChatService::new(conversation_dao);
    let chat_controller = Arc::new(ChatController::new(chat_service));

    Router::new().route(
        "/chat/api/:user_id/conversations",
        get({
            let chat_controller = Arc::clone(&chat_controller);
            |user_id| async move { chat_controller.fetch_all_conversations(user_id).await }
        }),
    )
}
