use futures_util::{SinkExt, StreamExt};
use std::{net::SocketAddr, path::PathBuf};
use native_tls::Identity;
use anyhow::Result;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::{TcpListener, TcpStream}
};
use tokio_tungstenite::{
    accept_async,
    WebSocketStream,
    tungstenite::{Message, Utf8Bytes},
};

use utils::{
    error_response,
    get_file,
    success_response
};

mod utils;

pub struct TunoServer {
    host: String,
    port: u16,
    identity: Option<TunoIdentity>
}

#[derive(Clone)]
struct TunoIdentity {
    cert_path: PathBuf,
    key_path: PathBuf,
}

impl TunoServer {
    pub fn new(host: String, port: u16, cert_dir: Option<PathBuf>) -> Self {
        Self {
            host,
            port,
            identity: cert_dir.and_then(|dir| 
                Some(TunoIdentity {
                    cert_path: dir.join("fullchain.pem"),
                    key_path: dir.join("privkey.pem")
                })
            )
        }
    }

    fn create_tls_acceptor(cert_path: &PathBuf, key_path: &PathBuf) -> Result<tokio_native_tls::TlsAcceptor> {
        if !cert_path.exists() {
            return Err(anyhow::anyhow!("Certificate file not found: {:?}", cert_path));
        }

        if !key_path.exists() {
            return Err(anyhow::anyhow!("Certificate file not found: {:?}", key_path));
        }

        let cert = Identity::from_pkcs8(
            &std::fs::read(&cert_path)?,
            &std::fs::read(&key_path)?
        )?;

        Ok(
            tokio_native_tls::TlsAcceptor::from(
                native_tls::TlsAcceptor::builder(cert).build()?
            )
        )
    }

    pub async fn run(&self) -> Result<()> {
        let addr = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(&addr).await?;

        let tls_acceptor = self.identity.as_ref().and_then(
            |TunoIdentity { cert_path, key_path }|
                Self::create_tls_acceptor(cert_path, key_path).ok()
        );

        match tls_acceptor {
            Some(_) => println!("Secure WebSocket server listening on: wss://{}", addr),
            None => println!("WebSocket server listening on: ws://{}", addr)
        }

        while let Ok((stream, peer)) = listener.accept().await {
            let tls_acceptor = tls_acceptor.clone();
            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(stream, peer, tls_acceptor).await {
                    println!("Error processing connection: {}", e);
                }
            });
        }
        
        Ok(())
    }

    async fn handle_connection(
        stream: TcpStream,
        peer: SocketAddr,
        tls_acceptor: Option<tokio_native_tls::TlsAcceptor>
    ) -> Result<()> {
        match tls_acceptor {
            Some(acceptor) => {
                let tls_stream = acceptor.accept(stream).await?;
                let ws_stream = accept_async(tls_stream).await?;
                
                println!("Secure connection established with: {}", peer);
                Self::process_websocket(ws_stream, peer).await
            },
            None => {
                let ws_stream = accept_async(stream).await?;
                
                println!("Standard connection established with: {}", peer);
                Self::process_websocket(ws_stream, peer).await
            }
        }
    }

    async fn process_websocket<S>(ws_stream: WebSocketStream<S>, peer: SocketAddr) -> Result<()>
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        
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
                    if let Err(e) = ws_sender.send(Self::handle_message(text)).await {
                        println!("Error sending response: {}", e);
                        break;
                    }
                },
                Message::Close(_) => {
                    println!("Connection closed by client: {}", peer);
                    break;
                },
                _ => ()
            }
        }
        
        println!("Connection closed: {}", peer);
        
        Ok(())
    }
    
    fn handle_message(text: Utf8Bytes) -> Message {
        println!("Received text message: {}", text);
                    
        match serde_json::from_str::<serde_json::Value>(&text) {
            Ok(json) => Self::handle_request(json),
            Err(_) => error_response("Invalid JSON")
        }
    }

    fn handle_request(json: serde_json::Value) -> Message {
        match json.get("req").and_then(|a| a.as_str()) {
            Some("echo") => Self::handle_echo(json),
            Some("stream") => Self::handle_stream(json),
            _ => error_response("Unknown request")
        }
    }

    fn handle_echo(json: serde_json::Value) -> Message {
        match json.get("message").and_then(|a| a.as_str()) {
            Some(message) => success_response(message),
            None => error_response("Invalid echo request")
        }
    }

    fn handle_stream(json: serde_json::Value) -> Message {
        match json.get("object_id").and_then(|a| a.as_str()) {
            Some(object_id) => {
                match get_file(object_id) {
                    Ok(bin) => Message::Binary(bin.into()),
                    Err(_) => error_response("Invalid object_id")
                }
            }
            None => error_response("Invalid stream request")
        }
    }
}
