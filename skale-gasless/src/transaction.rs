use ethers::{
    prelude::*,
    providers::{Http, Provider},
    types::{TransactionReceipt, TransactionRequest, U256},
    utils::hex,
};

use crate::{chains::ChainConfig, miner, wallet};

/// Entry point for the gas request process
pub async fn request_gas(
    config: &ChainConfig
) -> Result<Option<TransactionReceipt>, Box<dyn std::error::Error>> {
    // Connect to the network
    let provider = Provider::<Http>::try_from(config.rpc_url.as_str())?;
    
    // Create a wallet and get the signer client
    let (wallet_with_chain_id, wallet_address, client) = wallet::create_wallet(config).await?;

    // Get the current nonce for the wallet
    let nonce = provider.get_transaction_count(wallet_address, None).await?;
    
    // Set gas limit
    let gas: u64 = 100000;
    
    // Mine for gas
    println!("Mining for gas price...");
    let result = miner::mine_gas_for_transaction(nonce.as_u64(), gas, wallet_address).await?;
    println!("Mining completed in {:.2} seconds", result.duration);
    println!("Found gas price: {}", result.gas_price);
    
    // Create the transaction data
    let transaction = create_transaction(
        wallet_address,
        config.target_address,
        nonce,
        result.gas_price,
        gas,
        &config.function_signature_without_prefix(),
    )?;
    
    // Send the transaction
    send_transaction(client, transaction).await
}

/// Create a transaction with the mined gas price
fn create_transaction(
    from: Address,
    to: Address,
    nonce: U256,
    gas_price: U256,
    gas: u64,
    function_signature: &str,
) -> Result<TransactionRequest, Box<dyn std::error::Error>> {
    // Create the transaction data
    let mut data = hex::decode(function_signature)?;
    let mut address_padded = vec![0u8; 12]; // Padding to 32 bytes
    address_padded.extend_from_slice(&from.as_bytes());
    data.extend_from_slice(&address_padded);
    
    // Create the transaction
    let tx = TransactionRequest::new()
        .from(from)
        .to(to)
        .nonce(nonce)
        .gas_price(gas_price)
        .gas(U256::from(gas))
        .data(Bytes::from(data));
    
    Ok(tx)
}

/// Send a transaction and handle the result
async fn send_transaction(
    client: SignerMiddleware<Provider<Http>, LocalWallet>,
    tx: TransactionRequest,
) -> Result<Option<TransactionReceipt>, Box<dyn std::error::Error>> {
    println!("Sending transaction...");
    
    // Send transaction
    let tx_result = client.send_transaction(tx, None).await;
    
    // Handle the result
    match tx_result {
        Ok(pending_tx) => {
            println!("Transaction submitted with hash: {}", pending_tx.tx_hash());
            println!("Waiting for confirmation...");
            
            match pending_tx.await {
                Ok(receipt_option) => {
                    match receipt_option {
                        Some(receipt) => {
                            println!("Transaction confirmed in block: {}", receipt.block_number.unwrap_or_default());
                            Ok(Some(receipt))
                        }
                        None => {
                            println!("Transaction pending but not yet mined");
                            Ok(None)
                        }
                    }
                }
                Err(e) => {
                    println!("Error while waiting for transaction: {}", e);
                    Err(e.into())
                }
            }
        }
        Err(e) => {
            println!("Failed to send transaction: {}", e);
            
            // Provide more detailed guidance based on the error
            if e.to_string().contains("Invalid transaction signature") {
                println!("\nThe 'Invalid transaction signature' error typically occurs when:");
                println!("1. The wallet has no funds to pay for gas");
                println!("2. The account doesn't exist on the blockchain");
                println!("3. The chain ID is incorrect");
                println!("\nTo resolve this:");
                println!("- Get testnet ETH from a faucet");
                println!("- Make sure the RPC URL is correct");
                println!("- Verify the chain ID is correct for this network");
            }
            
            Err(e.into())
        }
    }
}