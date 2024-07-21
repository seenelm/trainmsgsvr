use mongodb::bson::oid::ObjectId;
use socketioxide::extract::{Data, SocketRef};
use std::sync::Arc;
use tracing::info;

use crate::model::chat_model::{CreateConversation, CreateConversationResponse, MessageRequest};
use crate::service::chat_service::ChatService;

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

        let message_request = MessageRequest::new(
            data.init_message_request.sender_id,
            conversation_response.id,
            data.init_message_request.text,
            data.init_message_request.created_at,
        );

        // Insert new message into database
        let message_response = match self.chat_service.insert_message(message_request).await {
            Ok(message) => message,
            Err(e) => {
                println!("Failed to insert message: {}", e);
                return;
            }
        };

        let create_conversation_response = CreateConversationResponse {
            conversation_response: conversation_response.clone(),
            message_response,
        };

        // Check if recpients are connected before sending the message
        // Send message to all recipients

        match socket
            .within(conversation_response.members[0].to_string())
            .emit("create-chat-response", create_conversation_response)
        {
            Ok(_) => info!("Successfully sent create-chat response"),
            Err(e) => println!("Failed to send create-chat response: {}", e),
        };
    }

    pub async fn handle_join(&self, socket: SocketRef, Data(conversation_id): Data<ObjectId>) {
        let room = conversation_id.to_string();
        info!("Joining room: {}", room);
        let _ = socket.leave_all(); // leave all rooms to ensure the socket is only in one room
        let _ = socket.join(room); // join the room
    }

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
