use database::model::User;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRequest {
    pub id: ObjectId,
    pub name: String,
    pub username: String,
}

impl From<UserRequest> for User {
    fn from(user_request: UserRequest) -> Self {
        Self {
            id: user_request.id,
            name: user_request.name,
            username: user_request.username,
        }
    }
}
