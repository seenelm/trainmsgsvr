// use database::dao::conversation_dao::{Conversation, User};
// use mongodb::bson::oid::ObjectId;
// use mongodb::bson::serde_helpers::serialize_object_id_as_hex_string;
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct UserResponse {
//     #[serde(serialize_with = "serialize_object_id_as_hex_string")]
//     pub id: ObjectId,
//     pub name: String,
// }

// impl From<User> for UserResponse {
//     fn from(user: User) -> Self {
//         Self {
//             id: user.id,
//             name: user.name,
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct LastMessage {
//     pub sender_id: ObjectId,
//     pub sender_name: String,
//     pub sender_user_id: ObjectId,
//     pub conversation_id: ObjectId,
//     pub text: String,
//     pub created_at: chrono::DateTime<chrono::Utc>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct ConversationResponse {
//     #[serde(serialize_with = "serialize_object_id_as_hex_string")]
//     pub id: ObjectId,
//     pub name: String,
//     #[serde(serialize_with = "serialize_object_id_as_hex_string")]
//     pub owner_id: ObjectId,
//     pub owner_name: String,
//     pub members: Vec<UserResponse>,
//     pub created_at: chrono::DateTime<chrono::Utc>,
//     pub updated_at: chrono::DateTime<chrono::Utc>,
// }

// impl From<Conversation> for ConversationResponse {
//     fn from(conversation: Conversation) -> Self {
//         let members: Vec<UserResponse> = conversation
//             .members
//             .iter()
//             .map(|user| UserResponse::from(user.clone()))
//             .collect();

//         Self {
//             id: conversation._id,
//             name: conversation.name.unwrap_or_default(),
//             owner_id: conversation.owner_id,
//             owner_name: conversation.owner_name,
//             members,
//             created_at: conversation.created_at,
//             updated_at: conversation.updated_at.unwrap_or_default(),
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct ConversationListResponse {
//     pub conversations: Vec<ConversationResponse>,
// }
