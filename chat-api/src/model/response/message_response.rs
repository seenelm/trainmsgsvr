// use database::dao::message_dao::Message;
// use mongodb::bson::oid::ObjectId;
// use mongodb::bson::serde_helpers::serialize_object_id_as_hex_string;
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct MessageResponse {
//     #[serde(serialize_with = "serialize_object_id_as_hex_string")]
//     pub id: ObjectId,
//     #[serde(serialize_with = "serialize_object_id_as_hex_string")]
//     pub sender_id: ObjectId,
//     #[serde(serialize_with = "serialize_object_id_as_hex_string")]
//     pub conversation_id: ObjectId,
//     pub text: String,
//     pub created_at: chrono::DateTime<chrono::Utc>,
// }

// impl From<Message> for MessageResponse {
//     fn from(message: Message) -> Self {
//         Self {
//             id: message.id,
//             sender_id: message.sender_id,
//             conversation_id: message.conversation_id,
//             text: message.text,
//             created_at: message.created_at,
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct MessageListResponse {
//     pub messages: Vec<MessageResponse>,
// }
