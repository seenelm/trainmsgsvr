use database::dao::conversation_dao::{Conversation, ConversationDAO, IConversationDAO};
use database::DB;
use dotenv::dotenv;
use mongodb::bson::oid::ObjectId;
use std::env;

#[tokio::test]
async fn test_database() {
    dotenv().ok();
    let db_uri = env::var("DB_URI").expect("DB_URI must be set");
    println!(
        "DB_URI: {}",
        env::var("DB_URI").unwrap_or("Not set".to_string())
    );

    let db_client = match DB::new(&db_uri).await {
        Ok(client) => client,
        Err(e) => {
            // Add logging
            panic!("{:?}", e)
        }
    };

    let db = db_client.get_database("test");
    let conversation_dao = ConversationDAO::new(&db).unwrap();
    let conversation_name = "Test Conversation";

    let conversation = Conversation {
        _id: ObjectId::new(),
        name: Some("Test Conversation".to_string()),
        owner_id: ObjectId::new(),
        members: vec![ObjectId::new(), ObjectId::new()],
        created_at: chrono::Utc::now(),
        updated_at: None,
    };

    let conversation_id = conversation_dao
        .insert_document(&conversation)
        .await
        .unwrap();

    let found_conversation = conversation_dao
        .find_one(&conversation_id, &conversation_name)
        .await
        .unwrap();

    assert_eq!(found_conversation, conversation);
}
