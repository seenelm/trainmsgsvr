use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct User {
    pub id: ObjectId,
    pub name: String,
    pub username: String,
}

// Make Conversation name a String.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Conversation {
    pub _id: ObjectId,
    pub name: String,
    pub owner: User,
    pub members: Vec<User>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
