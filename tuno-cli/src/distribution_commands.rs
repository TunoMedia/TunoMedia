use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use iota_sdk::types::base_types::ObjectID;
use tokio::{signal, sync::oneshot};
use tokio_stream::StreamExt;

use crate::server::TunoGrpcServer;
use crate::client::{Client, Connection};
use crate::local_storage::{store_song_from_bytes, store_song_from_file};
use crate::constants::TUNO_BASE_CHUNK_SIZE;
use crate::types::TunoSignature;

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
                    cert_dir,
                    conn.clone()
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
                let distributors = client.get_song(song).await?.distributors;

                println!("{}", distributors);
                Ok(())
            }

            DistributionCommands::Add {
                file,
                song,
                conn
            } => {

                let client = Client::new(conn)?;
                let obj = client.get_song(song).await?;

                if TunoSignature::from(&file) != obj.signature {
                    panic!("File's signature cannot be verified succesfully");
                }

                println!("File's signature verified");
                println!("location: {}", store_song_from_file(&file, &song.to_hex())?.display());

                Ok(())
            }

            DistributionCommands::Download {
                song,
                conn
            } => {
                let client = Client::new(conn)?;
                let mut obj = client.get_song(song).await?;

                let (
                    address,
                    distributor
                ) = obj.distributors.0.first_key_value().unwrap();

                let tx = client.get_payment_transaction(song, address).await?;
                let mut channel = pb::tuno_client::TunoClient::connect(
                    distributor.url.clone()
                ).await?;

                let mut stream = channel.stream_song(
                    pb::SongStreamRequest {
                        req: Some(pb::SongRequest { raw_transaction: bcs::to_bytes(&tx)? }),
                        block_size: 4 * TUNO_BASE_CHUNK_SIZE as u32,
                    }
                ).await?
                .into_inner();

                let mut data: Vec<u8> = vec![];
                while let Some(item) = stream.next().await {
                    match obj.signature.consume_data(item.unwrap().data) {
                        Some(mut d) => data.append(&mut d),
                        None => panic!("Received data cannot be verified succesfully")
                    };
                }
            
                println!("File's signature verified");
                println!("location: {}", store_song_from_bytes(data, &song.to_hex())?.display());

                Ok(())
            }
        }
    }
}
