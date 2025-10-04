# Build and Run Guide - Pasifika PoA Chain

Complete step-by-step instructions to build, test, and run your PoA blockchain.

## System Requirements

### Minimum Requirements
- **CPU**: 2 cores
- **RAM**: 4GB minimum, 8GB recommended
- **Disk**: 10GB free space for build + blockchain data
- **OS**: Linux (Ubuntu 20.04+, Debian 11+) or macOS 12+
- **Internet**: Broadband connection for downloading dependencies

### Software Requirements
- **Rust**: 1.88.0 or later (REQUIRED)
- **Clang/LLVM**: 15.0 or later (REQUIRED for libmdbx)
- **Build tools**: gcc, make, pkg-config
- **SSL**: OpenSSL development libraries

## Complete Prerequisites Installation

### Step 1: Install Rust 1.88+

```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow prompts, then source the environment
source $HOME/.cargo/env

# IMPORTANT: Update to latest stable (1.88+)
rustup update stable

# Verify version - MUST BE 1.88.0 or later
rustc --version
```

**Expected output:**
```
rustc 1.90.0 (1159e78c4 2025-09-14)
```

**If you see version 1.86 or earlier:**
```bash
rustup update stable
rustup default stable
rustc --version  # Should now show 1.88+
```

### Step 2: Install Build Dependencies

**For Ubuntu/Debian:**
```bash
# Update package list
sudo apt-get update

# Install essential build tools
sudo apt-get install -y build-essential pkg-config libssl-dev

# Install clang and LLVM (CRITICAL for libmdbx compilation)
sudo apt-get install -y clang libclang-dev

# Verify installations
gcc --version       # Should show gcc 9.0+
clang --version     # Should show clang 15.0+
pkg-config --version
```

**For macOS:**
```bash
# Install Xcode Command Line Tools (includes clang)
xcode-select --install

# Or if you prefer Homebrew
brew install llvm openssl pkg-config

# Add LLVM to PATH if using Homebrew
echo 'export PATH="/opt/homebrew/opt/llvm/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### Step 3: Verify All Prerequisites

```bash
# Run these checks to ensure everything is ready
echo "Rust version:"
rustc --version     # Must be 1.88.0+

echo "Cargo version:"
cargo --version     # Should match Rust

echo "Clang version:"
clang --version     # Must be 15.0+

echo "GCC version:"
gcc --version       # Should be 9.0+

echo "OpenSSL:"
pkg-config --modversion openssl  # Should show version
```

**All checks must pass before proceeding!**

## Building Reth with Pasifika Genesis

### Step 1: Navigate to Project Root

```bash
# Go to the main project directory (NOT pasifika-poa subdirectory)
cd /home/user/Documents/pasifika-web3-tech-hub/pasifika-poa-chain

# Verify you're in the right place
ls -la | grep -E "Cargo.toml|crates|pasifika-poa"
# Should show Cargo.toml, crates/ directory, and pasifika-poa/ directory
```

### Step 2: Clean Previous Builds (Optional)

```bash
# If you had previous build attempts, clean them
cargo clean

# This removes the target/ directory (~1GB if it exists)
```

### Step 3: Build Reth Binary

```bash
# Build Reth in release mode
# FIRST BUILD TAKES 10-15 MINUTES
cargo build --release --bin reth
```

**What happens during build:**
1. **Dependency Download** (2-3 min): Downloads ~400 Rust crates
2. **libmdbx Compilation** (1-2 min): Compiles database engine using clang
3. **Reth Core** (5-8 min): Compiles blockchain engine
4. **Optimization** (2-3 min): Creates optimized release binary
5. **Final Binary**: `target/release/reth` (~60-70MB)

**Expected output (abbreviated):**
```
   Compiling proc-macro2 v1.0.70
   Compiling unicode-ident v1.0.12
   ...
   Compiling reth-mdbx-sys v1.8.2  ← Database engine (uses clang)
   Compiling reth-primitives v1.8.2
   Compiling reth-network v1.8.2
   ...
   Compiling reth v1.8.2           ← Main binary
    Finished release [optimized] target(s) in 12m 45s
```

### Common Build Errors and Fixes

#### Error: "rustc 1.86.0 is not supported"

**Cause**: Rust version too old

**Fix**:
```bash
rustup update stable
rustup default stable
rustc --version  # Verify it's 1.88+
cargo build --release --bin reth
```

#### Error: "fatal error: 'stdarg.h' file not found"

**Cause**: Missing clang development headers

**Fix**:
```bash
# Ubuntu/Debian
sudo apt-get install -y clang libclang-dev

# Verify installation
clang --version

# Retry build
cargo clean
cargo build --release --bin reth
```

#### Error: "Could not find libssl"

**Cause**: Missing OpenSSL development libraries

**Fix**:
```bash
sudo apt-get install -y libssl-dev pkg-config
cargo build --release --bin reth
```

#### Error: Out of Memory

**Cause**: System has < 4GB RAM

**Fix**:
```bash
# Build with fewer parallel jobs
cargo build --release --bin reth -j 2

# Or add swap space
sudo fallocate -l 4G /swapfile
sudo chmod 600 /swapfile  
sudo mkswap /swapfile
sudo swapon /swapfile
```

### Step 4: Verify Build Success

```bash
# Check binary exists and size
ls -lh target/release/reth

# Expected output:
# -rwxrwxr-x 2 user user 68M Oct 4 13:06 target/release/reth
```

**Binary should be 60-70MB in size.**

```bash
# Test the binary
./target/release/reth --version
```

**Expected output:**
```
reth-ethereum-cli Version: 1.8.2-dev
Commit SHA: 78535b0747692f9bb20b2b1eca4c00346f4d8921
Build Timestamp: 2025-10-03T23:54:44.379517780Z
Build Features: jemalloc
Build Profile: release
```

✅ **If you see this output, the build was successful!**

## Initialize Pasifika PoA Chain

### Step 1: Prepare Test Scripts

```bash
# Make all scripts executable
chmod +x quick-test.sh run-node.sh test-rpc.sh

# Verify genesis configuration exists
ls -la pasifika-poa/genesis.json
# Should show the genesis file with zero-gas configuration
```

### Step 2: Initialize Chain

```bash
# Run the initialization script
./quick-test.sh
```

**Expected output:**
```
🌺 Pasifika PoA Chain - Quick Test
==================================

✅ Reth binary found

🧹 Cleaning previous test data...
📁 Creating data directory...
🔧 Initializing Pasifika PoA chain...

Genesis block written
Genesis hash: 0x1234567890abcdef...

✅ Chain initialized successfully!

📊 Chain Configuration:
  - Chain ID: 999888
  - Consensus: Proof-of-Authority
  - Gas Fees: ZERO
  - Pre-funded accounts: 10

🚀 To start the node, run:

  ./run-node.sh
```

**What this does:**
- Creates `./pasifika-test-data` directory
- Initializes blockchain with Pasifika genesis.json
- Sets up Chain ID 999888
- Configures zero-gas transactions
- Pre-funds 10 test accounts

## Run the Node

### Terminal 1: Start Pasifika PoA Node

```bash
# Start the node
./run-node.sh
```

**Expected output:**
```
🌺 Starting Pasifika PoA Node...

Node Configuration:
  - HTTP RPC: http://localhost:8545
  - Chain ID: 999888
  - Gas Fees: ZERO
  - Dev Mode: Enabled (auto-mining)

Press Ctrl+C to stop the node

Starting in 3 seconds...

2025-10-04T00:15:32.123456Z  INFO reth::cli: Starting Reth
2025-10-04T00:15:32.234567Z  INFO reth::cli: Initialized chain: 999888
2025-10-04T00:15:32.345678Z  INFO reth::cli: HTTP RPC listening on 127.0.0.1:8545
2025-10-04T00:15:32.456789Z  INFO reth::cli: Auto-mining enabled
2025-10-04T00:15:33.567890Z  INFO reth::cli: Block #1 mined
Data Dir: ./data/node1

    Finished release [optimized] target(s)
     Running `target/release/pasifika-poa --node-id node1 ...`

2025-10-04T10:47:11.123456Z  INFO pasifika_poa: Starting Pasifika PoA Node: node1
2025-10-04T10:47:11.234567Z  INFO pasifika_poa: HTTP RPC Port: 8545
2025-10-04T10:47:11.345678Z  INFO pasifika_poa: P2P Port: 30303
2025-10-04T10:47:12.456789Z  INFO pasifika_poa: ✅ Node started successfully!
2025-10-04T10:47:12.567890Z  INFO pasifika_poa: HTTP RPC: http://localhost:8545
2025-10-04T10:47:12.678901Z  INFO pasifika_poa: Chain ID: 999888
2025-10-04T10:47:12.789012Z  INFO pasifika_poa: This node is configured as a validator
2025-10-04T10:47:12.890123Z  WARN pasifika_poa: ⚠️  Gas fees are set to ZERO
2025-10-04T10:47:12.901234Z  WARN pasifika_poa: ⚠️  This is a PoA network - all connected nodes validate blocks
```

**Node is running when you see:**
- ✅ "Node started successfully!"
- HTTP RPC URL displayed
- No error messages

## Step 4: Test the Node

### Terminal 2: Run Tests

```bash
# In a new terminal
cd /home/user/Documents/pasifika-web3-tech-hub/pasifika-poa-chain/pasifika-poa

# Run the test script
./scripts/test-network.sh
```

**Expected output:**
```
🧪 Testing Pasifika PoA Network
================================

1️⃣  Testing node connectivity...
✅ Node is running
   Chain ID: 0xf4030 (999888)

2️⃣  Testing block production...
✅ Latest block: 0x5 (5)

3️⃣  Testing zero gas configuration...
✅ Gas price is 0 (zero gas fees enabled)

4️⃣  Testing pre-funded accounts...
✅ Pre-funded account has balance
   Address: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
   Balance: 0x200000000000000000000000000000000000000000000000000000000000000

5️⃣  Testing client version...
✅ Client: reth/v1.x.x

================================
✅ All tests passed!

Network is ready for use:
  - Chain ID: 999888
  - RPC URL: http://localhost:8545
  - Zero gas: Enabled
  - Consensus: Proof-of-Authority
```

## Step 5: Send Your First Transaction

### Option A: Using curl

```bash
# Check balance
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc":"2.0",
    "method":"eth_getBalance",
    "params":["0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266", "latest"],
    "id":1
  }'

# Expected: Huge balance in hex
```

### Option B: Using JavaScript

```bash
# Install dependencies
cd examples/
npm install

# Send a transaction (ZERO GAS!)
npm run send
```

**Expected output:**
```
💸 Sending transaction on Pasifika PoA Chain...

👤 From: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
👤 To: 0x70997970C51812dc3A010C7d01b50e0d17dc79C8

📊 Balances before:
  Sender: 1000000000000000000000000000000000000000000000000000000000000.0 ETH
  Receiver: 1000000000000000000000000000000000000000000000000000000000000.0 ETH

💰 Sending: 10.0 ETH
📤 Transaction hash: 0xabc123...
⏳ Waiting for confirmation...
✅ Transaction confirmed in block: 6

📊 Balances after:
  Sender: 999999999999999999999999999999999999999999999999999999999990.0 ETH
  Receiver: 1000000000000000000000000000000000000000000000000000000000010.0 ETH

💵 Transaction cost:
  Gas used: 21000
  Gas price: 0 wei
  Total cost: 0 ETH
  ✨ ZERO GAS FEES! ✨

✅ Transfer verified:
  Sender lost: 10.0 ETH (+ 0 gas)
  Receiver gained: 10.0 ETH
```

### Option C: Using Python

```bash
# Install dependencies
pip install web3

# Create a test script
cat > test_transaction.py << 'EOF'
from web3 import Web3

w3 = Web3(Web3.HTTPProvider('http://localhost:8545'))

print(f"Connected: {w3.is_connected()}")
print(f"Chain ID: {w3.eth.chain_id}")
print(f"Latest Block: {w3.eth.block_number}")

# Pre-funded account
account = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266'
balance = w3.eth.get_balance(account)
print(f"Balance: {w3.from_wei(balance, 'ether')} ETH")
EOF

python test_transaction.py
```

## Step 6: Deploy a Smart Contract

```bash
cd examples/

# Deploy a contract (ZERO GAS!)
npm run deploy
```

**Expected output:**
```
🚀 Deploying contract to Pasifika PoA Chain...

📝 Deployer address: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
💰 Balance: 1000000000000000000000000000000000000000000000000000000000000.0 ETH

🌐 Network: { chainId: '999888', name: 'Pasifika PoA' }

📤 Deploying contract...
⏳ Waiting for deployment...
✅ Contract deployed at: 0x5FbDB2315678afecb367f032d93F642f64180aa3

🧪 Testing contract...
📝 Storing value 42...
✅ Value stored
📖 Retrieved value: 42

✅ All done!
Contract address: 0x5FbDB2315678afecb367f032d93F642f64180aa3
Total gas cost: 0 ETH (ZERO GAS!)
```

## Step 7: Run Multiple Nodes (Optional)

### Terminal 1: Node 1 (Already Running)
```bash
# Keep running from Step 3
```

### Terminal 2: Get Node 1's Enode

Look at Node 1's logs for a line like:
```
enode://abc123def456...@127.0.0.1:30303
```

Copy this enode URL.

### Terminal 3: Edit Node 2 Script

```bash
# Edit the script
nano scripts/start-node2.sh

# Find this line:
BOOTNODE="enode://[NODE1_ENODE]@127.0.0.1:30303"

# Replace [NODE1_ENODE] with the actual enode from Node 1
BOOTNODE="enode://abc123def456...@127.0.0.1:30303"

# Save and exit (Ctrl+X, Y, Enter)
```

### Terminal 3: Start Node 2

```bash
./scripts/start-node2.sh
```

### Terminal 4: Start Node 3 (Optional)

```bash
# Edit bootnode
nano scripts/start-node3.sh
# Replace [NODE1_ENODE] with actual enode

# Start node
./scripts/start-node3.sh
## Step 9: Stop the Nodes

```bash
# In each terminal, press:
Ctrl + C

# Or kill all nodes:
pkill -f pasifika-poa
```

## Step 10: Clean Data (Optional)

```bash
# Remove all blockchain data
./scripts/clean-data.sh

# Confirm when prompted
# This will delete all blocks, transactions, and state
```

## Docker Alternative

If you prefer Docker:

### Build Docker Image

```bash
docker build -t pasifika-poa -f Dockerfile ..
```

### Run Single Node

```bash
docker run -p 8545:8545 -p 30303:30303 pasifika-poa
```

### Run Multi-Node with Docker Compose

```bash
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f node1

# Stop all nodes
docker-compose down
```

## Troubleshooting

### Problem: Build fails with "linker error"

```bash
# Install build tools
sudo apt-get install build-essential pkg-config libssl-dev

# Install clang
sudo apt-get install clang

# Try again
cargo build --release
```

### Problem: Port 8545 already in use

```bash
# Find what's using the port
lsof -i :8545

# Kill it
kill -9 <PID>

# Or use a different port
./target/release/pasifika-poa --http-port 9545
```

### Problem: Node crashes immediately

```bash
# Run with debug logging
RUST_LOG=debug ./target/release/pasifika-poa --node-id node1 --dev

# Check the error message
# Common issues:
# - Port in use
# - Data directory permissions
# - Corrupted database (clean data and restart)
```

### Problem: Transactions not confirming

```bash
# Check if node is producing blocks
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'

# Check dev mode is enabled
# Should see auto-mining in logs

# Restart node in dev mode
./scripts/start-node1.sh
```

### Problem: Can't connect to RPC

```bash
# Verify node is running
ps aux | grep pasifika-poa

# Check port
netstat -tuln | grep 8545

# Test connection
curl http://localhost:8545

# Should return: {"error":"invalid request"}
# (This is OK - means RPC is listening)
```

## Next Steps

Now that your blockchain is running:

1. **Build dApps** - Connect with ethers.js, web3.py, etc.
2. **Deploy Contracts** - Use Hardhat, Foundry, or Remix
3. **Customize** - Modify genesis, consensus, or P2P logic
4. **Scale** - Add more nodes, optimize performance
5. **Monitor** - Add logging, metrics, alerting

## Quick Reference

### Common Commands

```bash
# Build
cargo build --release

# Run node
./scripts/start-node1.sh

# Test network
./scripts/test-network.sh

# Clean data
./scripts/clean-data.sh

# Run tests
cargo test

# Send transaction
cd examples/ && npm run send

# Deploy contract
cd examples/ && npm run deploy
```

### Important Files

- `src/genesis.rs` - Chain configuration
- `src/consensus.rs` - PoA logic
- `src/p2p.rs` - Networking
- `genesis.json` - Genesis block
- `Cargo.toml` - Dependencies

### RPC Endpoints

- **Node 1**: http://localhost:8545
- **Node 2**: http://localhost:8546
- **Node 3**: http://localhost:8547

### Test Accounts

All have huge balances:

```
0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
Private Key: 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

0x70997970C51812dc3A010C7d01b50e0d17dc79C8
Private Key: 0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d
```

⚠️ **Never use these keys with real funds!**

## Support

- **README.md** - Features and overview
- **QUICKSTART.md** - 5-minute guide
- **CONFIG.md** - Configuration options
- **IMPLEMENTATION.md** - Technical details
- **PROJECT_SUMMARY.md** - Complete overview

---

**Congratulations!** 🎉

You've successfully built and run your own Proof-of-Authority blockchain with:
- ✅ Zero gas fees
- ✅ All nodes as validators
- ✅ Custom P2P discovery
- ✅ Full EVM compatibility

**Happy Building!** 🚀
