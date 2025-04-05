use std::path::PathBuf;
use anyhow::Result;
use clap::{CommandFactory, Parser};

use crate::{
    distribution_commands::DistributionCommands,
    music_commands::MusicCommands
};

#[derive(Parser)]
pub enum TunoCommands {
    /// Publish tuno package to IOTA
    Publish {
        /// The IOTA CLI config file, (default: ~/.iota/iota_config/client.yaml)
        #[arg(long)]
        config: Option<PathBuf>
    },

    /// Client for distributors
    Distribution {
        #[command(subcommand)]
        cmd: Option<DistributionCommands>
    },

    /// Client for musicians
    Music {
        #[command(subcommand)]
        cmd: Option<MusicCommands>
    },
}

impl TunoCommands {
    pub async fn execute(self) -> Result<()> {
        match self {
            TunoCommands::Publish {
                config
            } => {
                todo!("publish command");

                Ok(())
            }

            TunoCommands::Distribution {
                cmd
            } => {
                if let Some(cmd) = cmd {
                    cmd.execute().await?;
                } else {
                    let mut app = TunoCommands::command();
                    app.build();
                    app.find_subcommand_mut("distribution").unwrap().print_help()?;
                }

                Ok(())
            }

            TunoCommands::Music {
                cmd
            } => {
                if let Some(cmd) = cmd {
                    cmd.execute().await?;
                } else {
                    let mut app = TunoCommands::command();
                    app.build();
                    app.find_subcommand_mut("music").unwrap().print_help()?;
                }

                Ok(())
            }
        }
    }
}