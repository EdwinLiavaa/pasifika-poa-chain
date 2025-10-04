//! P2P Discovery and Networking Module
//!
//! Custom P2P discovery logic for Pasifika PoA:
//! - Automatic peer discovery
//! - All peers are treated as validators
//! - Custom bootnode configuration

use eyre::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug};

/// Represents a peer in the P2P network
#[derive(Debug, Clone)]
pub struct Peer {
    /// Peer's enode URL
    pub enode: String,
    /// Peer's address (if known)
    pub address: Option<alloy_primitives::Address>,
    /// Whether peer is connected
    pub connected: bool,
    /// Whether peer is a validator
    pub is_validator: bool,
}

/// P2P Discovery Manager
pub struct P2PDiscovery {
    /// List of known peers
    peers: Arc<RwLock<Vec<Peer>>>,
    /// Bootnodes
    bootnodes: Vec<String>,
}

impl P2PDiscovery {
    /// Create a new P2P discovery manager
    pub fn new(bootnodes: Vec<String>) -> Self {
        Self {
            peers: Arc::new(RwLock::new(Vec::new())),
            bootnodes,
        }
    }

    /// Initialize discovery (connect to bootnodes)
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing P2P discovery...");
        
        for bootnode in &self.bootnodes {
            info!("Connecting to bootnode: {}", bootnode);
            self.add_peer(bootnode.clone(), None, true).await?;
        }

        info!("P2P discovery initialized with {} bootnodes", self.bootnodes.len());
        Ok(())
    }

    /// Add a new peer
    pub async fn add_peer(
        &self,
        enode: String,
        address: Option<alloy_primitives::Address>,
        is_validator: bool,
    ) -> Result<()> {
        let mut peers = self.peers.write().await;

        // Check if peer already exists
        if peers.iter().any(|p| p.enode == enode) {
            debug!("Peer {} already exists", enode);
            return Ok(());
        }

        let peer = Peer {
            enode: enode.clone(),
            address,
            connected: true,
            is_validator,
        };

        peers.push(peer);
        info!("Added new peer: {} (validator: {})", enode, is_validator);
        
        Ok(())
    }

    /// Get all connected peers
    pub async fn get_peers(&self) -> Vec<Peer> {
        let peers = self.peers.read().await;
        peers.iter().filter(|p| p.connected).cloned().collect()
    }

    /// Get validator peers
    pub async fn get_validator_peers(&self) -> Vec<Peer> {
        let peers = self.peers.read().await;
        peers.iter()
            .filter(|p| p.connected && p.is_validator)
            .cloned()
            .collect()
    }

    /// Mark a peer as validator
    pub async fn mark_as_validator(&self, enode: &str) -> Result<()> {
        let mut peers = self.peers.write().await;
        
        if let Some(peer) = peers.iter_mut().find(|p| p.enode == enode) {
            if !peer.is_validator {
                peer.is_validator = true;
                info!("Marked peer {} as validator", enode);
            }
        }

        Ok(())
    }

    /// Handle peer connection
    pub async fn on_peer_connected(&self, enode: String) -> Result<()> {
        info!("Peer connected: {}", enode);
        
        // In Pasifika PoA, all connected peers automatically become validators
        self.add_peer(enode.clone(), None, true).await?;
        
        info!("Auto-registered peer as validator");
        Ok(())
    }

    /// Handle peer disconnection
    pub async fn on_peer_disconnected(&self, enode: &str) -> Result<()> {
        info!("Peer disconnected: {}", enode);
        
        let mut peers = self.peers.write().await;
        if let Some(peer) = peers.iter_mut().find(|p| p.enode == enode) {
            peer.connected = false;
        }

        Ok(())
    }

    /// Get peer count
    pub async fn peer_count(&self) -> usize {
        let peers = self.peers.read().await;
        peers.iter().filter(|p| p.connected).count()
    }

    /// Get validator count
    pub async fn validator_peer_count(&self) -> usize {
        let peers = self.peers.read().await;
        peers.iter().filter(|p| p.connected && p.is_validator).count()
    }
}

/// Build enode URL from node information
pub fn build_enode(
    node_id: &str,
    ip: &str,
    port: u16,
    discovery_port: Option<u16>,
) -> String {
    let disc_port = discovery_port.unwrap_or(port);
    format!("enode://{}@{}:{}?discport={}", node_id, ip, port, disc_port)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_peer() {
        let discovery = P2PDiscovery::new(vec![]);
        let enode = "enode://test@127.0.0.1:30303".to_string();
        
        discovery.add_peer(enode.clone(), None, true).await.unwrap();
        
        assert_eq!(discovery.peer_count().await, 1);
        assert_eq!(discovery.validator_peer_count().await, 1);
    }

    #[tokio::test]
    async fn test_peer_connection() {
        let discovery = P2PDiscovery::new(vec![]);
        let enode = "enode://test@127.0.0.1:30303".to_string();
        
        discovery.on_peer_connected(enode.clone()).await.unwrap();
        
        // Should be automatically marked as validator
        assert_eq!(discovery.validator_peer_count().await, 1);
    }

    #[test]
    fn test_build_enode() {
        let enode = build_enode(
            "abc123",
            "192.168.1.1",
            30303,
            Some(30304),
        );
        
        assert!(enode.contains("abc123"));
        assert!(enode.contains("192.168.1.1:30303"));
        assert!(enode.contains("discport=30304"));
    }
}
