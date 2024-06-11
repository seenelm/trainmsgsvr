use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConversationResponse {
    pub id: ObjectId,
    pub name: Option<String>,
    pub owner_id: ObjectId,
    pub members: Vec<ObjectId>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
