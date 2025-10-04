#!/bin/bash

# Start Pasifika PoA Node

echo "🌺 Starting Pasifika PoA Node..."
echo ""

if [ ! -f "./target/release/reth" ]; then
    echo "❌ Reth not built. Please run: cargo build --release --bin reth"
    exit 1
fi

if [ ! -d "./pasifika-test-data" ]; then
    echo "❌ Chain not initialized. Please run: ./quick-test.sh"
    exit 1
fi

echo "Node Configuration:"
echo "  - HTTP RPC: http://localhost:8545"
echo "  - Chain ID: 999888"
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
  --http.addr 127.0.0.1 \
  --http.port 8545 \
  --http.corsdomain "*" \
  --gpo.blocks 1 \
  --gpo.percentile 1 \
  --gpo.maxprice 0 \
  --gpo.ignoreprice 0
