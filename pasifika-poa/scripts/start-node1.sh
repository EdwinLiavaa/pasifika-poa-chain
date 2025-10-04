#!/bin/bash

# Start Node 1 (Bootstrap Node)

NODE_ID="node1"
HTTP_PORT=8545
P2P_PORT=30303
DATA_DIR="./data/${NODE_ID}"

echo "Starting Pasifika PoA Node 1 (Bootstrap Node)"
echo "======================================"
echo "Node ID: ${NODE_ID}"
echo "HTTP RPC: http://localhost:${HTTP_PORT}"
echo "P2P Port: ${P2P_PORT}"
echo "Data Dir: ${DATA_DIR}"
echo ""

# Create data directory if it doesn't exist
mkdir -p "${DATA_DIR}"

# Run the node
cargo run --release -- \
    --node-id "${NODE_ID}" \
    --http-port "${HTTP_PORT}" \
    --p2p-port "${P2P_PORT}" \
    --datadir "${DATA_DIR}" \
    --dev
