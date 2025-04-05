use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use crate::server::TunoGrpcServer;

#[derive(Parser)]
pub enum DistributionCommands {
    /// Start distribution 
    Start {
        /// certificate directory with `fullchain.pem` and `privkey.pem` files to enable HTTPS connections
        #[arg(long)]
        cert_dir: Option<PathBuf>,
        /// IP to start RPC server on. (default: 127.0.0.1)
        #[arg(long, default_value = "127.0.0.1")]
        rpc_ip: String,
        /// Port to start RPC server on. (default: 4114)
        #[arg(long, default_value = "4114")]
        rpc_port: u16
    },

    /// Add song manually
    Add,

    /// Download a song from other distributor
    Download
}

impl DistributionCommands {
    pub async fn execute(self) -> Result<()> {
        match self {
            DistributionCommands::Start {
                cert_dir,
                rpc_ip,
                rpc_port
            } => {
                let server = TunoGrpcServer::new(
                    rpc_ip,
                    rpc_port,
                    cert_dir
                );
                server.run().await?;

                Ok(())
            }

            DistributionCommands::Add {

            } => {
                todo!("add command");

                Ok(())
            }

            DistributionCommands::Download {

            } => {
                todo!("download command");

                Ok(())
            }
        }
    }
}