use std::str::FromStr as _;

use iota_sdk::rpc_types::{IotaObjectData, IotaObjectDataFilter, IotaObjectDataOptions, IotaObjectResponse, IotaObjectResponseQuery, IotaTransactionBlockResponse, ObjectChange};
use iota_sdk::types::coin::Coin;
use iota_sdk::types::transaction::ObjectArg;
use iota_sdk::types::base_types::{IotaAddress, ObjectID};
use iota_sdk::types::object::Owner;
use iota_sdk::types::TypeTag;
use iota_sdk::types::IOTA_FRAMEWORK_PACKAGE_ID;
use iota_sdk::wallet_context::WalletContext;

use anyhow::{bail, Result};

use crate::constants::USDC_TYPE_TAG_STR;

pub(crate) fn extract_created_cap(
    resp: &IotaTransactionBlockResponse,
    package_id: ObjectID
) -> Result<ObjectID> {
    extract_created_object(resp, "CreatorCap", package_id)
}

pub(crate) fn extract_created_song(
    resp: &IotaTransactionBlockResponse,
    package_id: ObjectID
) -> Result<ObjectID> {
    extract_created_object(resp, "Song", package_id)
}

pub(crate) fn extract_created_kiosk(resp: &IotaTransactionBlockResponse) -> Result<ObjectID> {
    extract_created_object(resp, "Kiosk", IOTA_FRAMEWORK_PACKAGE_ID)
}

pub(crate) fn extract_created_kiosk_cap(resp: &IotaTransactionBlockResponse) -> Result<ObjectID> {
    extract_created_object(resp, "KioskOwnerCap", IOTA_FRAMEWORK_PACKAGE_ID)
}

fn extract_created_object(
    resp: &IotaTransactionBlockResponse,
    object_name: &str,
    object_addr: ObjectID
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
                if ObjectID::from(object_type.address) != object_addr {
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

pub(crate) async fn get_shared_object_ref(
    id: ObjectID,
    mutable: bool,
    wallet: &WalletContext,
) -> Result<ObjectArg> {
    let response = wallet.get_client().await?
        .read_api()
        .get_object_with_options(
            id,
            IotaObjectDataOptions::new().with_owner(),
        ).await?;

    if let Some(err) = response.error {
        bail!(err);
    }

    let Some(IotaObjectData {
        owner: Some(owner),
        ..
    }) = response.data else {
        bail!("No data for object {}", id);
    };

    let Owner::Shared {
        initial_shared_version
    } = owner else {
        bail!("Object {} is not shared", id);
    };

    Ok(ObjectArg::SharedObject { id, initial_shared_version, mutable })
}

pub(crate) async fn query_owned_songs(
    _wallet: &WalletContext,
    _package_id: ObjectID
) -> Result<Vec<IotaObjectResponse>> {
    todo!("Get all events emited filter by owner")
}

pub(crate) async fn query_usdc_coins(address: IotaAddress, wallet: &WalletContext) -> Result<Vec<IotaObjectResponse>> {
    let response = wallet.get_client().await?
        .read_api()
        .get_owned_objects(
            address,
            IotaObjectResponseQuery {
                filter: Some(IotaObjectDataFilter::StructType(
                    Coin::type_(get_usdc_type_tag()?)
                )),
                options: Some(IotaObjectDataOptions::new().with_content())
            },
            None,
            None
        ).await?;
        // TODO: Deal with next page

    Ok(response.data)
}

pub(crate) async fn query_kiosk_songs(
    wallet: &WalletContext,
    kiosk: ObjectID
) -> Result<Vec<IotaObjectResponse>> {
    let client = wallet.get_client().await?;
    let fields = client
        .read_api()
        .get_dynamic_fields(kiosk, None, None).await?
        .data;
    // TODO: Deal with next page

    let mut displays = vec![];
    for f in fields {
        displays.push(
            client.read_api()
                .get_object_with_options(
                    f.object_id,
                    IotaObjectDataOptions::new().with_content()
                ).await?
        );
    }

    Ok(displays)
}

pub(crate) async fn query_object(
    wallet: &WalletContext,
    song: ObjectID
) -> Result<IotaObjectResponse> {
    let response = wallet.get_client().await?
        .read_api()
        .get_object_with_options(
            song,
            IotaObjectDataOptions::new().with_content()
        ).await?;

    Ok(response)
}

pub(crate) fn get_usdc_type_tag() -> Result<TypeTag> {
    TypeTag::from_str(USDC_TYPE_TAG_STR)
}
