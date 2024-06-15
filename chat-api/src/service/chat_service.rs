use database::dao::conversation_dao::{Conversation, ConversationDAO};
use database::dao::BaseDAO;
use database::DataError;
use mongodb::bson::oid::ObjectId;

pub struct ChatService {
    conversation_dao: ConversationDAO,
}

impl ChatService {
    pub fn new(conversation_dao: ConversationDAO) -> Self {
        Self { conversation_dao }
    }

    pub async fn insert_one(&self, data: Conversation) -> Result<ObjectId, DataError> {
        let id = self.conversation_dao.insert_document(&data).await?;
        Ok(id)
    }
}
