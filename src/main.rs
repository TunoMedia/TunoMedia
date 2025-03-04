use anyhow::Result;

use tuno_cli::run;

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}
