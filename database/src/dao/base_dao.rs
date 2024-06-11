// base_dao.rs
use crate::DataError;
use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;

#[async_trait]
pub trait BaseDAO<T> {
    async fn insert_document(&self, document: &T) -> Result<ObjectId, DataError>;
}
