use tonic::transport::{Identity, ServerTlsConfig};
use std::{fs, path::PathBuf};
use anyhow::Result;

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

// TODO: get object_id as bytes
pub fn get_file(object_id: &str) -> Result<Vec<u8>, std::io::Error> {
    fs::read(format!("./media/{object_id}.mp3"))
}
