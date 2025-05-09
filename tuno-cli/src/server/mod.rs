use log::info;
use tokio::sync::oneshot;
use tonic::transport::Server;
use std::path::PathBuf;
use anyhow::Result;

mod tuno;
use tuno::pb::tuno_server::TunoServer;

use crate::client::{Client, Connection};

mod utils;

pub struct TunoGrpcServer {
    host: String,
    port: u16,
    identity: Option<TunoIdentity>,
    conn: Connection
}

#[derive(Clone)]
struct TunoIdentity {
    cert_path: PathBuf,
    key_path: PathBuf,
}

impl TunoGrpcServer {
    pub fn new(host: String, port: u16, cert_dir: Option<PathBuf>, conn: Connection) -> Self {
        Self {
            host,
            port,
            identity: cert_dir.and_then(|dir| 
                Some(TunoIdentity {
                    cert_path: dir.join("fullchain.pem"),
                    key_path: dir.join("privkey.pem")
                })
            ),
            conn
        }
    }

    pub fn get_url(&self) -> String {
        format!(
            "http{}://{}:{}",
            if self.identity.is_some() { "s" } else { "" },
            self.host,
            self.port
        )
    }

    pub async fn run(
        &self,
        shutdown: Option<oneshot::Receiver<()>>
    ) -> Result<()> {
        let addr = format!("{}:{}", self.host, self.port).parse()?;

        let mut server = Server::builder()
            .layer(tower_http::cors::CorsLayer::permissive());
        server = match &self.identity {
            Some(TunoIdentity { cert_path, key_path }) => {
                let tls_config = utils::load_tls_config(cert_path, key_path)?;
                info!("Secure gRPC server listening on: https://{}", addr);
                server.tls_config(tls_config)?
            },
            None => {
                info!("gRPC server listening on: http://{}", addr);
                server.accept_http1(true)
            }
        };

        let client = Client::new(self.conn.clone())?;
        let tuno_service = TunoServer::new(tuno::TunoService::new(client));
        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(tuno::pb::FILE_DESCRIPTOR_SET)
            .build_v1()?;

        let served = server
            .add_service(reflection_service)
            .add_service(tonic_web::enable(tuno_service))
            .serve(addr);

        if let Some(handle) = shutdown {
            let server_handle = tokio::spawn(served);
            tokio::select! {
                _ = handle => (),
                _ = server_handle => unreachable!("Server completed")
            };
        } else {
            served.await?;
        }
        
        Ok(())
    }
}
