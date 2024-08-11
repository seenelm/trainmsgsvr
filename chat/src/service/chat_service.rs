use crate::error::{ApiResult, ChatError};

use database::dao::conversation_dao::ConversationDAO;
use database::dao::message_dao::MessageDAO;
use database::model::Conversation;
use database::model::Message;

use common::conversation_request::ConversationRequest;
use common::message_request::MessageRequest;

use common::conversation_response::ConversationResponse;
use common::message_response::MessageResponse;
use common::user_response::UserResponse;

use tracing::info;

pub struct ChatService {
    conversation_dao: ConversationDAO,
    message_dao: MessageDAO,
}

impl ChatService {
    pub fn new(conversation_dao: ConversationDAO, message_dao: MessageDAO) -> Self {
        Self {
            conversation_dao,
            message_dao,
        }
    }

    pub async fn create_chat(&self, data: ConversationRequest) -> ApiResult<ConversationResponse> {
        let mut conversation = Conversation::from(&data);

        // Check if conversation is not a group conversation.
        if conversation.members.len() == 1 {
            conversation.name = "Peer to Peer".to_string();
        }

        // If conersation already exists return existing conversation.
        // if let Ok(Some(existing_conversation)) =
        //     self.conversation_dao.find_one(&data.owner.id).await
        // {
        //     println!("Conversation already exists");
        //     let members: Vec<UserResponse> = existing_conversation
        //         .members
        //         .iter()
        //         .map(|user| UserResponse::from(user.clone()))
        //         .collect();

        //     return Ok(ConversationResponse {
        //         id: existing_conversation._id,
        //         name: existing_conversation.name,
        //         owner_id: existing_conversation.owner.id,
        //         owner_name: existing_conversation.owner.name,
        //         members,
        //         created_at: existing_conversation.created_at,
        //     });
        // }

        // Conversation doesn't exist
        let id = self.conversation_dao.insert_document(&conversation).await?;

        let members: Vec<UserResponse> = conversation
            .members
            .iter()
            .map(|user| UserResponse::from(user.clone()))
            .collect();

        Ok(ConversationResponse {
            id,
            name: conversation.name,
            owner: UserResponse::from(conversation.owner),
            members,
            created_at: conversation.created_at,
            updated_at: None,
        })
    }

    pub async fn insert_message(
        &self,
        message_request: MessageRequest,
    ) -> ApiResult<MessageResponse> {
        let message = Message::from(message_request);

        let id = self.message_dao.insert_document(&message).await?;
        info!("Inserted message with id: {:?}", id);

        Ok(MessageResponse {
            sender: UserResponse::from(message.sender),
            conversation_id: message.conversation_id,
            text: message.text,
            created_at: message.created_at,
        })
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use database::dao::conversation_dao::MockConversationDAO;
    // use mongodb::bson::oid::ObjectId;

    // #[tokio::test]
    // async fn test_insert_one() {
    //     // Arrange
    //     let mut mock_conversation_dao = MockConversationDAO::new();
    //     let expected_conversation_id = ObjectId::new();

    //     let conversation_request = ConversationRequest {
    //         name: Some("Test Conversation".to_string()),
    //         owner_id: ObjectId::new(),
    //         members: vec![ObjectId::new(), ObjectId::new()],
    //         created_at: chrono::Utc::now(),
    //     };

    //     mock_conversation_dao
    //         .expect_insert_document()
    //         .withf(move |doc: &Conversation| doc.name == Some("Test Conversation".to_string()))
    //         .returning(move |_| Ok(expected_conversation_id));

    //     let chat_service = ChatService::new(mock_conversation_dao);

    //     // Act
    //     let result = chat_service.insert_one(&conversation_request).await;

    //     // Assert
    //     assert!(result.is_ok());
    //     let conversation = result.unwrap();
    //     assert_eq!(conversation.id, expected_conversation_id);
    //     assert_eq!(conversation.name, "Test Conversation");
    //     assert_eq!(conversation.owner_id, conversation_request.owner_id);
    //     assert_eq!(conversation.members, conversation_request.members);
    //     assert_eq!(conversation.created_at, conversation_request.created_at);
    // }
}
