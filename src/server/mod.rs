use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{protocol::Message, Utf8Bytes},
};

use anyhow::Result;
pub struct TunoServer {
    host: String,
    port: u16,
}


impl TunoServer {
    pub fn new(host: String, port: u16) -> Self {
        Self {
            host,
            port,
        }
    }

    pub async fn run(&self) -> Result<()> {
        let addr = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(&addr).await?;

        println!("WebSocket server listening on: {}", addr);

        while let Ok((stream, peer)) = listener.accept().await {
            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(stream, peer).await {
                    println!("Error processing connection: {}", e);
                }
            });
        }
        
        Ok(())
    }

    async fn handle_connection(
        stream: TcpStream,
        peer: SocketAddr,
    ) -> Result<()> {
        println!("New WebSocket connection from: {}", peer);
        
        let ws_stream = accept_async(stream).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        
        // Process incoming messages
        while let Some(msg) = ws_receiver.next().await {
            let msg = match msg {
                Ok(msg) => msg,
                Err(e) => {
                    println!("Error receiving message: {}", e);
                    break;
                }
            };
            
            match msg {
                Message::Text(text) => {
                    let res = Self::handle_message(text).to_string();
                    if let Err(e) = ws_sender.send(Message::Text(res.into())).await {
                        println!("Error sending response: {}", e);
                        break;
                    }
                },
                Message::Close(_) => {
                    println!("WebSocket connection closed by client: {}", peer);
                    break;
                },
                _ => ()
            }
        }
        
        println!("WebSocket connection closed: {}", peer);
        
        Ok(())
    }

    fn handle_message(text: Utf8Bytes) -> serde_json::Value {
        println!("Received text message: {}", text);
                    
        match serde_json::from_str::<serde_json::Value>(&text) {
            Ok(json) => Self::handle_request(json),
            Err(_) => serde_json::json!({
                "status": "error",
                "message": "Invalid JSON"
            })
        }
    }

    fn handle_request(json: serde_json::Value) -> serde_json::Value {
        match json.get("req").and_then(|a| a.as_str()) {
            Some(req) if req == "echo" => Self::handle_echo(json),
            _ => serde_json::json!({
                "status": "error",
                "message": "Unknown request"
            })
        }
    }

    fn handle_echo(json: serde_json::Value) -> serde_json::Value {
        match json.get("message").and_then(|a| a.as_str()) {
            Some(message) => serde_json::json!({
                "status": "success",
                "message": message
            }),
            None => serde_json::json!({
                "status": "error",
                "message": "Invalid echo request"
            })
        }
    }

}