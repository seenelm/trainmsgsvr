use crate::error::{ApiResult, ChatError};
use crate::model::chat_model::{
    ConversationRequest, ConversationResponse, MessageRequest, MessageResponse,
};
use database::dao::conversation_dao::{Conversation, ConversationDAO, IConversationDAO};
use database::dao::message_dao::{Message, MessageDAO};

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
        info!("Creat chat");

        let conversation = match Conversation::try_from(&data) {
            Ok(conversation) => conversation,
            Err(e) => {
                return Err(ChatError::BadRequest(format!(
                    "Failed to convert ConversationRequest to Conversation: {}",
                    e
                )));
            }
        };

        // Return ConflictError if conversation already exists.
        if let Ok(_) = self
            .conversation_dao
            .find_one(&data.owner_id, &data.name.unwrap_or_default())
            .await
        {
            println!("Conversation already exists");
            return Err(ChatError::ConflictError(
                "Conversation already exists".to_string(),
            ));
        }

        let id = self.conversation_dao.insert_document(&conversation).await?;

        Ok(ConversationResponse {
            id,
            name: conversation.name.unwrap_or_default(),
            owner_id: conversation.owner_id,
            members: conversation.members,
            created_at: conversation.created_at,
        })
    }

    pub async fn insert_message(
        &self,
        message_request: MessageRequest,
    ) -> ApiResult<MessageResponse> {
        let message = match Message::try_from(message_request) {
            Ok(message) => message,
            Err(e) => {
                return Err(ChatError::BadRequest(format!(
                    "Failed to convert MessageRequest to Message: {}",
                    e
                )));
            }
        };

        let id = self.message_dao.insert_document(&message).await?;
        info!("Inserted message with id: {:?}", id);

        Ok(MessageResponse { text: message.text })
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
