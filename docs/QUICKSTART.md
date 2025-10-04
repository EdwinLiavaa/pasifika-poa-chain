# Pasifika PoA Chain - Quick Start Guide

Get your PoA blockchain running in minutes!

## System Requirements

- **Operating System**: Linux (Ubuntu/Debian) or macOS
- **RAM**: 4GB minimum, 8GB recommended
- **Disk Space**: 10GB free space
- **Internet**: Required for downloading dependencies

## Prerequisites Installation

### Step 1: Install Rust (1.88.0 or later)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Update to latest stable version
rustup update stable

# Verify installation (must be 1.88+)
rustc --version  # Should show 1.88.0 or later
cargo --version
```

### Step 2: Install Build Dependencies

**For Ubuntu/Debian:**
```bash
# Install required build tools
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev

# Install clang and LLVM (required for libmdbx)
sudo apt-get install -y clang libclang-dev

# Verify clang installation
clang --version  # Should show clang 15.0 or later
```

**For macOS:**
```bash
# Install Xcode Command Line Tools (includes clang)
xcode-select --install

# Or using Homebrew
brew install llvm
```

### Step 3: Verify Prerequisites

```bash
# Check all prerequisites
rustc --version    # Should be 1.88+ 
cargo --version    # Should match Rust version
clang --version    # Should be 15.0+
gcc --version      # Should be installed
```

## Building Reth

### Step 1: Navigate to Project Directory

```bash
cd /home/user/Documents/pasifika-web3-tech-hub/pasifika-poa-chain
```

### Step 2: Build Reth Binary

```bash
# Build Reth in release mode (FIRST BUILD TAKES 10-15 MINUTES)
cargo build --release --bin reth
```

**What happens during build:**
- Downloads ~400+ Rust dependencies
- Compiles libmdbx (database engine) with clang
- Compiles Reth core blockchain components
- Creates optimized binary at `target/release/reth`

**Expected output:**
```
   Compiling reth-mdbx-sys v1.8.2
   Compiling reth-primitives v1.8.2
   ...
   Compiling reth v1.8.2
    Finished release [optimized] target(s) in 12m 45s
```

**If build fails with missing dependencies:**
```bash
# Install additional dependencies
sudo apt-get install -y libclang-dev llvm-dev

# Clean and retry
cargo clean
cargo build --release --bin reth
```

### Step 3: Verify Build Success

```bash
# Check binary exists and size
ls -lh target/release/reth
# Should show ~60-70MB file

# Check version
./target/release/reth --version
# Should show: reth-ethereum-cli Version: 1.8.2-dev
```

## Initialize the Pasifika PoA Chain

### Step 1: Run Quick Test Script

```bash
# Make scripts executable
chmod +x quick-test.sh run-node.sh test-rpc.sh

# Initialize chain with Pasifika genesis
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
Genesis hash: 0x...

✅ Chain initialized successfully!

📊 Chain Configuration:
  - Chain ID: 999888
  - Consensus: Proof-of-Authority  
  - Gas Fees: ZERO
  - Pre-funded accounts: 10
```

## Start the Node

### Terminal 1: Run the Node

```bash
# Start Pasifika PoA node
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

Starting Reth...
Initialized chain: 999888  
Listening on http://127.0.0.1:8545
Mining blocks...
```

## Test the Chain

### Terminal 2: Run Test Script

Open a new terminal and run:

```bash
# Run automated tests
./test-rpc.sh
```

**Expected output:**
```
🧪 Testing Pasifika PoA Chain RPC
=================================

1️⃣  Testing Chain ID...
✅ Chain ID: 0xf4030 (999888 in decimal)

2️⃣  Testing Gas Price (should be 0)...
✅ Gas Price: 0 (ZERO GAS FEES!)

3️⃣  Testing Block Production...
✅ Latest Block: 0x15 (21 in decimal)

4️⃣  Testing Pre-funded Account...
✅ Account has balance: 0x...
   Address: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266

5️⃣  Testing Client Version...
✅ Client: reth/v1.8.2-dev

=================================
✅ ALL TESTS PASSED!

Your Pasifika PoA chain is ready:
  - Chain ID: 999888 ✅
  - Zero Gas: Enabled ✅
  - RPC: http://localhost:8545 ✅
```

### Manual Testing with curl

```bash
# Get chain ID
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}'
# Expected: {"jsonrpc":"2.0","id":1,"result":"0xf4030"}

# Get gas price (should be 0)
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_gasPrice","params":[],"id":1}'
# Expected: {"jsonrpc":"2.0","id":1,"result":"0x0"}

# Get latest block
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'
# Expected: {"jsonrpc":"2.0","id":1,"result":"0x..."}
```

## Step 4: Start Additional Nodes (Optional)

### Get Node 1's Enode

From Node 1's logs, copy the enode URL (looks like):
```
enode://abc123def456...@127.0.0.1:30303
```

### Update Bootnode in Scripts

Edit `scripts/start-node2.sh` and replace `[NODE1_ENODE]` with the actual enode.

### Start Node 2

```bash
# In a new terminal
./scripts/start-node2.sh
```

### Start Node 3

```bash
# In another new terminal
./scripts/start-node3.sh
```

## Step 5: Send a Transaction

### Using curl

```bash
# First, get a pre-funded account balance
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc":"2.0",
    "method":"eth_getBalance",
    "params":["0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266", "latest"],
    "id":1
  }'
```

### Using JavaScript (Node.js)

```javascript
// install: npm install ethers
const { ethers } = require('ethers');

async function main() {
    // Connect to node
    const provider = new ethers.JsonRpcProvider('http://localhost:8545');
    
    // Check connection
    const network = await provider.getNetwork();
    console.log('Connected to chain:', network.chainId);
    
    // Create wallet from private key (test account)
    const privateKey = '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80';
    const wallet = new ethers.Wallet(privateKey, provider);
    
    console.log('Wallet address:', wallet.address);
    
    // Check balance
    const balance = await provider.getBalance(wallet.address);
    console.log('Balance:', ethers.formatEther(balance), 'ETH');
    
    // Send transaction (ZERO GAS!)
    const tx = await wallet.sendTransaction({
        to: '0x70997970C51812dc3A010C7d01b50e0d17dc79C8',
        value: ethers.parseEther('1.0')
    });
    
    console.log('Transaction hash:', tx.hash);
    
    // Wait for confirmation
    const receipt = await tx.wait();
    console.log('Transaction confirmed in block:', receipt.blockNumber);
}

main().catch(console.error);
```

### Using Python (web3.py)

```python
# install: pip install web3
from web3 import Web3

# Connect to node
w3 = Web3(Web3.HTTPProvider('http://localhost:8545'))

# Check connection
print(f"Connected: {w3.is_connected()}")
print(f"Chain ID: {w3.eth.chain_id}")

# Test account (pre-funded in genesis)
account = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266'
private_key = '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80'

# Check balance
balance = w3.eth.get_balance(account)
print(f"Balance: {w3.from_wei(balance, 'ether')} ETH")

# Send transaction (ZERO GAS!)
tx = {
    'from': account,
    'to': '0x70997970C51812dc3A010C7d01b50e0d17dc79C8',
    'value': w3.to_wei(1, 'ether'),
    'gas': 21000,
    'gasPrice': 0,  # ZERO GAS!
    'nonce': w3.eth.get_transaction_count(account),
}

# Sign and send
signed_tx = w3.eth.account.sign_transaction(tx, private_key)
tx_hash = w3.eth.send_raw_transaction(signed_tx.rawTransaction)
print(f"Transaction hash: {tx_hash.hex()}")

# Wait for receipt
receipt = w3.eth.wait_for_transaction_receipt(tx_hash)
print(f"Transaction confirmed in block: {receipt['blockNumber']}")
```

## Common Commands

### Check Node Status

```bash
# Get latest block number
curl -s -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' | jq

# Get peer count (when p2p is fully integrated)
curl -s -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"net_peerCount","params":[],"id":1}' | jq
```

### Stop Nodes

```bash
# Find and kill node processes
pkill -f pasifika-poa

# Or use Ctrl+C in each terminal window
```

### Clean Data

```bash
# Remove all blockchain data
rm -rf data/

# Restart from genesis
./scripts/start-node1.sh
```

## Next Steps

1. **Deploy Smart Contracts**: Use Hardhat, Foundry, or Remix
2. **Build dApps**: Connect your web app with ethers.js or web3.js
3. **Customize**: Modify consensus, p2p, or genesis configs
4. **Monitor**: Add logging and metrics

## Test Accounts (Pre-funded)

All accounts have massive balances in genesis:

| Address | Private Key (DO NOT USE IN PRODUCTION) |
|---------|----------------------------------------|
| `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266` | `0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80` |
| `0x70997970C51812dc3A010C7d01b50e0d17dc79C8` | `0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d` |
| `0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC` | `0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a` |

⚠️ **These are well-known test keys. NEVER use them on mainnet or with real funds!**

## Troubleshooting

### Build Errors

```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

### Port Conflicts

```bash
# Find process using port 8545
lsof -i :8545

# Kill it
kill -9 <PID>
```

### Can't Connect to Node

1. Check if node is running: `ps aux | grep pasifika-poa`
2. Check logs in terminal where node is running
3. Try restarting the node

## Support

Need help? Check:
- Main [README.md](README.md) for detailed documentation
- [Issue Tracker](https://github.com/pasifika-web3-tech-hub/pasifika-poa-chain/issues)

---

**Happy Building! 🚀**
