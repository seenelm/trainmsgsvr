use std::sync::Arc;

use crate::socketio::server::{ChatHandler, MessageIn};
use train_messaging_server::{ChatDAO, Chat};

use socketioxide::extract::{SocketRef, Data};
pub struct ChatController {
    // chat_dao: ChatDAO,
    chat_handler: Arc<ChatHandler>,
}

impl ChatController {
    pub fn new(chat_handler: ChatHandler) -> Self {
        // let chat_handler = ChatHandler::new(chat_dao.clone());
        Self {
            chat_handler: Arc::new(chat_handler),
        }
    }

    pub async fn register_chat_handlers(&self, socket: SocketRef) {
        let chat_handler = Arc::clone(&self.chat_handler);
        
        socket.on("create-chat", move |socket: SocketRef, Data::<Chat>(data)| async move {
            let chat_handler = Arc::clone(&chat_handler);
            chat_handler.handle_create_chat(socket, Data(data)).await;
        });

        let chat_handler = Arc::clone(&self.chat_handler);
        socket.on("join", move |socket: SocketRef, Data::<String>(room)| {
            let chat_handler = Arc::clone(&chat_handler);
            chat_handler.handle_join(socket, Data(room));
        });

        let chat_handler = Arc::clone(&self.chat_handler);
        socket.on("message", move |socket: SocketRef, Data::<MessageIn>(data)| {
            let chat_handler = Arc::clone(&chat_handler);
            chat_handler.handle_message(socket, Data(data));
        });
    }
}

