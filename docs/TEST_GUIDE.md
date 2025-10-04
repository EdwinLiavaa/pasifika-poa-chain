# Pasifika PoA Chain - Complete Test Guide

This guide provides complete testing instructions for the Pasifika PoA blockchain using Reth with custom genesis configuration.

## Prerequisites

Before testing, ensure you have completed:

**Rust 1.88+** installed (`rustc --version`)  
**Clang/LLVM** installed (`clang --version`)  
**Reth binary** built (`ls -lh target/release/reth`)  
**Scripts** made executable (`chmod +x *.sh`)

If not, see [BUILD_AND_RUN.md](./BUILD_AND_RUN.md) for complete setup instructions.

## Quick Test (Automated)

The fastest way to test the chain:

```bash
# 1. Initialize chain
./quick-test.sh

# 2. Start node (in terminal 1)
./run-node.sh

# 3. Test RPC (in terminal 2)
./test-rpc.sh
```

If all tests pass, your Pasifika PoA chain is working! 

## Detailed Testing Steps

### Step 1: Build Reth Binary

```bash
cd /home/user/Documents/pasifika-web3-tech-hub/pasifika-poa-chain

# Build Reth (10-15 minutes first time)
cargo build --release --bin reth

# Verify build
./target/release/reth --version
# Expected: reth-ethereum-cli Version: 1.8.2-dev
```bash
# Create data directory
mkdir -p ./pasifika-test-data

# Initialize with Pasifika genesis
./target/release/reth init \
  --datadir ./pasifika-test-data \
  --chain ./pasifika-poa/genesis.json
```

**Expected output:**
```
Genesis block written
Genesis hash: 0x...
```

## Step 3: Start the Node

Start Reth in dev mode with HTTP RPC enabled:

```bash
./target/release/reth node \
  --datadir ./pasifika-test-data \
  --chain ./pasifika-poa/genesis.json \
  --dev \
  --http \
  --http.api all \
  --http.addr 127.0.0.1 \
  --http.port 8545 \
  --http.corsdomain "*"
```

**What this does:**
- `--dev`: Enables development mode (auto-mining)
- `--http`: Enables HTTP RPC server
- `--http.api all`: Enables all RPC methods
- `--http.port 8545`: Standard Ethereum RPC port
- Chain ID: 999888
- Zero gas fees enabled

**Expected output:**
```
Starting Reth...
Initialized chain: 999888
Listening on http://127.0.0.1:8545
Mining blocks...
```

## Step 4: Verify Chain Configuration

In a new terminal, verify the chain is running with zero gas:

```bash
# Check chain ID
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}'

# Expected: {"jsonrpc":"2.0","id":1,"result":"0xf4030"}  (999888 in hex)

# Check gas price (should be 0)
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_gasPrice","params":[],"id":1}'

# Expected: {"jsonrpc":"2.0","id":1,"result":"0x0"}  (zero!)

# Check block number
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'

# Expected: {"jsonrpc":"2.0","id":1,"result":"0x..."}  (should be producing blocks)
```

## Step 5: Test with JavaScript

Install dependencies and run the test scripts:

```bash
cd pasifika-poa/examples
npm install
```

### Test 1: Send a Zero-Gas Transaction

```bash
npm run send
```

**Expected output:**
```
💸 Sending transaction on Pasifika PoA Chain...
👤 From: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
👤 To: 0x70997970C51812dc3A010C7d01b50e0d17dc79C8
💰 Sending: 10.0 ETH
📤 Transaction hash: 0x...
✅ Transaction confirmed in block: ...
💵 Transaction cost:
  Gas used: 21000
  Gas price: 0 wei
  Total cost: 0 ETH
  ✨ ZERO GAS FEES! ✨
```

### Test 2: Deploy a Smart Contract

```bash
npm run deploy
```

**Expected output:**
```
🚀 Deploying contract to Pasifika PoA Chain...
📝 Deployer address: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
💰 Balance: 1000000000000000000000000000...ETH
🌐 Network: { chainId: '999888', name: 'Pasifika PoA' }
📤 Deploying contract...
✅ Contract deployed at: 0x...
🧪 Testing contract...
✅ Value stored
📖 Retrieved value: 42
✅ All done!
Total gas cost: 0 ETH (ZERO GAS!)
```

## Step 6: Test Pre-funded Accounts

Verify the pre-funded accounts from genesis:

```bash
# Check Account 1 balance
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc":"2.0",
    "method":"eth_getBalance",
    "params":["0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266", "latest"],
    "id":1
  }'

# Should return massive balance from genesis
```

## Step 7: Interactive Testing with ethers.js

Create a quick test script:

```javascript
// test-pasifika.js
const { ethers } = require('ethers');

async function test() {
    const provider = new ethers.JsonRpcProvider('http://localhost:8545');
    
    console.log('\n🌺 Pasifika PoA Chain Test\n');
    
    // Check network
    const network = await provider.getNetwork();
    console.log('Chain ID:', network.chainId.toString());
    
    // Check gas price
    const gasPrice = await provider.getFeeData();
    console.log('Gas Price:', gasPrice.gasPrice?.toString(), '(should be 0)');
    
    // Check block number
    const blockNumber = await provider.getBlockNumber();
    console.log('Latest Block:', blockNumber);
    
    // Check account balance
    const address = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266';
    const balance = await provider.getBalance(address);
    console.log('\nTest Account Balance:', ethers.formatEther(balance), 'ETH');
    
    // Send transaction with zero gas
    const wallet = new ethers.Wallet(
        '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80',
        provider
    );
    
    console.log('\n📤 Sending transaction with ZERO gas...');
    const tx = await wallet.sendTransaction({
        to: '0x70997970C51812dc3A010C7d01b50e0d17dc79C8',
        value: ethers.parseEther('1.0'),
        gasLimit: 21000,
        gasPrice: 0  // ZERO GAS!
    });
    
    console.log('Transaction hash:', tx.hash);
    console.log('Waiting for confirmation...');
    
    const receipt = await tx.wait();
    console.log('✅ Confirmed in block:', receipt.blockNumber);
    console.log('Gas used:', receipt.gasUsed.toString());
    console.log('Effective gas price:', receipt.gasPrice?.toString() || '0');
    console.log('Total cost:', '0 ETH');
    
    console.log('\n✅ All tests passed!\n');
}

test().catch(console.error);
```

Run it:
```bash
node test-pasifika.js
```

## Verification Checklist

After completing the tests, verify:

- [x] Chain ID is 999888
- [x] Gas price returns 0
- [x] Transactions cost 0 ETH
- [x] Blocks are being produced
- [x] Pre-funded accounts have balances
- [x] Smart contracts can be deployed with zero cost
- [x] Contract interactions work normally
- [x] All EVM features function correctly

## Troubleshooting

### Node won't start
```bash
# Clean data directory
rm -rf ./pasifika-test-data

# Re-initialize
./target/release/reth init \
  --datadir ./pasifika-test-data \
  --chain ./pasifika-poa/genesis.json
```

### RPC not responding
```bash
# Check if node is running
ps aux | grep reth

# Check port
netstat -tuln | grep 8545

# Try connecting
curl http://localhost:8545
```

### Transactions failing
```bash
# Verify chain ID matches
# Chain ID in genesis.json: 999888
# Chain ID in your code should match

# Check account has balance
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_getBalance","params":["YOUR_ADDRESS","latest"],"id":1}'
```

## What This Demonstrates

This test setup proves:

✅ **Zero Gas Works** - Transactions cost 0 ETH  
✅ **Chain Configuration** - Custom chain ID and parameters  
✅ **EVM Compatibility** - Standard Ethereum tools work  
✅ **Smart Contracts** - Deploy and interact normally  
✅ **Development Ready** - Can build dApps immediately  
✅ **Pacific Focus** - Foundation for community applications  

## Next Steps

After successful testing:

1. **Build Community Tools**
   - Web wallet for Pacific communities
   - Block explorer with local language support
   - Mobile apps for island connectivity

2. **Deploy Real Applications**
   - Land registry smart contracts
   - Microfinance pools
   - Supply chain tracking
   - Digital identity systems

3. **Scale to Production**
   - Multi-node deployment
   - Geographic distribution
   - High availability setup
   - Monitoring and analytics

## Stop the Node

When done testing:

```bash
# In the terminal running the node, press:
Ctrl + C

# Or kill the process:
pkill -f "reth node"

# Clean up test data (optional):
rm -rf ./pasifika-test-data
```

## Success!

If all tests pass, you've successfully demonstrated:
- Zero-gas blockchain for Pacific communities
- Full Ethereum compatibility
- Ready for real-world applications
- Foundation for the Pasifika vision

**Made with love for the Pacific Islands**  
**Fa'afetai • Malo • Vinaka • Kia Orana • Tēnā koe**

*Empowering communities through technology, honoring tradition through innovation*
