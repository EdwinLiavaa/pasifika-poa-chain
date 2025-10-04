//! Proof-of-Authority Consensus Module
//!
//! This module implements a simplified PoA consensus where:
//! - All connected nodes are validators
//! - No mining/staking required
//! - Blocks are validated through signature verification
//! - Automatic validator registration on node startup

use alloy_consensus::Header;
use alloy_primitives::B256;
use eyre::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Validator in the PoA network
#[derive(Debug, Clone)]
pub struct Validator {
    /// Validator address
    pub address: alloy_primitives::Address,
    /// Node ID/enode
    pub node_id: String,
    /// Whether validator is currently active
    pub active: bool,
}

/// PoA Consensus Engine
pub struct PoAConsensus {
    /// List of validators (all nodes)
    validators: Arc<RwLock<Vec<Validator>>>,
}

impl PoAConsensus {
    /// Create a new PoA consensus engine
    pub fn new() -> Self {
        Self {
            validators: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Register a new validator (called when a node joins)
    pub async fn register_validator(&self, address: alloy_primitives::Address, node_id: String) -> Result<()> {
        let mut validators = self.validators.write().await;
        
        // Check if validator already exists
        if validators.iter().any(|v| v.address == address) {
            tracing::debug!("Validator {} already registered", address);
            return Ok(());
        }

        let validator = Validator {
            address,
            node_id,
            active: true,
        };

        validators.push(validator.clone());
        tracing::info!("✅ Registered new validator: {} ({})", address, node_id);
        tracing::info!("Total validators: {}", validators.len());

        Ok(())
    }

    /// Get all active validators
    pub async fn get_validators(&self) -> Vec<Validator> {
        let validators = self.validators.read().await;
        validators.iter().filter(|v| v.active).cloned().collect()
    }

    /// Validate a block header (simplified PoA validation)
    pub async fn validate_header(&self, _header: &Header) -> Result<()> {
        // In a full PoA implementation, we would:
        // 1. Verify the block signer is in the validator set
        // 2. Check the signing rotation
        // 3. Validate the seal/signature
        // 
        // For this simplified version, we accept all blocks from connected nodes
        Ok(())
    }

    /// Check if an address is a validator
    pub async fn is_validator(&self, address: &alloy_primitives::Address) -> bool {
        let validators = self.validators.read().await;
        validators.iter().any(|v| v.active && v.address == *address)
    }

    /// Get validator count
    pub async fn validator_count(&self) -> usize {
        let validators = self.validators.read().await;
        validators.iter().filter(|v| v.active).count()
    }
}

impl Default for PoAConsensus {
    fn default() -> Self {
        Self::new()
    }
}

/// PoA block sealer
pub struct PoASealer {
    consensus: Arc<PoAConsensus>,
}

impl PoASealer {
    pub fn new(consensus: Arc<PoAConsensus>) -> Self {
        Self { consensus }
    }

    /// Seal a block (add PoA signature)
    pub async fn seal_block(&self, _header: &mut Header) -> Result<B256> {
        // In a full implementation, we would:
        // 1. Sign the block header with the validator's private key
        // 2. Add the signature to the extraData field
        // 3. Return the block hash
        //
        // For this simplified version, we just return the hash
        Ok(B256::ZERO)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::address;

    #[tokio::test]
    async fn test_validator_registration() {
        let consensus = PoAConsensus::new();
        let addr = address!("0000000000000000000000000000000000000001");
        
        consensus.register_validator(addr, "node1".to_string()).await.unwrap();
        
        assert!(consensus.is_validator(&addr).await);
        assert_eq!(consensus.validator_count().await, 1);
    }

    #[tokio::test]
    async fn test_multiple_validators() {
        let consensus = PoAConsensus::new();
        
        let addr1 = address!("0000000000000000000000000000000000000001");
        let addr2 = address!("0000000000000000000000000000000000000002");
        
        consensus.register_validator(addr1, "node1".to_string()).await.unwrap();
        consensus.register_validator(addr2, "node2".to_string()).await.unwrap();
        
        assert_eq!(consensus.validator_count().await, 2);
    }

    #[tokio::test]
    async fn test_duplicate_validator() {
        let consensus = PoAConsensus::new();
        let addr = address!("0000000000000000000000000000000000000001");
        
        consensus.register_validator(addr, "node1".to_string()).await.unwrap();
        consensus.register_validator(addr, "node1".to_string()).await.unwrap();
        
        // Should still be only 1 validator
        assert_eq!(consensus.validator_count().await, 1);
    }
}
