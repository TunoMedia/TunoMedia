use anyhow::Result;
use clap::Parser;
use iota_sdk::types::base_types::ObjectID;

use crate::client::{Client, Connection};

#[derive(Parser)]
pub enum KioskCommands {
    /// List all songs available in kiosk
    List {
        /// Kiosk's object id
        #[arg(long, env = "KIOSK")]
        kiosk: ObjectID,

        #[command(flatten)]
        conn: Connection
    }
}

impl KioskCommands {
    pub async fn execute(self) -> Result<()> {
        match self {
            KioskCommands::List {
                kiosk,
                conn
            } => {
                let client = Client::new(conn)?;
                let songs = client.get_kiosk_songs(kiosk).await?;

                println!("{}", songs);
                
                Ok(())
            }
        }
    }
}
