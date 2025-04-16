use iota_sdk::types::{base_types::ObjectID, transaction::{Command, Transaction}};
use tonic::transport::{Identity, ServerTlsConfig};
use std::{fs, path::PathBuf};
use anyhow::{bail, Result};

pub fn load_tls_config(cert_path: &PathBuf, key_path: &PathBuf) -> Result<ServerTlsConfig> {
    if !cert_path.exists() {
        return Err(anyhow::anyhow!("Certificate file not found: {:?}", cert_path));
    }

    if !key_path.exists() {
        return Err(anyhow::anyhow!("Certificate file not found: {:?}", key_path));
    }

    let cert_data = fs::read(&cert_path)?;
    let key_data = fs::read(&key_path)?;

    let identity = Identity::from_pem(cert_data, key_data);
    
    Ok(ServerTlsConfig::new().identity(identity))
}

pub fn verify_payment(
    raw_transaction: Vec<u8>,
    package_id: ObjectID
) -> Result<String> {
    let Ok(tx): Result<Transaction, _> = bcs::from_bytes(&raw_transaction) else {
        bail!("Transaction could not be deserialized");
    };

    if tx.verify_signature_for_testing(0, &Default::default()).is_err() {
        bail!("Signature could not be verified");
    }

    let (kind, _, _) = tx.transaction_data().execution_parts();
    if !kind.num_commands().eq(&1) {
        bail!("Transaction contains more than one command");
    }

    let Some(Command::MoveCall(call)) = kind.iter_commands().next() else {
        bail!("Command is not a call");
    };

    if !call.package.eq(&package_id) {
        bail!("Call does not target `{package_id}` as package");
    }

    if !call.module.to_string().eq("tuno") {
        bail!("Call does not target `tuno` as module");
    }

    if !call.function.to_string().eq("pay_royalties") {
        bail!("Call does not target `pay_royalties` as function");
    }

    let song = match kind.input_objects() {
        Ok(objs) if objs.len().eq(&2) => objs[0].object_id(),
        _ => bail!("Transaction does not contain a correct input")
    };

    Ok(song.to_hex())
}