// use chat::{
//     handlers::chat_handler,
//     model::chat_model::{CreateConversation, MessageRequest},
// };
use chat::{
    handlers::chat_handler,
    model::request::chat_request::{CreateConversation, MessageRequest},
};
use dotenv::dotenv;
use mongodb::{bson::oid::ObjectId, Database};
use std::env;
use std::sync::Arc;

use axum::routing::get;
use socketioxide::{
    extract::{Data, SocketRef, TryData},
    SocketIo,
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

use chat::handlers::chat_handler::ChatHandler;
use chat::service::chat_service::ChatService;
use database::dao::conversation_dao::ConversationDAO;
use database::dao::message_dao::MessageDAO;
use database::DataError;
use database::DB;

fn init_services(db: &Database) -> Result<ChatService, DataError> {
    let conversation_dao = ConversationDAO::new(db)?;
    let message_dao = MessageDAO::new(db)?;
    let chat_service = ChatService::new(conversation_dao, message_dao);
    Ok(chat_service)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

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
    let conversation_dao = ConversationDAO::new(&db)?;
    let message_dao = MessageDAO::new(&db)?;
    let chat_service = Arc::new(ChatService::new(conversation_dao, message_dao));

    let (layer, io) = SocketIo::builder().with_state(db).build_layer();

    io.ns("/", move |socket: SocketRef| {
        info!("Socket connected: {:?}", socket.id);
        let chat_handler = ChatHandler::new(chat_service.clone());

        socket.on(
            "join",
            |socket: SocketRef, Data(user_id): Data<ObjectId>| {
                info!("Received join");
                let room = user_id.to_string();
                _ = socket.join(room);
            },
        );

        socket.on("create-chat", {
            let chat_handler = chat_handler.clone();
            move |socket: SocketRef, data: Data<CreateConversation>| async move {
                info!("Received create-chat");
                chat_handler.handle_create_chat(socket, data).await;
            }
        });

        socket.on("join-chat", {
            // println!("Join chat");
            // let chat_handler = chat_handler.clone();
            |socket: SocketRef, Data(conversation_id): Data<ObjectId>| {
                let room = conversation_id.to_string();
                info!("Joining room: {}", room);
                let _ = socket.leave_all();
                let _ = socket.join(room);
                // chat_handler.handle_join(socket, data).await;
            }
        });

        socket.on("new-message", {
            info!("New Message");
            let chat_handler = chat_handler.clone();
            move |socket: SocketRef, data: TryData<MessageRequest>| async move {
                info!("Received New Message");
                match data.0 {
                    Ok(message_request) => {
                        chat_handler.handle_message(socket, message_request).await;
                    }
                    Err(e) => {
                        println!("Failed to parse message request: {}", e);
                    }
                }
                // chat_handler.handle_message(socket, data).await;
            }
        });
    });

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        );

    let test_uri = env::var("TEST_URI").expect("TEST_URI must be set");
    let listener = tokio::net::TcpListener::bind(test_uri).await.unwrap();
    info!("Server running on port 3000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
