use std::path::PathBuf;

use iota_sdk::{
    types::{
        Identifier,
        base_types::ObjectID,
        digests::TransactionDigest,
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        transaction::{Argument, ObjectArg, ProgrammableTransaction, TransactionData}
    },
    rpc_types::IotaTransactionBlockResponse,
    wallet_context::WalletContext
};
use anyhow::{bail, Result};
use clap::Parser;
use log::info;

use crate::utils::{
    extract_created_cap,
    extract_created_kiosk,
    extract_created_kiosk_cap,
    extract_created_song,
    execute_transaction
};

#[derive(Parser)]
pub struct Connection {
    /// The IOTA CLI config file, (default: ~/.iota/iota_config/client.yaml)
    #[arg(long)]
    config: Option<PathBuf>,

    /// Object ID of the game's package.
    #[arg(long, short, env = "PKG")]
    package_id: ObjectID,
}

#[derive(Parser)]
pub struct SongMetadata {
    #[arg(long)]
    title: String,
    /// Artist's name
    #[arg(long)]
    artist: String,
    /// Album name
    #[arg(long)]
    album: String,
    /// Release year
    #[arg(long)]
    release_year: u64,
    /// Genre
    #[arg(long)]
    genre: String,
    /// Url for cover art
    #[arg(long)]
    cover_art_url: String,
    /// Price for streaming entire song
    #[arg(long)]
    streaming_price: u64,
}

impl SongMetadata {
    pub(crate) fn as_arguments(
        &self,
        ptb: &mut ProgrammableTransactionBuilder
    ) -> Result<Vec<Argument>> {
        Ok(vec![
            ptb.pure(&self.title)?,
            ptb.pure(&self.artist)?,
            ptb.pure(&self.album)?,
            ptb.pure(&self.release_year)?,
            ptb.pure(&self.genre)?,
            ptb.pure(&self.cover_art_url)?,
            ptb.pure(&self.streaming_price)?,
        ])
    }
}

pub struct CreatorSetup {
    pub cap: ObjectID,
    pub kiosk: ObjectID,
    pub kiosk_cap: ObjectID
}

pub(crate) struct Client {
    wallet: WalletContext,
    package_id: ObjectID,
}

impl Client {
    pub(crate) fn new(conn: Connection) -> Result<Self> {
        let Some(config) = conn.config.or_else(|| {
            let mut default = dirs::home_dir()?;
            default.extend([".iota", "iota_config", "client.yaml"]);
            Some(default)
        }) else {
            bail!(
                "Cannot find wallet config. No config was supplied, and the default path \
                 (~/.iota/iota_config/client.yaml) does not exist.",
            );
        };

        let wallet = WalletContext::new(&config, None, None)?;
        Ok(Self {
            wallet,
            package_id: conn.package_id,
        })
    }

    pub(crate) async fn register_creator(
        &self
    ) -> Result<(CreatorSetup, TransactionDigest)> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        ptb.programmable_move_call(
            self.package_id,
            Identifier::new("tuno").unwrap(),
            Identifier::new("register_creator").unwrap(),
            vec![],
            vec![]
        );

        let resp = self.build_and_execute_transaction_data(
            ptb.finish()
        ).await?;

        Ok((
            CreatorSetup {
                cap: extract_created_cap(&resp, self.package_id)?,
                kiosk: extract_created_kiosk(&resp)?,
                kiosk_cap: extract_created_kiosk_cap(&resp)?
            },
            resp.digest
        ))
    }

    pub(crate) async fn create_song(
        &self,
        cap: ObjectID,
        metadata: SongMetadata
    ) -> Result<(ObjectID, TransactionDigest)> {
        let mut ptb = ProgrammableTransactionBuilder::new();
        let mut args = vec![
            ptb.obj(ObjectArg::ImmOrOwnedObject(
                self.wallet.get_object_ref(cap).await?
            ))?
        ];
        args.append(&mut metadata.as_arguments(&mut ptb)?);

        ptb.programmable_move_call(
            self.package_id,
            Identifier::new("tuno").unwrap(),
            Identifier::new("create_song").unwrap(),
            vec![],
            args
        );

        let resp = self.build_and_execute_transaction_data(
            ptb.finish()
        ).await?;

        Ok((
            extract_created_song(&resp, self.package_id)?,
            resp.digest
        ))
    }

    async fn build_and_execute_transaction_data(
        &self,
        pt: ProgrammableTransaction
    ) -> Result<IotaTransactionBlockResponse> {
        info!("building transaction: \n{}", pt.to_string());
        let sender = self.wallet.active_address()?;
        let tx_data = TransactionData::new_programmable(
            sender,
            self.wallet.get_all_gas_objects_owned_by_address(sender).await?,
            pt,
            10_000_000,
            self.wallet.get_reference_gas_price().await?,
        );

        info!("Signing {}...", tx_data.digest());
        let tx = self.wallet.sign_transaction(&tx_data);

        info!("Executing {}...", tx.digest());
        execute_transaction(&self.wallet, tx).await
    }
}
