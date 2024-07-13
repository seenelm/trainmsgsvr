use crate::model::request::request::CreateConversation;
use crate::model::response::response::CreateConversationResponse;
use rust_socketio::{
    client::{Client, RawClient},
    ClientBuilder, Payload, Socket,
};

pub struct WSService {
    client: Client,
    builder: ClientBuilder,
}

impl WSService {
    pub fn new(url: &str, namespace: &str) -> Self {
        let builder = ClientBuilder::new(url).namespace(namespace);
        let client = builder.clone().connect().expect("Connection failed");

        Self { client, builder }
    }

    pub fn create_conversation(&self, create_conversation: CreateConversation) {
        // Add Error handling
        self.client
            .emit("create-chat", create_conversation)
            .unwrap();
    }

    pub fn handle_create_conversation_response(&self) -> CreateConversationResponse {
        let mut response: Option<CreateConversationResponse> = None;

        self.builder
            .on("create-chat", |payload, socket: RawClient| {
                response = serde_json::to_value(payload).unwrap();
            });
    }
}

// pub struct WSService {
//     client: Client,
// }

// impl WSService {
//     pub fn new(url: &str) -> Self {
//         // Add Error handling.
//         let client = ClientBuilder::new(url)
//             .namespace("/")
//             .connect()
//             .expect("Connection failed");

//         Self { client }
//     }

//     pub fn create_conversation(&self, create_conversation: CreateConversation) {
//         // Add Error handling
//         self.client
//             .emit("create-chat", create_conversation)
//             .unwrap();
//     }
// }
