use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use iota_sdk::types::base_types::ObjectID;

use crate::client::{Client, Connection, SongMetadata};

#[derive(Parser)]
pub enum MusicCommands {
    /// Register active address as creator
    Register {
        #[command(flatten)]
        conn: Connection
    },

    /// Publish new song 
    Publish {
        /// MP3 file containing the song
        #[arg(long)]
        file: PathBuf,

        /// Creator capability's object id (created at registration)
        #[arg(long, env = "CREATOR_CAP")]
        cap: ObjectID,

        #[command(flatten)]
        metadata: SongMetadata,
        #[command(flatten)]
        conn: Connection
    },

    /// Update song's metadata
    SetSong,

    /// Get song's metadata
    GetSong
}

impl MusicCommands {
    pub async fn execute(self) -> Result<()> {
        match self {
            MusicCommands::Register {
                conn
            } => {
                let client = Client::new(conn)?;
                let (
                    creator,
                    digest
                ) = client.register_creator().await?;

                println!("Creator succesfully registered on {}", digest);
                println!("CreatorCap: {}", creator.cap);
                println!("Kiosk: {}", creator.kiosk);
                println!("KioskOwnerCap: {}", creator.kiosk_cap);
                
                Ok(())
            }

            MusicCommands::Publish {
                file,
                cap,
                metadata,
                conn
            } => {
                let client = Client::new(conn)?;

                let (
                    song_id,
                    digest
                ) = client.create_song(cap, metadata).await?;

                println!("Song succesfully published on {}", digest);
                println!("ID: {}", song_id);
                println!("file: {}", file.display());
                
                Ok(())
            }

            MusicCommands::SetSong {

            } => {
                todo!("set-song command");
            }

            MusicCommands::GetSong {

            } => {
                todo!("get-song command");
            }
        }
    }
}
