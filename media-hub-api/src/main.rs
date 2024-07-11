use database::DB;
use dotenv::dotenv;
use std::env;

use chat_api::router::create_router;

use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    tracing::subscriber::set_global_default(FmtSubscriber::default()).unwrap();

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

    let chat_router = create_router(&db);

    let test_uri = env::var("CHAT_API_URI").expect("TEST_URI must be set");
    let listener = tokio::net::TcpListener::bind(test_uri).await.unwrap();
    info!("Server running on port 3002");
    axum::serve(listener, chat_router).await.unwrap();
}
