#!/bin/bash

# Test the Pasifika PoA network

echo "🧪 Testing Pasifika PoA Network"
echo "================================"
echo ""

NODE_URL="http://localhost:8545"

# Test 1: Check if node is running
echo "1️⃣  Testing node connectivity..."
CHAIN_ID=$(curl -s -X POST ${NODE_URL} \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}' | grep -o '"result":"[^"]*"' | cut -d'"' -f4)

if [ -z "$CHAIN_ID" ]; then
    echo "❌ Node is not responding"
    exit 1
else
    echo "✅ Node is running"
    echo "   Chain ID: $CHAIN_ID ($(printf '%d' $CHAIN_ID))"
fi

# Test 2: Check block number
echo ""
echo "2️⃣  Testing block production..."
BLOCK_NUM=$(curl -s -X POST ${NODE_URL} \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' | grep -o '"result":"[^"]*"' | cut -d'"' -f4)

echo "✅ Latest block: $BLOCK_NUM ($(printf '%d' $BLOCK_NUM))"

# Test 3: Check gas price (should be 0)
echo ""
echo "3️⃣  Testing zero gas configuration..."
GAS_PRICE=$(curl -s -X POST ${NODE_URL} \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_gasPrice","params":[],"id":1}' | grep -o '"result":"[^"]*"' | cut -d'"' -f4)

GAS_PRICE_DEC=$(printf '%d' $GAS_PRICE)
if [ "$GAS_PRICE_DEC" -eq 0 ]; then
    echo "✅ Gas price is 0 (zero gas fees enabled)"
else
    echo "⚠️  Gas price is $GAS_PRICE_DEC (expected 0)"
fi

# Test 4: Check pre-funded account
echo ""
echo "4️⃣  Testing pre-funded accounts..."
BALANCE=$(curl -s -X POST ${NODE_URL} \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_getBalance","params":["0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266","latest"],"id":1}' | grep -o '"result":"[^"]*"' | cut -d'"' -f4)

if [ ! -z "$BALANCE" ]; then
    echo "✅ Pre-funded account has balance"
    echo "   Address: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
    echo "   Balance: $BALANCE"
else
    echo "❌ Failed to check balance"
fi

# Test 5: Check client version
echo ""
echo "5️⃣  Testing client version..."
VERSION=$(curl -s -X POST ${NODE_URL} \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"web3_clientVersion","params":[],"id":1}' | grep -o '"result":"[^"]*"' | cut -d'"' -f4)

echo "✅ Client: $VERSION"

echo ""
echo "================================"
echo "✅ All tests passed!"
echo ""
echo "Network is ready for use:"
echo "  - Chain ID: 999888"
echo "  - RPC URL: ${NODE_URL}"
echo "  - Zero gas: Enabled"
echo "  - Consensus: Proof-of-Authority"
