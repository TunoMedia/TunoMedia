use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use iota_sdk::types::base_types::ObjectID;
use tokio::{signal, sync::oneshot};

use crate::{client::{Client, Connection}, server::TunoGrpcServer};

#[derive(Parser)]
pub enum DistributionCommands {
    /// Start distribution 
    Start {
        /// Certificate directory with `fullchain.pem` and `privkey.pem` files to enable HTTPS connections
        #[arg(long)]
        cert_dir: Option<PathBuf>,
        /// IP to start RPC server on. (default: 127.0.0.1)
        #[arg(long, default_value = "127.0.0.1")]
        rpc_ip: String,
        /// Port to start RPC server on. (default: 4114)
        #[arg(long, default_value = "4114")]
        rpc_port: u16,
        
        #[command(flatten)]
        conn: Connection
    },

    Undistribute {
        /// Song's object id
        #[arg(long)]
        song: ObjectID,
        
        #[command(flatten)]
        conn: Connection
    },

    /// Add song manually
    Add {
        /// MP3 file containing the song
        #[arg(long)]
        file: PathBuf,
        
        /// Song's object id
        #[arg(long)]
        song: ObjectID,
        
        #[command(flatten)]
        conn: Connection
    },

    /// Download a song from other distributor
    Download {
        /// Song's object id
        #[arg(long)]
        song: ObjectID,
        
        #[command(flatten)]
        conn: Connection
    }
}

impl DistributionCommands {
    pub async fn execute(self) -> Result<()> {
        match self {
            DistributionCommands::Start {
                cert_dir,
                rpc_ip,
                rpc_port,
                conn
            } => {
                let server = TunoGrpcServer::new(
                    rpc_ip,
                    rpc_port,
                    cert_dir
                );

                let client = Client::new(conn)?;
                let distributing = client.distribute_all(
                    &server.get_url(),
                    100_000
                ).await?;

                println!("Distributing {} file(s)", distributing.len());

                let (shutdown_tx, shutdown_rx) = oneshot::channel();
                let shutdown_handle = tokio::spawn(async move {
                    signal::ctrl_c().await.expect("Failed to listen for ctrl+c signal");
                    println!("\nReceived shutdown signal, starting graceful shutdown...");
                    
                    match client.undistribute_all().await {
                        Ok(undistributed) => 
                            println!("Undistributed {}/{} song(s)", undistributed.len(), distributing.len()),
                        Err(e) => eprintln!("Error during undistribution: {:?}", e)
                    }
                    
                    let _ = shutdown_tx.send(());
                });
            
                server.run(Some(shutdown_rx)).await?;
                let _ = shutdown_handle.await;

                Ok(())
            }

            DistributionCommands::Undistribute {
                song,
                conn
            } => {
                let client = Client::new(conn)?;
                let digest = client.undistribute(song).await?;

                println!("Song ({}) is not longer being distributed [{}]", song, digest);
                Ok(())
            }

            DistributionCommands::Add {
                file,
                song,
                conn
            } => {
                todo!("add command");
            }

            DistributionCommands::Download {
                song,
                conn
            } => {
                todo!("download command");
            }
        }
    }
}
