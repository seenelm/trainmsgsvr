use crate::model::ConversationResponse;
use database::dao::conversation_dao::{Conversation, ConversationDAO};
use database::dao::BaseDAO;
use database::DataError;

pub struct ChatService {
    conversation_dao: ConversationDAO,
}

impl ChatService {
    pub fn new(conversation_dao: ConversationDAO) -> Self {
        Self { conversation_dao }
    }

    pub async fn create_chat(&self, data: Conversation) -> Result<Conversation, DataError> {
        self.conversation_dao.insert_document(&data).await?;
        Ok(data)
    }
}
