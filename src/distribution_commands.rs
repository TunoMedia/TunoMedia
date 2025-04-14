use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use iota_sdk::types::base_types::ObjectID;
use tokio::{signal, sync::oneshot};
use tokio_stream::StreamExt;

use crate::{
    server::TunoGrpcServer,
    client::{Client, Connection},
    local_storage::{store_song_from_bytes, store_song_from_file}
};

pub mod pb {
    tonic::include_proto!("tuno");
}

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

    /// Remove the active address from distribution of a song
    Undistribute {
        /// Song's object id
        #[arg(long)]
        song: ObjectID,
        
        #[command(flatten)]
        conn: Connection
    },

    /// List of distributors of a song
    List {
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

            DistributionCommands::List {
                song,
                conn
            } => {
                let client = Client::new(conn)?;
                let distributors = client.get_distributors(song).await?;

                println!("{}", distributors);
                Ok(())
            }

            DistributionCommands::Add {
                file,
                song
            } => {
                // TODO: verify file's signature with on-chain metadata
                println!("location: {}", store_song_from_file(&file, &song.to_hex())?.display());

                Ok(())
            }

            DistributionCommands::Download {
                song,
                conn
            } => {
                let client = Client::new(conn)?;
                let distributors = client.get_distributors(song).await?;
                let (
                    address,
                    distributor
                ) = distributors.0.first_key_value().unwrap();

                let mut tuno_client = pb::tuno_client::TunoClient::connect(
                    distributor.url.clone()
                ).await?;

                let mut stream = tuno_client.stream_song(
                    pb::SongStreamRequest {
                        object_id: song.to_hex(),
                        block_size: 1024 * 1024
                    }
                ).await?
                .into_inner();

                let mut data = vec![];
                while let Some(item) = stream.next().await {
                    // TODO: verify file's signature with on-chain metadata
                    data.append(&mut item.unwrap().data);
                }
            
                println!("location: {}", store_song_from_bytes(data, &song.to_hex())?.display());

                Ok(())
            }
        }
    }
}
