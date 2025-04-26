use ethers::types::{Address, Chain, U256};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkaleChain {
    Calypso,
    Nebula,
    Titan,
    Europa,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkType {
    Mainnet,
    Testnet,
}

#[derive(Debug, Clone)]
pub struct ChainConfig {
    pub chain: SkaleChain,
    pub network_type: NetworkType,
    pub rpc_url: String,
    pub chain_id: u64,
    pub function_signature: String,
    pub target_address: Address,
}

impl ChainConfig {
    pub fn new(chain: SkaleChain, network_type: NetworkType) -> Self {
        let (rpc_url, chain_id, function_signature, target_address) = match (chain, network_type) {
            // Calypso configurations
            (SkaleChain::Calypso, NetworkType::Mainnet) => (
                "https://mainnet.skalenodes.com/v1/honorable-steel-rasalhague",
                1564830818,
                "0x0c11dedd",
                Address::from_str("0x02891b34B7911A9C68e82C193cd7A6fBf0c3b30A").unwrap(),
            ),
            (SkaleChain::Calypso, NetworkType::Testnet) => (
                "https://testnet.skalenodes.com/v1/giant-half-dual-testnet",
                974399131,
                "0x0c11dedd",
                Address::from_str("0x62Fe932FF26e0087Ae383f6080bd2Ed481bA5A8A").unwrap(),
            ),
            
            // Nebula configurations
            (SkaleChain::Nebula, NetworkType::Mainnet) => (
                "https://mainnet.skalenodes.com/v1/green-giddy-denebola",
                1482601649,
                "0x0c11dedd",
                Address::from_str("0x5a6869ef5b81DCb58EBF51b8F893c31f5AFE3Fa8").unwrap(),
            ),
            (SkaleChain::Nebula, NetworkType::Testnet) => (
                "https://testnet.skalenodes.com/v1/lanky-ill-funny-testnet",
                37084624,
                "0x0c11dedd",
                Address::from_str("0x000E9c53C4e2e21F5063f2e232d0AA907318dccb").unwrap(),
            ),
            
            // Titan configurations
            (SkaleChain::Titan, NetworkType::Mainnet) => (
                "https://mainnet.skalenodes.com/v1/parallel-stormy-spica",
                1350216234,
                "0x0c11dedd",
                Address::from_str("0xa5C297dF8f8386E4b940D61EF9A8f2bB367a6fAB").unwrap(),
            ),
            (SkaleChain::Titan, NetworkType::Testnet) => (
                "https://testnet.skalenodes.com/v1/aware-fake-trim-testnet",
                1020352220,
                "0x0c11dedd",
                Address::from_str("0x08f98Af60eb83C18184231591A8F89577E46A4B9").unwrap(),
            ),
            
            // Europa configurations
            (SkaleChain::Europa, NetworkType::Mainnet) => (
                "https://mainnet.skalenodes.com/v1/elated-tan-skat",
                2046399126,
                "0x6a627842",
                Address::from_str("0x2B267A3e49b351DEdac892400a530ABb2f899d23").unwrap(),
            ),
            (SkaleChain::Europa, NetworkType::Testnet) => (
                "https://testnet.skalenodes.com/v1/juicy-low-small-testnet",
                1444673419,
                "0x0c11dedd",
                Address::from_str("0x366727B410fE55774C8b0B5b5A6E2d74199a088A").unwrap(),
            ),
        };

        Self {
            chain,
            network_type,
            rpc_url: rpc_url.to_string(),
            chain_id,
            function_signature: function_signature.to_string(),
            target_address,
        }
    }

    pub fn name(&self) -> String {
        let chain_name = match self.chain {
            SkaleChain::Calypso => "Calypso",
            SkaleChain::Nebula => "Nebula",
            SkaleChain::Titan => "Titan",
            SkaleChain::Europa => "Europa",
        };

        let network_name = match self.network_type {
            NetworkType::Mainnet => "Mainnet",
            NetworkType::Testnet => "Testnet",
        };

        format!("SKALE {} {}", chain_name, network_name)
    }
    
    // Helper method to get function signature without 0x prefix
    pub fn function_signature_without_prefix(&self) -> String {
        self.function_signature.strip_prefix("0x").unwrap_or(&self.function_signature).to_string()
    }
}

// Default configuration
pub fn get_default_config() -> ChainConfig {
    ChainConfig::new(SkaleChain::Nebula, NetworkType::Testnet)
}
