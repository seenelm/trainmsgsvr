use mongodb::bson::oid::ObjectId;
use socketioxide::extract::{Data, SocketRef, TryData};
use std::sync::Arc;
use tracing::info;

// use crate::model::chat_model::{
//     CreateConversation, CreateConversationResponse, MessageRequest, User,
// };

use crate::model::request::chat_request::{CreateConversation, MessageRequest, UserRequest};

use crate::model::response::chat_response::CreateConversationResponse;
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
        let owner_id = data.conversation_request.owner_id;
        let members: Vec<UserRequest> = data.conversation_request.members.clone();
        let owner_name = data.conversation_request.owner_name.clone();

        // Insert new conversation into database
        let mut conversation_response = match self
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

        if conversation_response.owner_id == owner_id {
            // change conversation name to recipient name
            conversation_response.name = members[0].name.clone();

            let create_conversation_response = CreateConversationResponse {
                conversation_response: conversation_response.clone(),
                message_response: message_response.clone(),
            };

            info!(" Owner Conversation Response: {:?}", conversation_response);

            match socket
                .to(owner_id.to_string())
                .emit("create-chat-response", create_conversation_response)
            {
                Ok(_) => info!("Successfully sent Owner create-chat response"),
                Err(e) => println!("Failed to send create-chat response: {}", e),
            };
        }

        if conversation_response.members[0].id == members[0].id {
            // change conversation name to owner name
            conversation_response.name = owner_name.clone();

            let create_conversation_response = CreateConversationResponse {
                conversation_response: conversation_response.clone(),
                message_response: message_response.clone(),
            };

            info!(
                " Recipient Conversation Response: {:?}",
                conversation_response
            );

            match socket
                .to(conversation_response.members[0].id.to_string())
                .emit("create-chat-response", create_conversation_response)
            {
                Ok(_) => info!("Successfully sent Recipient create-chat response"),
                Err(e) => println!("Failed to send create-chat response: {}", e),
            };
        }
    }

    pub async fn handle_join(&self, socket: SocketRef, Data(conversation_id): Data<ObjectId>) {
        // Check if user is in conversation before joining.

        let room = conversation_id.to_string();
        info!("Joining room: {}", room);
        let _ = socket.leave_all(); // leave all rooms to ensure the socket is only in one room
        let _ = socket.join(room); // join the room
    }

    pub async fn handle_message(&self, socket: SocketRef, message_request: MessageRequest) {
        // Insert new message into database
        let message_response = match self.chat_service.insert_message(message_request).await {
            Ok(message) => message,
            Err(e) => {
                println!("Failed to insert message: {}", e);
                return;
            }
        };

        // Send the message back to the room that it came from
        // Send the message to all sockets that joined that room
        if let Err(e) = socket
            .within(message_response.conversation_id.to_string())
            .emit("chat-message", message_response)
        {
            println!("Failed to send message: {}", e);
            return;
        }
    }
}
