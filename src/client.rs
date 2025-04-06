use std::path::PathBuf;

use iota_sdk::{
    rpc_types::{IotaExecutionStatus, IotaTransactionBlockEffectsAPI, IotaTransactionBlockResponse, ObjectChange},
    types::{
        base_types::{ObjectID, ObjectRef},
        digests::TransactionDigest,
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        transaction::{Argument, ObjectArg, ProgrammableTransaction, Transaction, TransactionData},
        Identifier
    },
    wallet_context::WalletContext, IotaClient
};
use anyhow::{bail, Context, Result};
use clap::Parser;

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
    package: ObjectID,
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
            package: conn.package_id,
        })
    }

    pub(crate) async fn register_creator(
        &self
    ) -> Result<(CreatorSetup, TransactionDigest)> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        ptb.programmable_move_call(
            self.package,
            Identifier::new("tuno").unwrap(),
            Identifier::new("create_song").unwrap(),
            vec![],
            vec![]
        );

        let resp = self.execute_transaction(
            self.wallet.sign_transaction(
                &self.build_transaction_data(
                    ptb.finish()
                ).await?
            )
        ).await?;

        Ok((
            CreatorSetup {
                cap: self.extract_created_object(&resp, "CreatorCap")?,
                // fix: input `iota::kiosk` as type instead of self.package
                kiosk: self.extract_created_object(&resp, "Kiosk")?,
                kiosk_cap: self.extract_created_object(&resp, "KioskOwnerCap")?
            },
            resp.digest
        ))
    }

    pub(crate) async fn create_song(
        &self,
        cap_ref: ObjectRef,
        metadata: SongMetadata
    ) -> Result<(ObjectID, TransactionDigest)> {
        let mut ptb = ProgrammableTransactionBuilder::new();
        let mut args = metadata.as_arguments(&mut ptb)?;
        args.push(ptb.obj(ObjectArg::ImmOrOwnedObject(cap_ref))?);

        ptb.programmable_move_call(
            self.package,
            Identifier::new("tuno").unwrap(),
            Identifier::new("create_song").unwrap(),
            vec![],
            args
        );

        let resp = self.execute_transaction(
            self.wallet.sign_transaction(
                &self.build_transaction_data(
                    ptb.finish()
                ).await?
            )
        ).await?;

        Ok((
            self.extract_created_object(&resp, "Song")?,
            resp.digest
        ))
    }

    fn extract_created_object(
        &self,
        resp: &IotaTransactionBlockResponse,
        object_name: &str
    ) -> Result<ObjectID> {
        let IotaTransactionBlockResponse {
            object_changes: Some(object_changes),
            ..
        } = resp else {
            bail!("Can't find {object_name}'s ID");
        };

        let Some(song_id) = object_changes.into_iter().find_map(|change| {
            match change {
                ObjectChange::Created {
                    object_type,
                    object_id,
                    ..
                } => {
                    if ObjectID::from(object_type.address) != self.package {
                        return None;
                    }

                    if object_type.name.as_str() != object_name {
                        return None;
                    }
        
                    Some(object_id)
                }
                _ => None
            }
        }) else {
            bail!("Can't find {object_name}'s ID");
        };

        Ok(*song_id)
    }

    async fn build_transaction_data(
        &self,
        pt: ProgrammableTransaction
    ) -> Result<TransactionData> {
        let sender = self.wallet.active_address()?;
        let client = self.client().await?;
        let coin = client
            .coin_read_api()
            .get_coins(sender, None, None, None).await?
            .data.into_iter().next().unwrap();

        Ok(TransactionData::new_programmable(
            sender,
            vec![coin.object_ref()],
            pt,
            10_000_000,
            client.read_api().get_reference_gas_price().await?,
        ))
    }
    
    async fn execute_transaction(&self, tx: Transaction) -> Result<IotaTransactionBlockResponse> {
        let response = self
            .wallet
            .execute_transaction_may_fail(tx)
            .await
            .context("Error executing transaction")?;

        let Some(effects) = &response.effects else {
            bail!("Failed to find effects for transaction");
        };

        if let IotaExecutionStatus::Failure { error } = effects.status() {
            bail!(error.to_owned());
        }

        Ok(response)
    }

    async fn client(&self) -> Result<IotaClient> {
        self.wallet.get_client().await.context("Error fetching client")
    }
}
