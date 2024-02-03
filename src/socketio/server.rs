use std::sync::Arc;

use socketioxide::extract::{SocketRef, Data};
use tracing::info;
use train_messaging_server::{ChatDAO, BaseDAO, Chat};
use crate::controllers::chat_controller::ChatController;

// Message received from the client
#[derive(Debug, serde::Deserialize)]
pub struct MessageIn {
    room: String,
    text: String,
}

// Message sent to the client
#[derive(serde::Serialize)]
pub struct MessageOut {
    text: String,
    user: String, // user who sent the message
    date: chrono::DateTime<chrono::Utc>, // Timestamp for when the message was received
}

#[derive(Clone)]
pub struct ChatHandler {
    chat_dao: ChatDAO,
}

impl ChatHandler {
    pub fn new(chat_dao: ChatDAO) -> Self {
        Self { chat_dao }
    }

    pub async fn handle_create_chat(&self, socket: SocketRef, Data(data): Data<Chat>) {
        info!("Received create-chat: {:?}", data);

        if let Err(e) = self.chat_dao.insert_document(data).await {
            println!("Failed to insert document: {}", e);
            return;
        }

        let _ = socket.emit("create-chat", "Successfully created chat");
    }

    pub async fn handle_join(&self, socket: SocketRef, Data(room): Data<String>) {
        info!("Received join: {:?}", room);
        let _ = socket.leave_all(); // leave all rooms to ensure the socket is only in one room
        let _ = socket.join(room); // join the room
    }

    pub async fn handle_message(&self, socket: SocketRef, Data(data): Data<MessageIn>) {
        info!("Message received: {:?}", data);

        let response = MessageOut {
            text: data.text,
            user: format!("anon-{}", socket.id),
            date: chrono::Utc::now(),
        };

        // Send the message back to the room that it came from
        // Send the message to all sockets that joined that room
        let _ = socket.within(data.room).emit("message", response);
    }
}

pub struct Server {
    chat_controller: ChatController,
}

impl Server {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        // let chat_dao = ChatDAO::new().await?;
        // let chat_controller = ChatController::new(chat_dao);
        // Ok(Self { chat_controller })
        let chat_dao = ChatDAO::new().await?;
        let chat_handler = ChatHandler::new(chat_dao);
        let chat_controller = ChatController::new(chat_handler);
        Ok(Self { chat_controller })
    }

    pub async fn on_connect(&self, socket: SocketRef) {
        info!("Socket connected: {}", socket.id);
        self.chat_controller.register_chat_handlers(socket).await;
        
    }
}