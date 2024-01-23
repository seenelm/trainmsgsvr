use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    session_id: ObjectId,
    user_id: ObjectId,
    username: String,
    connected: bool,
}