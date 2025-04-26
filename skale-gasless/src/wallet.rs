use ethers::{
    prelude::*,
    providers::{Http, Provider},
    types::Address,
};
use rand::thread_rng;

use crate::chains::ChainConfig;

/// Creates a new random wallet connected to the provider's chain
pub async fn create_wallet(
    config: &ChainConfig
) -> Result<(LocalWallet, Address, SignerMiddleware<Provider<Http>, LocalWallet>), Box<dyn std::error::Error>> {
    // Connect to the network
    let provider = Provider::<Http>::try_from(config.rpc_url.as_str())?;
    
    // Get the chain ID from the provider
    let chain_id = config.chain_id;

    // Create a random wallet and connect it to the provider
    let wallet = LocalWallet::new(&mut thread_rng());
    let wallet_address = wallet.address();
    let wallet_with_chain_id = wallet.with_chain_id(config.chain_id);
    let client = SignerMiddleware::new(provider.clone(), wallet_with_chain_id.clone());

    println!("Generated wallet address: {}", wallet_address);
    
    Ok((wallet_with_chain_id, wallet_address, client))
}