use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use iota_sdk::types::base_types::ObjectID;

use crate::{client::{Client, Connection, OwnedKiosk, SongMetadata}, local_storage::store_song};

#[derive(Parser)]
pub enum MusicCommands {
    /// Register active address as creator
    Register {
        #[command(flatten)]
        conn: Connection
    },

    /// Publish new song and make it available
    Publish {
        /// MP3 file containing the song
        #[arg(long)]
        file: PathBuf,

        /// Creator capability's object id (created at registration)
        #[arg(long, env = "CREATOR_CAP")]
        cap: ObjectID,

        // fix: flatten makes arguments required. See: https://github.com/clap-rs/clap/issues/5092
        #[command(flatten)]
        owned_kiosk: Option<OwnedKiosk>,

        #[command(flatten)]
        metadata: SongMetadata,
        #[command(flatten)]
        conn: Connection
    },

    /// Make song available for distribution
    MakeAvailable {
        /// Song's object id
        #[arg(long)]
        song: ObjectID,

        #[command(flatten)]
        owned_kiosk: OwnedKiosk,

        #[command(flatten)]
        conn: Connection
    },

    /// Make song unavailable for distribution
    MakeUnavailable {
        /// Song's object id
        #[arg(long)]
        song: ObjectID,

        #[command(flatten)]
        owned_kiosk: OwnedKiosk,

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
                owned_kiosk,
                cap,
                metadata,
                conn
            } => {
                let client = Client::new(conn)?;

                let (
                    song,
                    digest
                ) = client.create_song(cap, metadata).await?;

                println!("Song succesfully published [{}]", digest);
                println!("ID: {}", song);

                if let Some(owned_kiosk) = owned_kiosk {
                    match client.make_song_available(song, owned_kiosk).await {
                        Ok(digest) => println!("Status: + [{}]", digest),
                        Err(e) => println!("Status: - [{e}]")
                    }
                } else {
                    println!("Status: -");
                }

                println!("location: {}", store_song(&file, &song.to_hex())?.display());
                
                Ok(())
            }

            MusicCommands::MakeAvailable {
                song,
                owned_kiosk,
                conn
            } => {
                let client = Client::new(conn)?;
                let digest = client.make_song_available(song, owned_kiosk).await?;

                println!("Song ({}) is now available [{}]", song, digest);
                Ok(())
            }

            MusicCommands::MakeUnavailable {
                song,
                owned_kiosk,
                conn
            } => {
                let client = Client::new(conn)?;
                let digest = client.make_song_unavailable(song, owned_kiosk).await?;

                println!("Song ({}) is now unavailable [{}]", song, digest);
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
