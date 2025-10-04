//! Genesis configuration for Pasifika PoA Chain

use std::sync::Arc;
use alloy_genesis::Genesis;
use reth_chainspec::ChainSpec;

/// Creates the Pasifika PoA genesis configuration
/// 
/// Features:
/// - Zero gas (base fee = 0, gas limit very high)
/// - PoA consensus (no mining difficulty)
/// - Pre-funded accounts for testing
pub fn create_poa_genesis() -> Arc<ChainSpec> {
    let custom_genesis = r#"
{
    "nonce": "0x0",
    "timestamp": "0x0",
    "extraData": "0x506173696669636120506f41",
    "gasLimit": "0xFFFFFFFFFFFF",
    "difficulty": "0x0",
    "mixHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
    "coinbase": "0x0000000000000000000000000000000000000000",
    "alloc": {
        "0x0000000000000000000000000000000000000001": {
            "balance": "0x200000000000000000000000000000000000000000000000000000000000000"
        },
        "0x6Be02d1d3665660d22FF9624b7BE0551ee1Ac91b": {
            "balance": "0x200000000000000000000000000000000000000000000000000000000000000"
        },
        "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266": {
            "balance": "0x200000000000000000000000000000000000000000000000000000000000000"
        },
        "0x70997970C51812dc3A010C7d01b50e0d17dc79C8": {
            "balance": "0x200000000000000000000000000000000000000000000000000000000000000"
        },
        "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC": {
            "balance": "0x200000000000000000000000000000000000000000000000000000000000000"
        },
        "0x90F79bf6EB2c4f870365E785982E1f101E93b906": {
            "balance": "0x200000000000000000000000000000000000000000000000000000000000000"
        },
        "0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65": {
            "balance": "0x200000000000000000000000000000000000000000000000000000000000000"
        },
        "0x9965507D1a55bcC2695C58ba16FB37d819B0A4dc": {
            "balance": "0x200000000000000000000000000000000000000000000000000000000000000"
        },
        "0x976EA74026E726554dB657fA54763abd0C3a0aa9": {
            "balance": "0x200000000000000000000000000000000000000000000000000000000000000"
        },
        "0x14dC79964da2C08b23698B3D3cc7Ca32193d9955": {
            "balance": "0x200000000000000000000000000000000000000000000000000000000000000"
        }
    },
    "number": "0x0",
    "gasUsed": "0x0",
    "parentHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
    "baseFeePerGas": "0x0",
    "config": {
        "chainId": 999888,
        "homesteadBlock": 0,
        "eip150Block": 0,
        "eip155Block": 0,
        "eip158Block": 0,
        "byzantiumBlock": 0,
        "constantinopleBlock": 0,
        "petersburgBlock": 0,
        "istanbulBlock": 0,
        "muirGlacierBlock": 0,
        "berlinBlock": 0,
        "londonBlock": 0,
        "arrowGlacierBlock": 0,
        "grayGlacierBlock": 0,
        "shanghaiTime": 0,
        "cancunTime": 0,
        "terminalTotalDifficulty": 0,
        "terminalTotalDifficultyPassed": true
    }
}
"#;

    let genesis: Genesis = serde_json::from_str(custom_genesis)
        .expect("Failed to parse genesis configuration");
    
    Arc::new(genesis.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_creation() {
        let chain_spec = create_poa_genesis();
        assert_eq!(chain_spec.chain().id(), 999888);
    }

    #[test]
    fn test_zero_gas() {
        let chain_spec = create_poa_genesis();
        let genesis_header = chain_spec.genesis_header();
        // Verify base fee is 0 if London fork is active
        assert!(genesis_header.base_fee_per_gas.is_none() || genesis_header.base_fee_per_gas == Some(0));
    }
}
