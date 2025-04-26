mod chains;
mod miner;
mod transaction;
mod wallet;

use chains::{ChainConfig, NetworkType, SkaleChain};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use default config (Nebula Testnet)
    //let config = chains::get_default_config();
    let config = ChainConfig::new(SkaleChain::Nebula, NetworkType::Testnet);

    println!("Using network: {}", config.name());    
    transaction::request_gas(&config).await?;
    Ok(())
}