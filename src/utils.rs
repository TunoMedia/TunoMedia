use iota_sdk::{
    rpc_types::{IotaExecutionStatus, IotaObjectData, IotaObjectDataOptions, IotaTransactionBlockEffectsAPI, IotaTransactionBlockResponse, ObjectChange},
    types::{base_types::{ObjectID, SequenceNumber}, object::Owner, transaction::Transaction, IOTA_FRAMEWORK_PACKAGE_ID},
    wallet_context::WalletContext
};
use anyhow::{bail, Context, Result};

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

pub(crate) async fn get_initial_shared_version(
    wallet: &WalletContext,
    id: ObjectID
) -> Result<SequenceNumber> {
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
    
    Ok(initial_shared_version)
}

pub(crate) async fn execute_transaction(
    wallet: &WalletContext,
    tx: Transaction
) -> Result<IotaTransactionBlockResponse> {
    let response = wallet
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
