use futures_util::{StreamExt, SinkExt};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_async,
    tungstenite::protocol::Message,
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
                    println!("Received text message: {}", text);
                    
                    // Echo the message
                    if let Err(e) = ws_sender.send(Message::Text(text)).await {
                        println!("Error sending response: {}", e);
                        break;
                    }
                }
                Message::Close(_) => {
                    println!("WebSocket connection closed by client: {}", peer);
                    break;
                }
                _ => {
                    // Ignore other message types
                }
            }
        }
        
        println!("WebSocket connection closed: {}", peer);
        
        Ok(())
    }

}