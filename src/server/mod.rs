use tonic::{transport::Server, Request, Response, Status};
use std::{net::SocketAddr, path::PathBuf};
use anyhow::Result;

mod utils;
use utils::{get_file, load_tls_config};

// Proto-generated code
pub mod tuno {
    tonic::include_proto!("tuno");
}

use tuno::{
    tuno_server::{Tuno, TunoServer as TunoGrpcServer},
    EchoRequest, EchoResponse,
    StreamRequest, StreamResponse,
};

pub struct TunoService {}

#[tonic::async_trait]
impl Tuno for TunoService {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let message = request.into_inner().message;
        println!("Received echo request: {:?}", message);
        
        if message.is_empty() {
            return Err(Status::invalid_argument("Invalid echo request: message is empty"));
        }
        
        let response = EchoResponse {
            message,
        };
        
        Ok(Response::new(response))
    }
    
    async fn stream(&self, request: Request<StreamRequest>) -> Result<Response<StreamResponse>, Status> {
        let object_id = request.into_inner().object_id;
        println!("Received stream request: {:?}", object_id);
        
        if object_id.is_empty() {
            return Err(Status::invalid_argument("Invalid stream request: object_id is empty"));
        }
        
        match get_file(&object_id) {
            Ok(data) => Ok(Response::new(StreamResponse { data })),
            Err(_) => Err(Status::not_found(format!("Invalid object_id: {}", object_id)))
        }
    }
}

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

    pub async fn run(&self) -> Result<()> {
        let addr = format!("{}:{}", self.host, self.port).parse::<SocketAddr>()?;
        let service = TunoService {};
        
        if let Some(TunoIdentity { cert_path, key_path }) = &self.identity {
            let tls_config = load_tls_config(cert_path, key_path)?;

            println!("Secure gRPC server listening on: https://{}", addr);
            Server::builder()
                .tls_config(tls_config)?
                .add_service(TunoGrpcServer::new(service))
                .serve(addr)
                .await?;
        } else {
            println!("gRPC server listening on: http://{}", addr);
            Server::builder()
                .add_service(TunoGrpcServer::new(service))
                .serve(addr)
                .await?;
        }
        
        Ok(())
    }
}
