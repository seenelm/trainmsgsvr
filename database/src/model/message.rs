use super::conversation::User;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Message {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub sender: User,
    pub conversation_id: ObjectId,
    pub text: String,
    // pub media_url: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
