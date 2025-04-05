use iota_sdk::IotaClientBuilder;
use anyhow::Result;

pub mod config;
pub mod server;

pub async fn run() -> Result<()> {
    let config = config::load_config()?;

    // IOTA testnet running at `https://api.testnet.iota.cafe`
    let iota_testnet = IotaClientBuilder::default().build_testnet().await?;
    println!("IOTA testnet version: {:?}", iota_testnet.api_version());

    let server = server::TunoGrpcServer::new(
        config.server.host,
        config.server.port,
        config.server.cert_dir
    );

    env_logger::init();
    server.run().await?;
    
    Ok(())
}
