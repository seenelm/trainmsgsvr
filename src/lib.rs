use async_trait::async_trait;
use mongodb::bson::doc;
use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;
use mongodb::{Collection, Database};
use mongodb::{Client, options::ClientOptions};
use std::env;


pub async fn init(db_uri: &str) -> mongodb::error::Result<Database> {
    let mut client_options = ClientOptions::parse(&db_uri).await?;
    client_options.app_name = Some("Train".to_string());
    let client = Client::with_options(client_options)?;
    let db = client.database("test");
    
    Ok(db)
}

#[async_trait]
pub trait BaseDAO<T> {
    async fn create(&self) -> Result<(), mongodb::error::Error>;
    async fn insert_document(&self, document: T) -> Result<(), mongodb::error::Error>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub chat_name: String,
    pub group_id: ObjectId,
    // pub participants: Vec<ObjectId>,
    // pub created_by: ObjectId,
    // pub created_at: chrono::DateTime<chrono::Utc>,
    // pub updated_at: chrono::DateTime<chrono::Utc>,
}
#[derive(Clone)]
pub struct ChatDAO {
    pub collection: Collection<Chat>,
    pub db: Database,
}

impl ChatDAO {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        let db_uri = env::var("DB_URI").expect("DB_URI must be set");
        let db = init(&db_uri).await?;
        let collection = db.collection("chat");
        Ok(Self { collection, db })
    }
}
        
#[async_trait]
impl BaseDAO<Chat> for ChatDAO {
    async fn create(&self) -> Result<(), mongodb::error::Error> {
        self.db.create_collection("chat", None).await?;
        Ok(())
    }

    async fn insert_document(&self, document: Chat) -> Result<(), mongodb::error::Error> {
        println!("insert_document: {:?}", document);
        let filter = doc! { "group_id": &document.group_id };
        let result = self.collection.find_one(filter, None).await?;
        println!("result: {:?}", result);
        if result.is_some() {
            println!("group_id already exists in the collection");
        } else {
            println!("inserting document");
            self.collection.insert_one(document, None).await?;
            println!("inserted document");
        }
        Ok(())
    }
    // async fn create() -> Result<(), mongodb::error::Error> {
    //     let db_uri = env::var("DB_URI").expect("DB_URI must be set");
    //     let db = init(&db_uri).await?;
    //     db.create_collection("chat", None).await?;
    //     Ok(())
    // }

    // async fn insert_document(document: Chat) -> Result<(), mongodb::error::Error> {
    //     let db_uri = env::var("DB_URI").expect("DB_URI must be set");
    //     let db = init(&db_uri).await?;
    //     let collection = db.collection("chat");

    //     let filter = doc! { "group_id": &document.group_id };
    //     let result = collection.find_one(filter, None).await?;
    //     if result.is_some() {
    //         println!("group_id already exists in the collection");
    //     } else {
    //         collection.insert_one(document, None).await?;
    //     }

    //     Ok(())
    // }
}