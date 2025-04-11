use std::path::PathBuf;

use iota_sdk::{
    rpc_types::{IotaParsedData, IotaTransactionBlockResponse},
    types::{
        Identifier,
        base_types::ObjectID,
        digests::TransactionDigest,
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        transaction::{Argument, ObjectArg, ProgrammableTransaction, TransactionData}
    }, wallet_context::WalletContext
};
use anyhow::{bail, Result};
use clap::Parser;
use log::{error, info, trace};

use crate::{displays::song_listing::{DisplayListing, SongList, SongListing}, local_storage::get_all_song_ids, utils::{
    execute_transaction, extract_created_cap, extract_created_kiosk, extract_created_kiosk_cap, extract_created_song, get_initial_shared_version, query_kiosk_songs, query_owned_songs
}};

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
pub struct OwnedKiosk {
    /// Kiosk's object id on which to place display item
    #[arg(long, env = "KIOSK")]
    kiosk: ObjectID,

    /// Kiosk owner capability's object id
    #[arg(long, env = "KIOSK_CAP")]
    kiosk_cap: ObjectID,
}

impl OwnedKiosk {
    pub(crate) async fn as_arguments(
        &self,
        wallet: &WalletContext,
        ptb: &mut ProgrammableTransactionBuilder
    ) -> Result<Vec<Argument>> {
        Ok(vec![
            ptb.obj(ObjectArg::SharedObject {
                id: self.kiosk,
                initial_shared_version: get_initial_shared_version(wallet, self.kiosk).await?,
                mutable: true
            })?,
            ptb.obj(ObjectArg::ImmOrOwnedObject(
                wallet.get_object_ref(self.kiosk_cap).await?
            ))?,
        ])
    }
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

    pub(crate) async fn make_song_available(
        &self,
        song: ObjectID,
        owned_kiosk: OwnedKiosk
    ) -> Result<TransactionDigest> {
        let mut ptb = ProgrammableTransactionBuilder::new();
        let mut args = vec![
            ptb.obj(ObjectArg::ImmOrOwnedObject(
                self.wallet.get_object_ref(song).await?
            ))?,
        ];
        args.append(&mut owned_kiosk.as_arguments(&self.wallet, &mut ptb).await?);

        ptb.programmable_move_call(
            self.package_id,
            Identifier::new("tuno").unwrap(),
            Identifier::new("make_song_available").unwrap(),
            vec![],
            args
        );

        Ok(
            self.build_and_execute_transaction_data(
                ptb.finish()
            ).await?
            .digest
        )
    }

    pub(crate) async fn make_song_unavailable(
        &self,
        song: ObjectID,
        owned_kiosk: OwnedKiosk
    ) -> Result<TransactionDigest> {
        let mut ptb = ProgrammableTransactionBuilder::new();
        let mut args = vec![
            ptb.obj(ObjectArg::ImmOrOwnedObject(
                self.wallet.get_object_ref(song).await?
            ))?,
        ];
        args.append(&mut owned_kiosk.as_arguments(&self.wallet, &mut ptb).await?);

        ptb.programmable_move_call(
            self.package_id,
            Identifier::new("tuno").unwrap(),
            Identifier::new("make_song_unavailable").unwrap(),
            vec![],
            args
        );

        Ok(
            self.build_and_execute_transaction_data(
                ptb.finish()
            ).await?.digest
        )
    }

    pub(crate) async fn distribute_all(
        &self,
        url: &str,
        streaming_price: usize
    ) -> Result<Vec<ObjectID>> {
        let mut distributing = vec![];
        for song_id in get_all_song_ids()? {
            let song = ObjectID::from_hex_literal(&song_id)?;
            match self.distribute(song, url, streaming_price).await {
                Ok(digest) => {
                    distributing.push(song);
                    info!("Registered to distribute {} [{}]", song_id, digest)
                },
                Err(e) => error!("Could not register to {}: {}", song_id, e)
            }
        }

        Ok(distributing)
    }

    async fn distribute(
        &self,
        song: ObjectID,
        url: &str,
        streaming_price: usize
    ) -> Result<TransactionDigest> {
        let mut ptb = ProgrammableTransactionBuilder::new();
        let args = vec![
            ptb.obj(ObjectArg::ImmOrOwnedObject(
                self.wallet.get_object_ref(song).await?
            ))?,
            ptb.pure(url)?,
            ptb.pure(streaming_price)?

        ];

        ptb.programmable_move_call(
            self.package_id,
            Identifier::new("tuno").unwrap(),
            Identifier::new("register_as_distributor").unwrap(),
            vec![],
            args
        );

        Ok(
            self.build_and_execute_transaction_data(
                ptb.finish()
            ).await?.digest
        )
    }

    pub(crate) async fn undistribute_all(&self) -> Result<Vec<ObjectID>> {
        let mut undistributed = vec![];
        for song_id in get_all_song_ids()? {
            let song = ObjectID::from_hex_literal(&song_id)?;
            match self.undistribute(song).await {
                Ok(digest) => {
                    undistributed.push(song);
                    info!("unregistered on {} [{}]", song_id, digest);
                },
                Err(e) => error!("Could not unregister on {}: {}", song_id, e)
            }
        }

        Ok(undistributed)
    }

    pub(crate) async fn undistribute(
        &self,
        song: ObjectID
    ) -> Result<TransactionDigest> {
        let mut ptb = ProgrammableTransactionBuilder::new();
        let args = vec![
            ptb.obj(ObjectArg::ImmOrOwnedObject(
                self.wallet.get_object_ref(song).await?
            ))?,
        ];

        ptb.programmable_move_call(
            self.package_id,
            Identifier::new("tuno").unwrap(),
            Identifier::new("remove_as_distributor").unwrap(),
            vec![],
            args
        );

        Ok(
            self.build_and_execute_transaction_data(
                ptb.finish()
            ).await?.digest
        )
    }

    pub(crate) async fn get_all_owned_songs(&self) -> Result<SongList> {
        let songs: Vec<SongListing> = query_owned_songs(&self.wallet, self.package_id).await?
            .into_iter()
            .map(|obj| obj.data.unwrap().content.unwrap())
            .map(|content| match content {
                IotaParsedData::MoveObject(o) => SongListing::from(o.fields),
                _ => panic!("IOTA Object Response could not be parsed")
            }).collect();

        Ok(SongList::from(songs))
    }

    pub(crate) async fn get_kiosk_songs(&self, kiosk: ObjectID) -> Result<SongList> {
        let songs: Vec<DisplayListing> = query_kiosk_songs(&self.wallet, kiosk).await?
            .into_iter()
            .map(|obj| obj.data.unwrap().content.unwrap())
            .map(|content| match content {
                IotaParsedData::MoveObject(o) => DisplayListing::from(o.fields),
                _ => panic!("IOTA Object Response could not be parsed")
            }).collect();

        Ok(SongList::from(songs))
    }

    async fn build_and_execute_transaction_data(
        &self,
        pt: ProgrammableTransaction
    ) -> Result<IotaTransactionBlockResponse> {
        trace!("building transaction: \n{}", pt.to_string());
        let sender = self.wallet.active_address()?;
        let tx_data = TransactionData::new_programmable(
            sender,
            self.wallet.get_all_gas_objects_owned_by_address(sender).await?,
            pt,
            10_000_000,
            self.wallet.get_reference_gas_price().await?,
        );

        trace!("Signing {}...", tx_data.digest());
        let tx = self.wallet.sign_transaction(&tx_data);

        trace!("Executing {}...", tx.digest());
        execute_transaction(&self.wallet, tx).await
    }
}
