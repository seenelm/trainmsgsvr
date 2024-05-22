// use mongodb::error::Result;
use mongodb::{options::ClientOptions, Client, Database};
use serde_json::error;
use thiserror::Error;

// pub async fn init(db_uri: &str) -> Result<Database> {
//     let mut client_options = ClientOptions::parse(&db_uri).await?;
//     client_options.app_name = Some("Train".to_string());
//     let client = Client::with_options(client_options)?;
//     let db = client.database("test");

//     Ok(db)
// }

#[derive(Debug, Error)]
pub enum DataError {
    #[error("Database error: {0}")]
    Database(#[from] mongodb::error::Error),
}

pub struct DB {
    client: Client,
}

impl DB {
    pub async fn new(connection_str: &str) -> Result<Self, DataError> {
        let mut client_options = ClientOptions::parse(&connection_str).await?;
        client_options.app_name = Some("Train".to_string());
        let client = Client::with_options(client_options)?;

        Ok(Self { client })
    }

    pub fn get_database(&self, db_name: &str) -> mongodb::Database {
        self.client.database(db_name)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_db_connection() {
        let connection_str = "mongodb://localhost:27017";
        let db = DB::new(connection_str).await;

        match db {
            Ok(db) => {
                let database = db.get_database("test");
                assert_eq!(database.name(), "test");
                println!("Successfully connected to db");
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
}
