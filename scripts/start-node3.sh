#!/bin/bash

# Pasifika PoA Chain - Node 3
# Run this on a third machine to join the network

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

echo "🌺 Pasifika PoA Chain - Node 3"
echo "=============================="
echo ""

if [ ! -f "./target/release/reth" ]; then
    echo "❌ Reth not built. Please run: cargo build --release --bin reth"
    exit 1
fi

# Initialize if needed
if [ ! -d "./pasifika-node3-data" ]; then
    echo "📁 Initializing Node 3..."
    mkdir -p ./pasifika-node3-data
    ./target/release/reth init \
      --datadir ./pasifika-node3-data \
      --chain ./pasifika-poa/genesis.json
fi

# Get LAN IP for display
LAN_IP=$(./scripts/get-ip.sh 2>/dev/null || echo "localhost")

echo ""
echo "Node 3 Configuration:"
echo "  - Role: Validator Node"
echo "  - HTTP RPC: http://$LAN_IP:8547"
echo "  - P2P Port: 30305"
echo "  - Chain ID: 999888"
echo "  - Gas Fees: ZERO"
echo ""
echo "To connect to Node 1, set BOOTNODE environment variable:"
echo "  export BOOTNODE=\"enode://<node1-id>@<node1-ip>:30303\""
echo ""
echo "Press Ctrl+C to stop the node"
echo ""

# Build bootnode argument if provided
BOOTNODE_ARG=""
if [ -n "$BOOTNODE" ]; then
    BOOTNODE_ARG="--bootnodes $BOOTNODE"
    echo "Connecting to bootnode: $BOOTNODE"
fi

./target/release/reth node \
  --datadir ./pasifika-node3-data \
  --chain ./pasifika-poa/genesis.json \
  --dev \
  --dev.block-time 1s \
  --http \
  --http.api all \
  --http.addr 0.0.0.0 \
  --http.port 8547 \
  --http.corsdomain "*" \
  --port 30305 \
  --gpo.blocks 1 \
  --gpo.percentile 1 \
  --gpo.maxprice 0 \
  --gpo.ignoreprice 0 \
  $BOOTNODE_ARG
