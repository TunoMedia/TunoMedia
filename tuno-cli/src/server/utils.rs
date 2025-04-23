use iota_sdk::types::base_types::IotaAddress;
use iota_sdk::types::supported_protocol_versions::ProtocolConfig;
use iota_sdk::types::transaction::{Argument, CallArg, Command, Transaction, TransactionKind};

use tonic::transport::{Identity, ServerTlsConfig};
use std::{fs, path::PathBuf};
use anyhow::{bail, Result};

use crate::client::Client;

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
    client: &Client
) -> Result<(String, Transaction)> {
    let Ok(tx): Result<Transaction, _> = bcs::from_bytes(&raw_transaction) else {
        bail!("Transaction could not be deserialized");
    };

    if tx.verify_signature_for_testing(0, &Default::default()).is_err() {
        bail!("Signature could not be verified");
    }

    let (kind, _, _) = tx.transaction_data().execution_parts();
    kind.validity_check(&ProtocolConfig::get_for_min_version())?;

    let TransactionKind::ProgrammableTransaction(pt) = kind else {
        bail!("Transaction does not contain a PTB")
    };

    let Some(Command::MoveCall(call)) = pt.commands.last() else {
        bail!("Last command is not a call");
    };

    if !call.package.eq(&client.package_id) {
        bail!("Call does not target `{}` as package", client.package_id);
    }

    if !call.module.to_string().eq("tuno") {
        bail!("Call does not target `tuno` as module");
    }

    if !call.function.to_string().eq("pay_royalties") {
        bail!("Call does not target `pay_royalties` as function");
    }

    let Some(
        CallArg::Object(song)
    ) = get_argument_by_index(&call.arguments, &pt.inputs, 0) else {
        bail!("Could not parse song's object");
    };

    let Some(
        CallArg::Pure(distributor_bytes)
    ) = get_argument_by_index(&call.arguments, &pt.inputs, 1) else {
        bail!("Could not parse distributor");
    };

    let Ok(distributor) = IotaAddress::from_bytes(distributor_bytes) else {
        bail!("Could not read distributor's address");
    };

    if distributor != client.address {
        bail!("Distributor's address is not correct");
    }

    Ok((song.id().to_hex(), tx))
}

fn get_argument_by_index<'a>(args: &'a Vec<Argument>, inputs: &'a Vec<CallArg>, index: usize) -> Option<&'a CallArg> {
    match args.get(index) {
        Some(&Argument::Input(i)) => inputs.get(i as usize),
        _ => None
    }
}
