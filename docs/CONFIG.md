# Pasifika PoA Chain - Configuration Guide

This document explains all configuration options and how to customize your PoA blockchain.

## Genesis Configuration

Location: `src/genesis.rs`

### Chain Parameters

```json
{
  "config": {
    "chainId": 999888,              // Unique chain identifier
    "homesteadBlock": 0,            // Enable Homestead from genesis
    "eip150Block": 0,               // Enable EIP-150 from genesis
    "eip155Block": 0,               // Enable EIP-155 (replay protection)
    "eip158Block": 0,               // State trie clearing
    "byzantiumBlock": 0,            // Byzantium features
    "constantinopleBlock": 0,       // Constantinople features
    "petersburgBlock": 0,           // Petersburg features
    "istanbulBlock": 0,             // Istanbul features
    "berlinBlock": 0,               // Berlin features
    "londonBlock": 0,               // London (EIP-1559) - enables base fee
    "shanghaiTime": 0,              // Shanghai (withdrawals)
    "cancunTime": 0,                // Cancun (blobs)
    "terminalTotalDifficulty": 0,   // PoS transition (0 = immediate)
    "terminalTotalDifficultyPassed": true
  }
}
```

### Customizing Chain ID

To change the chain ID:

1. Edit `src/genesis.rs`
2. Find `"chainId": 999888`
3. Change to your desired value
4. Rebuild: `cargo build --release`

**Recommended ranges:**
- Development: 1337, 31337
- Private networks: 1000-999999
- Avoid: 1 (mainnet), 5 (goerli), 11155111 (sepolia)

### Zero Gas Configuration

Gas is set to zero through multiple mechanisms:

1. **Base Fee**: Set to `0x0` in genesis
2. **London Fork**: Activated at block 0
3. **Gas Price**: Transactions can specify 0 gas price

To modify:
```json
{
  "baseFeePerGas": "0x0",        // Keep at 0 for zero gas
  "gasLimit": "0xFFFFFFFFFFFF"   // Very high limit
}
```

### Pre-funded Accounts

Located in the `alloc` section:

```json
{
  "alloc": {
    "0xYourAddress": {
      "balance": "0x200000000000000000000000000000000000000000000000000000000000000"
    }
  }
}
```

**Balance in hex** = ~10^77 wei ≈ 10^59 ETH (effectively unlimited for testing)

To add more accounts:
1. Generate new address
2. Add to `alloc` in `src/genesis.rs`
3. Rebuild

### Genesis Extra Data

```json
{
  "extraData": "0x506173696669636120506f41"
}
```

This is "Pasifika PoA" in hex. To customize:
```bash
echo -n "Your Custom Text" | xxd -p
```

## Consensus Configuration

Location: `src/consensus.rs`

### Validator Auto-Registration

All nodes automatically become validators when they connect. This is controlled in the `PoAConsensus` implementation.

To disable auto-registration:
```rust
// In src/consensus.rs
pub async fn register_validator(&self, ...) -> Result<()> {
    // Add manual approval logic here
    if !self.is_approved(&address) {
        return Err(eyre::eyre!("Validator not approved"));
    }
    // ... rest of the code
}
```

### Block Validation

Current implementation accepts all blocks. To add validation:

```rust
// In src/consensus.rs
pub async fn validate_header(&self, header: &Header) -> Result<()> {
    // 1. Extract signer from header seal
    let signer = self.extract_signer(header)?;
    
    // 2. Check if signer is a validator
    if !self.is_validator(&signer).await {
        return Err(eyre::eyre!("Invalid signer"));
    }
    
    // 3. Check rotation/round-robin
    // ... your logic here
    
    Ok(())
}
```

## P2P Configuration

Location: `src/p2p.rs`

### Bootnode Configuration

Bootnodes are specified via command line:
```bash
--bootnodes "enode://abc@127.0.0.1:30303,enode://def@192.168.1.100:30303"
```

### Auto-Validator on Connect

When a peer connects, it's automatically marked as a validator:

```rust
// In src/p2p.rs
pub async fn on_peer_connected(&self, enode: String) -> Result<()> {
    // Auto-register as validator
    self.add_peer(enode.clone(), None, true).await?;
    Ok(())
}
```

To disable auto-validator:
```rust
// Change `true` to `false`
self.add_peer(enode.clone(), None, false).await?;
```

### Network Ports

Default ports:
- HTTP RPC: 8545
- P2P: 30303

To customize:
```bash
pasifika-poa \
  --http-port 9545 \
  --p2p-port 31313
```

## Node Configuration

### Command Line Arguments

All available options:

```bash
pasifika-poa \
  --node-id <string>        # Node identifier (default: node1)
  --http-port <number>      # HTTP RPC port (default: 8545)
  --p2p-port <number>       # P2P network port (default: 30303)
  --bootnodes <enodes>      # Comma-separated bootnode enodes
  --datadir <path>          # Data directory path
  --dev <bool>              # Dev mode with auto-mining (default: true)
```

### Dev Mode

Dev mode enables:
- Automatic block production
- Faster block times (~1 second)
- No need for multiple validators

To disable (production mode):
```bash
pasifika-poa --dev false
```

## Production Deployment

### Multi-Node Setup

For a production PoA network:

1. **Choose validator nodes**: Select trusted entities to run validators
2. **Generate keys**: Create unique keys for each validator
3. **Configure bootnodes**: Set up bootstrap nodes
4. **Network isolation**: Use VPN or private network
5. **Monitoring**: Set up logging and metrics

### Security Considerations

**DO NOT use in production without:**

1. ✅ Proper key management (HSM, encrypted storage)
2. ✅ Network firewalls and access control
3. ✅ DDoS protection
4. ✅ Monitoring and alerting
5. ✅ Backup and disaster recovery
6. ✅ Regular security audits

### Recommended Production Config

```bash
# Validator Node
pasifika-poa \
  --node-id "validator-001" \
  --http-port 8545 \
  --p2p-port 30303 \
  --bootnodes "enode://..." \
  --datadir "/var/lib/pasifika/data" \
  --dev false
```

## Performance Tuning

### Database Settings

Reth database is configured in the node builder. To optimize:

```rust
// In src/main.rs
let node_config = NodeConfig::test()
    .with_database_path("/fast/ssd/path") // Use SSD
    // Add more config options
```

### Memory Limits

Set via environment variables:
```bash
export PASIFIKA_MAX_MEMORY=8G
export PASIFIKA_CACHE_SIZE=2G
```

### Block Time

Controlled by dev mode. For custom block times, modify:

```rust
// In node configuration
.with_dev_block_time(Some(Duration::from_secs(3))) // 3 second blocks
```

## Monitoring

### JSON-RPC Endpoints

Standard endpoints available:
- `eth_chainId`
- `eth_blockNumber`
- `eth_getBalance`
- `eth_sendRawTransaction`
- `eth_getTransactionReceipt`
- All standard Ethereum JSON-RPC methods

### Health Check

```bash
# Simple health check
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'
```

### Metrics

To add Prometheus metrics:

```rust
// In Cargo.toml
reth-metrics = { path = "../crates/metrics" }

// In src/main.rs
use reth_metrics::*;
```

## Troubleshooting

### Reset Configuration

```bash
# Remove all data
rm -rf data/

# Rebuild
cargo clean
cargo build --release
```

### Change Chain ID After Deployment

⚠️ **Warning**: Changing chain ID after deployment creates a new chain!

1. Stop all nodes
2. Clean data: `rm -rf data/`
3. Modify `src/genesis.rs`
4. Rebuild: `cargo build --release`
5. Restart nodes

### Port Conflicts

```bash
# Check what's using a port
lsof -i :8545

# Use different port
pasifika-poa --http-port 9545
```

## Advanced Topics

### Custom Consensus Rules

Implement custom consensus by modifying `src/consensus.rs`:

```rust
pub async fn validate_block(&self, block: &Block) -> Result<()> {
    // Your custom validation logic
    // Examples:
    // - Check block interval
    // - Verify signer rotation
    // - Validate transaction limits
    Ok(())
}
```

### Adding RPC Methods

Add custom RPC methods in `src/main.rs`:

```rust
// Add to node setup
let custom_rpc = MyCustomRpc::new();
node.rpc_registry.register(custom_rpc);
```

### State Pruning

To save disk space:

```rust
// Configure pruning
let node_config = NodeConfig::test()
    .with_pruning(PruneConfig::default());
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `RUST_LOG` | Log level | `info` |
| `PASIFIKA_DATA_DIR` | Data directory | `./data` |
| `PASIFIKA_HTTP_PORT` | RPC port | `8545` |
| `PASIFIKA_P2P_PORT` | P2P port | `30303` |

Example:
```bash
export RUST_LOG=debug
export PASIFIKA_DATA_DIR=/mnt/ssd/pasifika
pasifika-poa --node-id node1
```

## Support

For configuration help:
- Check logs: `RUST_LOG=debug pasifika-poa ...`
- Review [README.md](README.md)
- Open an issue on GitHub
