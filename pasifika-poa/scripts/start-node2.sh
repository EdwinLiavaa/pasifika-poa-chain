#!/bin/bash

# Start Node 2

NODE_ID="node2"
HTTP_PORT=8546
P2P_PORT=30304
DATA_DIR="./data/${NODE_ID}"
BOOTNODE="enode://[NODE1_ENODE]@127.0.0.1:30303"

echo "Starting Pasifika PoA Node 2"
echo "======================================"
echo "Node ID: ${NODE_ID}"
echo "HTTP RPC: http://localhost:${HTTP_PORT}"
echo "P2P Port: ${P2P_PORT}"
echo "Data Dir: ${DATA_DIR}"
echo "Bootnode: ${BOOTNODE}"
echo ""

# Create data directory if it doesn't exist
mkdir -p "${DATA_DIR}"

# Run the node
cargo run --release -- \
    --node-id "${NODE_ID}" \
    --http-port "${HTTP_PORT}" \
    --p2p-port "${P2P_PORT}" \
    --datadir "${DATA_DIR}" \
    --bootnodes "${BOOTNODE}" \
    --dev
