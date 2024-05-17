use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    session_id: ObjectId,
    user_id: ObjectId,
    username: String,
    connected: bool,
}
