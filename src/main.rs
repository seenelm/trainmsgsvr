mod models;

use std::env;
use dotenv::dotenv;

use socketioxide::{
    extract::{Data, SocketRef}, 
    SocketIo
};
use axum::{routing::get};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

// use database::db;
// use database::db::Chat;

use train_messaging_server::{ChatDAO, BaseDAO, Chat};

// Message received from the client
#[derive(Debug, serde::Deserialize)]
struct MessageIn {
    room: String,
    text: String,
}

// Message sent to the client
#[derive(serde::Serialize)]
struct MessageOut {
    text: String,
    user: String, // user who sent the message
    date: chrono::DateTime<chrono::Utc>, // Timestamp for when the message was received
}

async fn on_connect(socket: SocketRef) {
    info!("Socket connected: {}", socket.id);
    let chat_dao = match ChatDAO::new().await {
        Ok(dao) => dao,
        Err(e) => {
            println!("Failed to create ChatDAO: {}", e);
            return;
        }
    };

    socket.on("create-chat", move |socket: SocketRef, Data::<Chat>(data)| async move {
        info!("Received create-chat: {:?}", data);
        

        if let Err(e) = chat_dao.create().await {
            println!("Failed to create chat: {}", e);
            return;
        }

        // let _ = chat_dao.insert_document(data).await;
        if let Err(e) = chat_dao.insert_document(data).await {
            println!("Failed to insert document: {}", e);
            return;
        }
      
        let _ = socket.emit("create-chat", "Successfully created chat");
    });


     // join the socket to the room
     socket.on("join", |socket: SocketRef, Data::<String>(room)| {
        info!("Received join: {:?}", room);
        let _ = socket.leave_all(); // leave all rooms to ensure the socket is only in one room
        let _ = socket.join(room); // join the room
    });

    socket.on("message", |socket: SocketRef, Data::<MessageIn>(data)| {
        info!("Message received: {:?}", data);

        let response = MessageOut {
            text: data.text,
            user: format!("anon-{}", socket.id),
            date: chrono::Utc::now(),
        };

        // Send the message back to the room that it came from
        // Send the message to all sockets that joined that room
        let _ = socket.within(data.room).emit("message", response);
    });
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    dotenv().ok();
    let db_uri = env::var("DB_URI").expect("DB_URI must be set");
    println!("DB_URI: {}", env::var("DB_URI").unwrap_or("Not set".to_string()));
    let db = train_messaging_server::init(&db_uri).await?;

    let (layer, io) = SocketIo::builder().with_state(db).build_layer();

    io.ns("/", on_connect);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer)
        );

    // axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await?;

    let listener = tokio::net::TcpListener::bind("192.168.1.59:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    info!("Server running on port 3000");

    Ok(())
}
