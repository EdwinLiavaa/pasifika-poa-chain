# Pasifika PoA Chain

A custom Proof-of-Authority (PoA) blockchain built on Reth, designed for the Pasifika Web3 Tech Hub.

## Features

✅ **Proof-of-Authority Consensus**
- All connected nodes automatically become validators
- No mining or staking required
- Simplified consensus mechanism

✅ **Zero Gas Fees**
- `baseFeePerGas` set to `0x0`
- Extremely high gas limit for unlimited transactions
- Perfect for development and testing

✅ **Custom P2P Discovery**
- Automatic peer discovery
- All peers treated as validators
- Custom bootnode configuration

✅ **Built on Reth**
- Modular architecture
- High performance
- Production-ready foundation

## Architecture

### Core Components

1. **Genesis Configuration** (`src/genesis.rs`)
   - Chain ID: 999888
   - Zero gas configuration
   - Pre-funded test accounts
   - All EVM hardforks enabled

2. **PoA Consensus** (`src/consensus.rs`)
   - Automatic validator registration
   - All nodes are validators
   - Simplified block validation

3. **P2P Discovery** (`src/p2p.rs`)
   - Custom peer discovery
   - Auto-validator registration on peer connection
   - Bootnode management

## Installation

### Prerequisites

- Rust 1.88.0 or later
- Git

### Build from Source

```bash
# Clone the repository
cd /home/user/Documents/pasifika-web3-tech-hub/pasifika-poa-chain/pasifika-poa

# Build the project
cargo build --release

# Run tests
cargo test
```

## Usage

### Starting a Single Node

```bash
cargo run --release -- \
    --node-id node1 \
    --http-port 8545 \
    --p2p-port 30303 \
    --dev
```

### Starting Multiple Nodes

#### Node 1 (Bootstrap Node)

```bash
chmod +x scripts/start-node1.sh
./scripts/start-node1.sh
```

#### Node 2

```bash
chmod +x scripts/start-node2.sh
./scripts/start-node2.sh
```

#### Node 3

```bash
chmod +x scripts/start-node3.sh
./scripts/start-node3.sh
```

### Command Line Options

| Option | Description | Default |
|--------|-------------|---------|
| `--node-id` | Node identifier | `node1` |
| `--http-port` | HTTP RPC port | `8545` |
| `--p2p-port` | P2P network port | `30303` |
| `--bootnodes` | Comma-separated enode URLs | None |
| `--datadir` | Data directory | Auto-generated |
| `--dev` | Enable dev mode (auto-mining) | `true` |

## Network Configuration

### Chain Specification

- **Chain ID**: 999888
- **Network ID**: 999888
- **Consensus**: Proof-of-Authority (PoA)
- **Block Time**: ~1 second (dev mode)
- **Gas Limit**: Effectively unlimited
- **Base Fee**: 0 (zero gas)

### Pre-funded Accounts

The genesis block includes 10 pre-funded accounts with massive balances:

```
0x6Be02d1d3665660d22FF9624b7BE0551ee1Ac91b
0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
0x70997970C51812dc3A010C7d01b50e0d17dc79C8
... and 7 more
```

Each account has a balance of:
```
0x200000000000000000000000000000000000000000000000000000000000000
```

## RPC Endpoints

### Standard Ethereum JSON-RPC

All nodes expose standard Ethereum JSON-RPC endpoints:

```bash
# Check node version
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"web3_clientVersion","params":[],"id":1}'

# Get chain ID
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}'

# Get block number
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'
```

### Connecting with Web3 Libraries

#### JavaScript (ethers.js)

```javascript
const { ethers } = require('ethers');

const provider = new ethers.JsonRpcProvider('http://localhost:8545');
const chainId = await provider.getNetwork();
console.log('Chain ID:', chainId.chainId); // 999888
```

#### Python (web3.py)

```python
from web3 import Web3

w3 = Web3(Web3.HTTPProvider('http://localhost:8545'))
print(f"Chain ID: {w3.eth.chain_id}")  # 999888
print(f"Latest Block: {w3.eth.block_number}")
```

## P2P Networking

### Bootnode Setup

1. Start the first node (bootstrap node)
2. Get its enode URL from logs
3. Use the enode URL as bootnode for other nodes

### Example Enode URL

```
enode://abc123...@127.0.0.1:30303?discport=30303
```

### Connecting Nodes

```bash
# Node 2 connecting to Node 1
cargo run --release -- \
    --node-id node2 \
    --http-port 8546 \
    --p2p-port 30304 \
    --bootnodes "enode://[NODE1_ENODE]@127.0.0.1:30303"
```

## Validator Management

### Automatic Validator Registration

In Pasifika PoA, **all nodes are automatically validators**. When a node:

1. Joins the network
2. Connects to at least one peer
3. Is automatically registered as a validator

No manual validator registration required!

### Checking Validators

```javascript
// Using ethers.js with custom RPC
// (You would need to add a custom RPC endpoint for this)
const validatorCount = await provider.send("poa_getValidatorCount", []);
console.log(`Active Validators: ${validatorCount}`);
```

## Zero Gas Configuration

### How It Works

1. **Genesis Configuration**
   - `baseFeePerGas` set to `0x0` in genesis
   - London fork activated at block 0
   - Gas calculations return 0

2. **Transaction Costs**
   - Gas price: 0
   - Gas limit: Can be any value
   - Total cost: Always 0 ETH

3. **Example Transaction**

```javascript
// All transactions have zero cost
const tx = await wallet.sendTransaction({
    to: "0x...",
    value: ethers.parseEther("1.0"),
    gasLimit: 21000  // Doesn't matter, cost is 0
});
```

## Development

### Project Structure

```
pasifika-poa/
├── Cargo.toml              # Rust dependencies
├── genesis.json            # Genesis configuration
├── src/
│   ├── main.rs            # Main entry point
│   ├── genesis.rs         # Genesis creation
│   ├── consensus.rs       # PoA consensus logic
│   └── p2p.rs             # P2P discovery
├── scripts/
│   ├── start-node1.sh     # Bootstrap node
│   ├── start-node2.sh     # Node 2
│   └── start-node3.sh     # Node 3
└── README.md              # This file
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_genesis_creation
```

### Adding Custom Features

1. **Modify Consensus**
   - Edit `src/consensus.rs`
   - Implement custom validation logic
   - Add validator rotation if needed

2. **Customize P2P**
   - Edit `src/p2p.rs`
   - Add custom peer discovery
   - Implement peer filtering

3. **Adjust Genesis**
   - Edit `src/genesis.rs`
   - Modify chain parameters
   - Add/remove pre-funded accounts

## Troubleshooting

### Port Already in Use

If you see "Address already in use" errors:

```bash
# Check which process is using the port
lsof -i :8545

# Kill the process
kill -9 <PID>
```

### Nodes Not Connecting

1. Check firewall settings
2. Verify bootnode enode URL
3. Ensure P2P ports are accessible
4. Check node logs for connection errors

### Database Issues

```bash
# Clean data directory
rm -rf data/

# Restart nodes
./scripts/start-node1.sh
```

## Security Considerations

⚠️ **WARNING**: This is a PoA chain designed for development and private networks.

### Not Suitable For:

- Public mainnet deployment (without modifications)
- Production financial applications
- Untrusted environments

### Suitable For:

- Development and testing
- Private consortium networks
- Educational purposes
- Internal blockchain applications

## Roadmap

### Current Status (v0.1.0)

- ✅ Basic PoA consensus
- ✅ Zero gas configuration
- ✅ Automatic validator registration
- ✅ P2P networking
- ✅ Multi-node support

### Planned Features

- [ ] Validator reputation system
- [ ] Advanced P2P discovery (DHT)
- [ ] Block explorer integration
- [ ] Monitoring dashboard
- [ ] Automated testing suite
- [ ] Docker support
- [ ] Kubernetes deployment

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE))
- MIT License ([LICENSE-MIT](../LICENSE-MIT))

at your option.

## Acknowledgments

- Built on [Reth](https://github.com/paradigmxyz/reth)
- Inspired by Clique (Geth PoA)
- Part of the Pasifika Web3 Tech Hub initiative

## Support

For questions and support:

- GitHub Issues: [Create an issue](https://github.com/pasifika-web3-tech-hub/pasifika-poa-chain/issues)
- Documentation: This README
- Community: Pasifika Web3 Tech Hub

---

**Made with ❤️ for the Pasifika Web3 Community**
