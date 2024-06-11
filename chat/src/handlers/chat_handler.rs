use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use socketioxide::extract::{Data, SocketRef, TryData};
use std::sync::Arc;
use tracing::info;

use database::dao::conversation_dao::{Conversation, ConversationDAO};
use database::dao::message_dao::MessageDAO;
use database::dao::BaseDAO;

// Message received from the client
#[derive(Debug, Deserialize)]
pub struct Message {
    pub sender_id: ObjectId,
    pub conversation_id: ObjectId,
    pub text: String,
    pub media_url: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// Message sent to the client
#[derive(Debug, Serialize)]
pub struct MessageOut {
    text: String,
    user: String,                        // user who sent the message
    date: chrono::DateTime<chrono::Utc>, // Timestamp for when the message was received
}

#[derive(Clone)]
pub struct ChatHandler<'a> {
    conversation_dao: &'a ConversationDAO,
    message_dao: &'a MessageDAO,
    socket: &'a SocketRef,
}

impl<'a> ChatHandler<'a> {
    pub fn new(
        conversation_dao: &'a ConversationDAO,
        message_dao: &'a MessageDAO,
        socket: &'a SocketRef,
    ) -> Self {
        Self {
            conversation_dao,
            message_dao,
            socket,
        }
    }

    pub async fn handle_create_chat(&self, Data(data): Data<Conversation>) {
        info!("Received create-chat: {:?}", data);

        if let Err(e) = self.conversation_dao.insert_document(&data).await {
            println!("Failed to insert document: {}", e);
            return;
        }

        match self.socket.emit("create-chat", "Successfully created chat") {
            Ok(_) => info!("Successfully sent create-chat response"),
            Err(e) => println!("Failed to send create-chat response: {}", e),
        };
    }

    pub async fn handle_join(&self, socket: SocketRef, Data(room): Data<String>) {
        info!("Received join: {:?}", room);
        let _ = socket.leave_all(); // leave all rooms to ensure the socket is only in one room
        let _ = socket.join(room.clone()); // join the room
                                           // let _ = self.message_dao.find_messages_by_room(&room).await;
    }

    pub async fn handle_message(&self, Data(data): Data<Message>) {
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
