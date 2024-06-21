use crate::DataError;

use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};

use mockall::{automock, predicate::*};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Message {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub sender_id: ObjectId,
    pub conversation_id: ObjectId,
    pub text: String,
    // pub media_url: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct MessageDAO {
    pub collection: Collection<Message>,
}

impl MessageDAO {
    pub fn new(db: &Database) -> Result<Self, DataError> {
        let collection = db.collection("message");
        Ok(Self { collection })
    }

    pub async fn insert_document(&self, document: &Message) -> Result<(), DataError> {
        println!("insert_document: {:?}", document);
        self.collection.insert_one(document, None).await?;
        Ok(())
    }
}

// #[automock]
// #[async_trait]
// impl BaseDAO<Message> for MessageDAO {
//     async fn insert_document(&self, document: &Message) -> Result<(), DataError> {
//         println!("insert_document: {:?}", document);
//         self.collection.insert_one(document, None).await?;
//         Ok(())
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[tokio::test]
//     async fn test_insert_message() {
//         let mut message_dao = MockMessageDAO::new();

//         let message = Message {
//             id: None,
//             sender_id: ObjectId::new(),
//             conversation_id: ObjectId::new(),
//             text: "Hello".to_string(),
//             media_url: None,
//             created_at: chrono::Utc::now(),
//         };

//         message_dao
//             .expect_insert_document()
//             .with(eq(message.clone()))
//             .returning(|_| Ok(()));

//         let result = message_dao.insert_document(&message).await;
//         assert!(result.is_ok());
//     }
// }
