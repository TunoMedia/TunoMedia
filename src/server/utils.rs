use tonic::transport::{Identity, ServerTlsConfig};
use std::{fs, io::BufReader, path::PathBuf};
use anyhow::Result;

use crate::local_storage::get_local_song_file;

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

pub fn get_song_reader(object_id: &str) -> Result<BufReader<fs::File>> {
    Ok(
        BufReader::new(
            get_local_song_file(object_id)?
        )
    )
}
