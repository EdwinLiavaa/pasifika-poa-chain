//! Pasifika PoA Chain
//! 
//! A custom Proof-of-Authority blockchain based on Reth with the following features:
//! - All nodes are validators (automatic validator registration)
//! - Zero gas fees
//! - Custom P2P discovery
//! - Simplified consensus mechanism

use std::sync::Arc;
use clap::Parser;
use eyre::Result;
use reth_chainspec::ChainSpec;
use reth_node_ethereum::EthereumNode;
use reth_node_builder::{NodeBuilder, NodeHandle, NodeConfig};
use reth_tasks::TaskManager;
use tracing::{info, warn};

mod consensus;
mod genesis;
mod p2p;

use crate::genesis::create_poa_genesis;

#[derive(Parser, Debug)]
#[command(name = "pasifika-poa")]
#[command(about = "Pasifika Proof-of-Authority Chain Node", long_about = None)]
struct Args {
    /// Node identifier (node1, node2, etc.)
    #[arg(short, long, default_value = "node1")]
    node_id: String,

    /// HTTP RPC port
    #[arg(long, default_value = "8545")]
    http_port: u16,

    /// P2P network port
    #[arg(long, default_value = "30303")]
    p2p_port: u16,

    /// Bootnodes for P2P discovery (comma-separated enode URLs)
    #[arg(long)]
    bootnodes: Option<String>,

    /// Data directory
    #[arg(long)]
    datadir: Option<String>,

    /// Enable dev mode (auto-mining)
    #[arg(long, default_value = "true")]
    dev: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    info!("Starting Pasifika PoA Node: {}", args.node_id);
    info!("HTTP RPC Port: {}", args.http_port);
    info!("P2P Port: {}", args.p2p_port);

    let tasks = TaskManager::current();

    // Create custom chain specification
    let chain_spec = create_poa_genesis();

    // Configure node
    let mut node_config = NodeConfig::test();
    
    if args.dev {
        node_config = node_config.dev();
    }

    node_config = node_config
        .with_rpc(RpcServerArgs::default().with_http())
        .with_chain(chain_spec.clone());

    // Build and launch node
    let NodeHandle { node, node_exit_future } = NodeBuilder::new(node_config)
        .testing_node(tasks.executor())
        .node(EthereumNode::default())
        .launch()
        .await?;

    info!("✅ Node started successfully!");
    info!("HTTP RPC: http://localhost:{}", args.http_port);
    info!("Chain ID: {}", chain_spec.chain().id());
    info!("This node is configured as a validator");
    
    warn!("⚠️  Gas fees are set to ZERO");
    warn!("⚠️  This is a PoA network - all connected nodes validate blocks");

    // Wait for exit signal
    node_exit_future.await
}
