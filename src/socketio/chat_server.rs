use std::sync::Arc;

use crate::controllers::ChatController;
use crate::dao::{BaseDAO, Chat, ChatDAO, Message, MessageDAO};
use socketioxide::extract::{Data, SocketRef};
use tracing::info;

// Message received from the client
#[derive(Debug, serde::Deserialize)]
pub struct MessageIn {
    room: String,
    text: String,
}

// Message sent to the client
#[derive(Debug, serde::Serialize)]
pub struct MessageOut {
    text: String,
    user: String,                        // user who sent the message
    date: chrono::DateTime<chrono::Utc>, // Timestamp for when the message was received
}

#[derive(Clone)]
pub struct ChatHandler {
    chat_dao: ChatDAO,
    message_dao: MessageDAO,
}

impl ChatHandler {
    pub fn new(chat_dao: ChatDAO, message_dao: MessageDAO) -> Self {
        Self {
            chat_dao,
            message_dao,
        }
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
            text: data.text.clone(),
            user: format!("anon-{}", socket.id),
            date: chrono::Utc::now(),
        };
        info!("Message response: {:?}", response);

        let message = Message {
            id: None,
            room: data.room.clone(),
            message: data.text,
        };

        if let Err(e) = self.message_dao.insert_document(message).await {
            println!("Failed to insert document: {}", e);
            return;
        }

        // Send the message back to the room that it came from
        // Send the message to all sockets that joined that room
        if let Err(e) = socket.within(data.room).emit("message", response) {
            println!("Failed to send message: {}", e);
            return;
        }
    }
}

pub struct Server {
    chat_controller: ChatController,
}

impl Server {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        let chat_dao = ChatDAO::new().await?;
        let message_dao = MessageDAO::new().await?;
        let chat_handler = ChatHandler::new(chat_dao, message_dao);
        let chat_controller = ChatController::new(chat_handler);
        Ok(Self { chat_controller })
    }

    pub async fn on_connect(&self, socket: SocketRef) {
        info!("Socket connected: {}", socket.id);
        self.chat_controller.register_chat_handlers(socket).await;
    }
}
