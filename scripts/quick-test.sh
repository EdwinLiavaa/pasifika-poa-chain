#!/bin/bash

# Pasifika PoA Chain - Quick Test Script
# This script helps you quickly test the blockchain once Reth is built

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

echo "🌺 Pasifika PoA Chain - Quick Test"
echo "=================================="
echo ""

# Check if Reth is built
if [ ! -f "./target/release/reth" ]; then
    echo "❌ Reth binary not found at ./target/release/reth"
    echo ""
    echo "Please build Reth first:"
    echo "  cargo build --release --bin reth"
    echo ""
    exit 1
fi

echo "✅ Reth binary found"
echo ""

# Clean previous test data
if [ -d "./pasifika-test-data" ]; then
    echo "🧹 Cleaning previous test data..."
    rm -rf ./pasifika-test-data
fi

# Create data directory
echo "📁 Creating data directory..."
mkdir -p ./pasifika-test-data

# Initialize chain
echo "🔧 Initializing Pasifika PoA chain..."
./target/release/reth init \
  --datadir ./pasifika-test-data \
  --chain ./pasifika-poa/genesis.json

echo ""
echo "✅ Chain initialized successfully!"
echo ""
echo "📊 Chain Configuration:"
echo "  - Chain ID: 999888"
echo "  - Consensus: Proof-of-Authority"
echo "  - Gas Fees: ZERO"
echo "  - Pre-funded accounts: 10"
echo ""
echo "🚀 To start the node, run:"
echo ""
echo "  ./target/release/reth node \\"
echo "    --datadir ./pasifika-test-data \\"
echo "    --chain ./pasifika-poa/genesis.json \\"
echo "    --dev \\"
echo "    --http \\"
echo "    --http.api all \\"
echo "    --http.addr 127.0.0.1 \\"
echo "    --http.port 8545 \\"
echo "    --http.corsdomain \"*\""
echo ""
echo "📝 Or use the convenience script:"
echo "  ./scripts/run-node.sh"
echo ""
