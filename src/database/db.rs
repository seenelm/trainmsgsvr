use mongodb::{Client, options::ClientOptions};
use std::env;
use dotenv::dotenv;

pub struct Database {
    pub client: Client,
}

impl Database {
    pub async fn init(db_uri: &str) -> Result<Self, mongodb::error::Error> {
        let mut client_options = ClientOptions::parse(&db_uri).await?;
        client_options.app_name = Some("Train".to_string());
        let client = Client::with_options(client_options)?;
        
        Ok(Self {
            client,
        })
    }
}