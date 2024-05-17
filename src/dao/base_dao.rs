// base_dao.rs
use async_trait::async_trait;
use mongodb::error::Error;

#[async_trait]
pub trait BaseDAO<T> {
    async fn create(&self) -> Result<(), Error>;
    async fn insert_document(&self, document: T) -> Result<(), Error>;
}
