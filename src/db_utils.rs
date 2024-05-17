use mongodb::error::Result;
use mongodb::{options::ClientOptions, Client, Database};

pub async fn init(db_uri: &str) -> Result<Database> {
    let mut client_options = ClientOptions::parse(&db_uri).await?;
    client_options.app_name = Some("Train".to_string());
    let client = Client::with_options(client_options)?;
    let db = client.database("test");

    Ok(db)
}
