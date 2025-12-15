#!/bin/bash

# Pasifika PoA Chain - Node 1 (Bootstrap Node)
# This is the first node in the network that other nodes will connect to

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

echo "🌺 Pasifika PoA Chain - Node 1 (Bootstrap)"
echo "==========================================="
echo ""

if [ ! -f "./target/release/reth" ]; then
    echo "❌ Reth not built. Please run: cargo build --release --bin reth"
    exit 1
fi

# Initialize if needed
if [ ! -d "./pasifika-node1-data" ]; then
    echo "📁 Initializing Node 1..."
    mkdir -p ./pasifika-node1-data
    ./target/release/reth init \
      --datadir ./pasifika-node1-data \
      --chain ./pasifika-poa/genesis.json
fi

# Get LAN IP for display
LAN_IP=$(./scripts/get-ip.sh 2>/dev/null || echo "localhost")

echo ""
echo "Node 1 Configuration:"
echo "  - Role: Bootstrap Node"
echo "  - HTTP RPC: http://$LAN_IP:8545"
echo "  - P2P Port: 30303"
echo "  - Chain ID: 999888"
echo "  - Gas Fees: ZERO"
echo ""
echo "Other nodes should connect to this node's enode address."
echo "Press Ctrl+C to stop the node"
echo ""

./target/release/reth node \
  --datadir ./pasifika-node1-data \
  --chain ./pasifika-poa/genesis.json \
  --dev \
  --dev.block-time 1s \
  --http \
  --http.api all \
  --http.addr 0.0.0.0 \
  --http.port 8545 \
  --http.corsdomain "*" \
  --port 30303 \
  --gpo.blocks 1 \
  --gpo.percentile 1 \
  --gpo.maxprice 0 \
  --gpo.ignoreprice 0
