use crate::error::{ApiError, ApiResult};
use crate::model::chat_model::{ConversationRequest, ConversationResponse};
use database::dao::conversation_dao::Conversation;
use database::dao::BaseDAO;
use mongodb::bson::oid::ObjectId;

// use mockall::{automock, predicate::*};

pub struct ChatService<D>
where
    D: BaseDAO<Conversation> + Send + Sync,
{
    conversation_dao: D,
}

impl<D> ChatService<D>
where
    D: BaseDAO<Conversation> + Send + Sync,
{
    pub fn new(conversation_dao: D) -> Self {
        Self { conversation_dao }
    }

    pub async fn insert_one(&self, data: &ConversationRequest) -> ApiResult<ConversationResponse> {
        let conversation =
            Conversation::try_from(data).map_err(|err| ApiError::BadRequest(err.to_string()))?;

        let id = self.conversation_dao.insert_document(&conversation).await?;

        Ok(ConversationResponse {
            id,
            name: conversation.name.unwrap_or_default(),
            owner_id: conversation.owner_id,
            members: conversation.members,
            created_at: conversation.created_at,
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
