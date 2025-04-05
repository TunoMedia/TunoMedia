use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
pub enum MusicCommands {
    /// Publish new song 
    Publish,

    /// Update song's metadata
    SetSong,

    /// Get song's metadata
    GetSong
}

impl MusicCommands {
    pub async fn execute(self) -> Result<()> {
        match self {
            MusicCommands::Publish {

            } => {
                todo!("publish command");

                Ok(())
            }

            MusicCommands::SetSong {

            } => {
                todo!("set-song command");

                Ok(())
            }

            MusicCommands::GetSong {

            } => {
                todo!("get-song command");

                Ok(())
            }
        }
    }
}