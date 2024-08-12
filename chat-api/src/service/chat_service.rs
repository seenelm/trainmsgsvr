use crate::error::{ApiError, ApiResult};
use database::dao::conversation_dao::ConversationDAO;
use database::dao::message_dao::MessageDAO;
use mongodb::bson::oid::ObjectId;

use common::conversation_request::FindConversationRequest;

use common::conversation_response::{ConversationListResponse, ConversationResponse};
use common::message_response::{MessageListResponse, MessageResponse};
use common::user_response::UserResponse;

// use mockall::{automock, predicate::*};

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

    pub async fn fetch_all_conversations(
        &self,
        user_id: &ObjectId,
    ) -> ApiResult<ConversationListResponse> {
        let conversations = self.conversation_dao.find_all(&user_id).await?;
        let conversation_response: Vec<ConversationResponse> = conversations
            .into_iter()
            .map(ConversationResponse::from)
            .collect();

        Ok(ConversationListResponse {
            conversations: conversation_response,
        })
    }

    pub async fn find_conversation(
        &self,
        find_conversation_request: &FindConversationRequest,
    ) -> ApiResult<ConversationResponse> {
        // let existing_conversation = match self
        //     .conversation_dao
        //     .find_one(
        //         &find_conversation_request.sender,
        //         &find_conversation_request.members,
        //     )
        //     .await
        // {
        //     Ok(Some(conversation)) => Some(conversation),
        //     Ok(None) => {
        //         return Err(ApiError::ResourceNotFound);
        //     }
        //     Err(e) => {
        //         println!("Error: {}", e);
        //         None
        //     }
        // };

        let existing_conversation = self
            .conversation_dao
            .find_one(
                &find_conversation_request.sender,
                &find_conversation_request.members,
            )
            .await?
            .unwrap();

        let members: Vec<UserResponse> = existing_conversation
            .members
            .iter()
            .map(|user| UserResponse::from(user.clone()))
            .collect();

        return Ok(ConversationResponse {
            id: existing_conversation._id,
            name: existing_conversation.name,
            owner: UserResponse::from(existing_conversation.owner.clone()),
            members,
            created_at: existing_conversation.created_at,
            updated_at: existing_conversation.updated_at,
        });
    }

    pub async fn fetch_all_messages(
        &self,
        conversation_id: &ObjectId,
    ) -> ApiResult<MessageListResponse> {
        let messages = self.message_dao.find_all(&conversation_id).await?;
        let message_response: Vec<MessageResponse> =
            messages.into_iter().map(MessageResponse::from).collect();

        Ok(MessageListResponse {
            messages: message_response,
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

    // #[tokio::test]
    // async fn test_insert_one_bad_request_error() {
    //     // Arrange
    //     let mut mock_conversation_dao = MockConversationDAO::new();

    //     let conversation_request = ConversationRequest {
    //         name: None,
    //         owner_id: ObjectId::new(),
    //         members: vec![ObjectId::new(), ObjectId::new()],
    //         created_at: chrono::Utc::now(),
    //     };

    //     let chat_service = ChatService::new(mock_conversation_dao);

    //     // Act
    //     let result = chat_service.insert_one(&conversation_request).await;

    //     // Assert
    //     assert!(result.is_err());
    //     let err = result.unwrap_err();
    // }
}
