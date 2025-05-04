use std::path::PathBuf;

use iota_sdk::rpc_types::{IotaExecutionResult, IotaExecutionStatus, IotaMoveValue, IotaParsedData, IotaTransactionBlockEffectsAPI as _, IotaTransactionBlockResponse};
use iota_sdk::types::Identifier;
use iota_sdk::types::digests::TransactionDigest;
use iota_sdk::types::base_types::{IotaAddress, ObjectID, ObjectRef};
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_sdk::types::transaction::{Argument, Command, ObjectArg, ProgrammableTransaction, Transaction, TransactionData, TransactionKind};
use iota_sdk::wallet_context::WalletContext;

use anyhow::{bail, Result};
use clap::Parser;
use log::{error, info, trace};

use crate::local_storage::{get_all_song_ids, FileMetadata};
use crate::types::{Song, SongDisplay, SongDisplayList, SongList};
use crate::utils::*;

#[derive(Parser, Clone)]
pub struct Connection {
    /// The IOTA CLI config file, (default: ~/.iota/iota_config/client.yaml)
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// Object ID of the game's package.
    #[arg(long, short, env = "PKG")]
    pub package_id: ObjectID,
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
            ptb.obj(get_shared_object_ref(self.kiosk, true, wallet).await?)?,
            ptb.obj(ObjectArg::ImmOrOwnedObject(
                wallet.get_object_ref(self.kiosk_cap).await?
            ))?
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

pub struct Client {
    wallet: WalletContext,
    pub address: IotaAddress,
    pub package_id: ObjectID,
}

impl Client {
    pub fn new(conn: Connection) -> Result<Self> {
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
        let address = wallet.active_address()?;
        Ok(Self {
            wallet,
            address,
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
        song_md: SongMetadata,
        file_md: FileMetadata,
    ) -> Result<(ObjectID, TransactionDigest)> {
        let mut ptb = ProgrammableTransactionBuilder::new();
        
        let mut args = song_md.as_arguments(&mut ptb)?;
        args.append(&mut file_md.as_arguments(&mut ptb)?);
        args.push(
            ptb.obj(ObjectArg::ImmOrOwnedObject(
                self.wallet.get_object_ref(cap).await?
            ))?
        );

        ptb.programmable_move_call(
            self.package_id,
            Identifier::new("tuno").unwrap(),
            Identifier::new("create_song").unwrap(),
            vec![get_usdc_type_tag()?],
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
            ptb.obj(get_shared_object_ref(song, true, &self.wallet).await?)?
        ];
        args.append(&mut owned_kiosk.as_arguments(&self.wallet, &mut ptb).await?);

        ptb.programmable_move_call(
            self.package_id,
            Identifier::new("tuno").unwrap(),
            Identifier::new("make_song_available").unwrap(),
            vec![get_usdc_type_tag()?],
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
            ptb.obj(get_shared_object_ref(song, true, &self.wallet).await?)?
        ];
        args.append(&mut owned_kiosk.as_arguments(&self.wallet, &mut ptb).await?);

        ptb.programmable_move_call(
            self.package_id,
            Identifier::new("tuno").unwrap(),
            Identifier::new("make_song_unavailable").unwrap(),
            vec![get_usdc_type_tag()?],
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
            ptb.obj(get_shared_object_ref(song, true, &self.wallet).await?)?,
            ptb.pure(url)?,
            ptb.pure(streaming_price)?

        ];

        ptb.programmable_move_call(
            self.package_id,
            Identifier::new("tuno").unwrap(),
            Identifier::new("register_as_distributor").unwrap(),
            vec![get_usdc_type_tag()?],
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
            ptb.obj(get_shared_object_ref(song, true, &self.wallet).await?)?
        ];

        ptb.programmable_move_call(
            self.package_id,
            Identifier::new("tuno").unwrap(),
            Identifier::new("remove_as_distributor").unwrap(),
            vec![get_usdc_type_tag()?],
            args
        );

        Ok(
            self.build_and_execute_transaction_data(
                ptb.finish()
            ).await?.digest
        )
    }

    pub async fn get_payment_transaction(
        &self,
        song: ObjectID,
        distributor: &IotaAddress
    ) -> Result<Transaction> {
        let price = self.get_total_price(song, distributor).await?;

        let mut ptb = ProgrammableTransactionBuilder::new();
        let coin = ptb.obj(ObjectArg::ImmOrOwnedObject(
            self.get_payment_coin(price).await?
        ))?;
        let amounts = vec![ptb.pure(price)?];
        let args = vec![
            ptb.obj(get_shared_object_ref(song, true, &self.wallet).await?)?,
            ptb.pure(distributor)?,
            ptb.command(Command::SplitCoins(coin, amounts))
        ];

        ptb.programmable_move_call(
            self.package_id,
            Identifier::new("tuno").unwrap(),
            Identifier::new("pay_royalties").unwrap(),
            vec![get_usdc_type_tag()?],
            args
        );

        Ok(
            self.build_and_sign_transaction_data(
                ptb.finish()
            ).await?
        )
    }

    async fn get_total_price(
        &self,
        song: ObjectID,
        distributor: &IotaAddress
    ) -> Result<usize> {
        let mut ptb = ProgrammableTransactionBuilder::new();
        let args = vec![
            ptb.obj(get_shared_object_ref(song, true, &self.wallet).await?)?,
            ptb.pure(distributor)?
        ];

        ptb.programmable_move_call(
            self.package_id,
            Identifier::new("tuno").unwrap(),
            Identifier::new("get_total_price").unwrap(),
            vec![get_usdc_type_tag()?],
            args
        );
    
        let dev_inspect_result = self.wallet.get_client().await?
            .read_api()
            .dev_inspect_transaction_block(
                self.address,
                TransactionKind::programmable(ptb.finish()),
                None,
                None,
                None,
            ).await?;

        let Some(results) = dev_inspect_result.results else {
            bail!("Couldn't parse results");
        };

        let Some(IotaExecutionResult { return_values, .. }) = results.get(0) else {
            bail!("Couldn't get first result");
        };

        let Some((res, _)) = return_values.get(0) else {
            bail!("Couldn't get first return value");
        };

        Ok(usize::from_le_bytes(res.as_slice().try_into()?))
    }

    pub(crate) async fn get_all_owned_songs(&self) -> Result<SongList> {
        let songs = query_owned_songs(&self.wallet, self.package_id).await?
            .into_iter()
            .map(|obj| obj.data.unwrap().content.unwrap())
            .map(|content| match content {
                IotaParsedData::MoveObject(o) => Song::from(o.fields),
                _ => panic!("IOTA Object Response could not be parsed")
            }).collect();

        Ok(songs)
    }

    pub(crate) async fn get_kiosk_songs(&self, kiosk: ObjectID) -> Result<SongDisplayList> {
        let songs = query_kiosk_songs(&self.wallet, kiosk).await?
            .into_iter()
            .map(|obj| obj.data.unwrap().content.unwrap())
            .map(|content| match content {
                IotaParsedData::MoveObject(o) => SongDisplay::from(o.fields),
                _ => panic!("IOTA Object Response could not be parsed")
            }).collect();

        Ok(songs)
    }

    pub async fn get_song(&self, song: ObjectID) -> Result<Song> {
        let Some(data) = query_object(&self.wallet, song).await?.data else {
            bail!("Song's data could not be found");
        };

        let Some(IotaParsedData::MoveObject(o)) = data.content else {
            bail!("IOTA Object Response could not be parsed")
        };

        Ok(Song::from(o.fields))
    }

    pub(crate) async fn get_payment_coin(&self, min_amount: usize) -> Result<ObjectRef> {
        for c in query_usdc_coins(self.address, &self.wallet).await? {
            let Some(data) = c.data else {
                continue;
            };

            let Some(IotaParsedData::MoveObject(o)) = &data.content else {
                continue;
            };

            if let Some(IotaMoveValue::String(b)) = o.fields.read_dynamic_field_value("balance") {
                if b.parse::<usize>().unwrap() >= min_amount {
                    return Ok(data.object_ref());
                }
            }
        }

        bail!("No coin available")
    }

    pub(crate) async fn build_and_sign_transaction_data(
        &self,
        pt: ProgrammableTransaction
    ) -> Result<Transaction> {
        trace!("building transaction: \n{}", pt.to_string());
        let sender = self.address;
        let tx_data = TransactionData::new_programmable(
            sender,
            self.wallet.get_all_gas_objects_owned_by_address(sender).await?,
            pt,
            50_000_000,
            self.wallet.get_reference_gas_price().await?,
        );

        trace!("Signing {}...", tx_data.digest());
        Ok(self.wallet.sign_transaction(&tx_data))
    }

    async fn build_and_execute_transaction_data(
        &self,
        pt: ProgrammableTransaction
    ) -> Result<IotaTransactionBlockResponse> {
        let tx: Transaction = self.build_and_sign_transaction_data(pt).await?;

        trace!("Executing {}...", tx.digest());
        self.execute_transaction(tx).await
    }

    pub(crate) async fn execute_transaction(
        &self,
        tx: Transaction
    ) -> Result<IotaTransactionBlockResponse> {
        let response = match self.wallet.execute_transaction_may_fail(tx).await {
            Ok(res) => res,
            Err(e) => bail!("Error executing tx: {e}")
        };
    
        let Some(effects) = &response.effects else {
            bail!("Failed to find effects for transaction");
        };
    
        if let IotaExecutionStatus::Failure { error } = effects.status() {
            bail!("Error {}, executing {}", error.to_owned(), response.digest);
        }
    
        Ok(response)
    }

}
