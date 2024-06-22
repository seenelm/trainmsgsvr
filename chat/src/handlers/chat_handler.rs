use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use socketioxide::extract::{Data, SocketRef, TryData};
use socketioxide::handler::{FromMessageParts, MessageHandler};
use std::sync::Arc;
use tracing::info;

use database::dao::conversation_dao::ConversationDAO;
use database::dao::message_dao::MessageDAO;

use crate::error::ChatError;
use crate::model::chat_model::{
    ConversationRequest, CreateConversation, CreateConversationResponse, MessageRequest,
};
use crate::service::chat_service::ChatService;

use async_trait::async_trait;
use socketioxide::adapter::LocalAdapter;

// Message received from the client
// #[derive(Debug, Deserialize)]
// pub struct Message {
//     pub sender_id: ObjectId,
//     pub conversation_id: ObjectId,
//     pub text: String,
//     pub media_url: Option<String>,
//     pub created_at: chrono::DateTime<chrono::Utc>,
// }

// Message sent to the client
// #[derive(Debug, Serialize)]
// pub struct MessageOut {
//     text: String,
//     user: String,                        // user who sent the message
//     date: chrono::DateTime<chrono::Utc>, // Timestamp for when the message was received
// }

#[derive(Clone)]
pub struct ChatHandler {
    chat_service: Arc<ChatService>,
}

impl ChatHandler {
    pub fn new(chat_service: Arc<ChatService>) -> Self {
        Self { chat_service }
    }

    pub async fn handle_create_chat(
        &self,
        socket: SocketRef,
        Data(data): Data<CreateConversation>,
    ) {
        info!("Received create-chat: {:?}", data);
        // Insert new conversation into database
        let conversation_response = match self
            .chat_service
            .create_chat(data.conversation_request)
            .await
        {
            Ok(conversation) => conversation,
            Err(e) => {
                println!("Failed to create chat: {}", e);
                return;
            }
        };

        let message_request = MessageRequest {
            sender_id: data.message_request.sender_id,
            conversation_id: conversation_response.id,
            text: data.message_request.text,
            created_at: data.message_request.created_at,
        };

        // Insert new message into database
        let message_response = match self.chat_service.insert_message(message_request).await {
            Ok(message) => message,
            Err(e) => {
                println!("Failed to insert message: {}", e);
                return;
            }
        };

        let create_conversation_response = CreateConversationResponse {
            conversation_response,
            message_response,
        };

        match socket.emit("create-chat", create_conversation_response) {
            Ok(_) => info!("Successfully sent create-chat response"),
            Err(e) => println!("Failed to send create-chat response: {}", e),
        };
    }

    // pub async fn handle_join(&self, socket: SocketRef, Data(room): Data<String>) {
    //     info!("Received join: {:?}", room);
    //     let _ = socket.leave_all(); // leave all rooms to ensure the socket is only in one room
    //     let _ = socket.join(room.clone()); // join the room
    //                                        // let _ = self.message_dao.find_messages_by_room(&room).await;
    // }

    // pub async fn handle_message(&self, Data(data): Data<Message>) {
    //     info!("Message received: {:?}", data);

    //     let response = MessageOut {
    //         text: data.text.clone(),
    //         user: format!("anon-{}", socket.id),
    //         date: chrono::Utc::now(),
    //     };
    //     info!("Message response: {:?}", response);

    //     let message = Message {
    //         id: None,
    //         room: data.room.clone(),
    //         message: data.text,
    //     };

    //     if let Err(e) = self.message_dao.insert_document(message).await {
    //         println!("Failed to insert document: {}", e);
    //         return;
    //     }

    //     // Send the message back to the room that it came from
    //     // Send the message to all sockets that joined that room
    //     if let Err(e) = socket.within(data.room).emit("message", response) {
    //         println!("Failed to send message: {}", e);
    //         return;
    //     }
    // }
}

// #[async_trait]
// impl FromMessageParts<LocalAdapter> for ChatHandler {
//     async fn from_message_parts(&self, data: Data<CreateConversation>) -> Result<(), ChatError> {
//         self.handle_create_chat(data).await;
//         Ok(())
//     }
// }

// pub struct Server {
//     chat_handler: ChatHandler,
// }

// impl Server {
//     pub async fn new() -> Result<Self, mongodb::error::Error> {
//         let conversation_dao = ConversationDAO::new();
//         let message_dao = MessageDAO::new();
//         let chat_service = ChatService::new(conversation_dao, message_dao);
//         let socket = SocketRef::new();
//         let chat_handler = ChatHandler::new(chat_service, socket);
//         Ok(Self { chat_handler })
//     }
// }
