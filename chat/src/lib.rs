pub mod controllers;
pub mod error;
pub mod handlers;
pub mod model;
pub mod service;
pub mod socketio;

// Re-export modules
pub use controllers::ChatController;
pub use socketio::{ChatHandler, MessageIn, MessageOut, Server};
