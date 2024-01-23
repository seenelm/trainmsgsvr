mod database;
mod models;

use socketioxide::{
    extract::{Data, SocketRef}, 
    SocketIo
};
use axum::routing::get;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

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

    let (layer, io) = SocketIo::new_layer();

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

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    info!("Server running on port 3000");

    Ok(())
}
