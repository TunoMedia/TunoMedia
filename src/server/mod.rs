use axum::routing::get;
use rmpv::Value;
use anyhow::Result;
use socketioxide::{
    extract::{AckSender, Data, SocketRef},
    SocketIo,
};

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
        let (layer, io) = SocketIo::new_layer();

        io.ns("/", on_connect);

        let app = axum::Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .layer(layer);

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
        
        Ok(())
    }
}

fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    socket.emit("auth", &data).ok();

    socket.on("message", |socket: SocketRef, Data::<Value>(data)| {
        socket.emit("message-back", &data).ok();
    });

    socket.on("message-with-ack", |Data::<Value>(data), ack: AckSender| {
        ack.send(&data).ok();
    });
}