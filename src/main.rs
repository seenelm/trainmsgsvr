mod models;
mod socketio;
mod controllers;

use std::env;
use dotenv::dotenv;
use std::sync::Arc;

use socketioxide::SocketIo;
use axum::routing::get;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

use socketio::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    dotenv().ok();
    let db_uri = env::var("DB_URI").expect("DB_URI must be set");
    println!("DB_URI: {}", env::var("DB_URI").unwrap_or("Not set".to_string()));
    let db = train_messaging_server::init(&db_uri).await?;

    let (layer, io) = SocketIo::builder().with_state(db).build_layer();

    let server = Arc::new(Server::new().await?);

    io.ns("/", move |socket| async move {
        let server = Arc::clone(&server);
        server.on_connect(socket).await;
    });

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer)
        );

    let test_uri = env::var("TEST_URI").expect("TEST_URI must be set");
    let listener = tokio::net::TcpListener::bind(test_uri).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    info!("Server running on port 3000");

    Ok(())
}
