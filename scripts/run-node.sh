#!/bin/bash

# Start Pasifika PoA Node

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

echo "Starting Pasifika PoA Node..."
echo ""

if [ ! -f "./target/release/reth" ]; then
    echo "❌ Reth not built. Please run: cargo build --release --bin reth"
    exit 1
fi

if [ ! -d "./pasifika-test-data" ]; then
    echo "❌ Chain not initialized. Please run: ./scripts/quick-test.sh"
    exit 1
fi

# Automatically detect LAN IP address for display
LAN_IP=$(./scripts/get-ip.sh)

echo "Node Configuration:"
echo "  - Binding: 0.0.0.0:8545 (all interfaces)"
echo "  - Local access: http://localhost:8545"
echo "  - Network access: http://$LAN_IP:8545"
echo "  - Chain ID: 676888 (MEIDECC Chain)"
echo "  - Gas Fees: ZERO"
echo "  - Dev Mode: Enabled (auto-mining)"
echo ""
echo "Press Ctrl+C to stop the node"
echo ""
echo "Starting in 3 seconds..."
sleep 3

./target/release/reth node \
  --datadir ./pasifika-test-data \
  --chain ./pasifika-poa/genesis.json \
  --dev \
  --dev.block-time 1s \
  --http \
  --http.api all \
  --http.addr 0.0.0.0 \
  --http.port 8545 \
  --http.corsdomain "*" \
  --gpo.blocks 1 \
  --gpo.percentile 1 \
  --gpo.maxprice 0 \
  --gpo.ignoreprice 0
