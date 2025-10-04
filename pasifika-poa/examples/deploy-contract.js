#!/usr/bin/env node

/**
 * Example: Deploy a smart contract to Pasifika PoA Chain
 * 
 * Install dependencies:
 * npm install ethers
 * 
 * Run:
 * node deploy-contract.js
 */

const { ethers } = require('ethers');

// Simple contract (Storage)
const contractABI = [
    "function store(uint256 num) public",
    "function retrieve() public view returns (uint256)"
];

const contractBytecode = "0x608060405234801561001057600080fd5b50610150806100206000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c80632e64cec11461003b5780636057361d14610059575b600080fd5b610043610075565b60405161005091906100a1565b60405180910390f35b610073600480360381019061006e91906100ed565b61007e565b005b60008054905090565b8060008190555050565b6000819050919050565b61009b81610088565b82525050565b60006020820190506100b66000830184610092565b92915050565b600080fd5b6100ca81610088565b81146100d557600080fd5b50565b6000813590506100e7816100c1565b92915050565b600060208284031215610103576101026100bc565b5b6000610111848285016100d8565b9150509291505056fea2646970667358221220d8378feed47d0cfc6a6e6c68e17f2e0c6e1b4f9c8e7f3c5a8f8c3f0b8e7f3c64736f6c63430008120033";

async function main() {
    console.log('🚀 Deploying contract to Pasifika PoA Chain...\n');

    // Connect to local node
    const provider = new ethers.JsonRpcProvider('http://localhost:8545');
    
    // Test account with private key (from genesis)
    const privateKey = '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80';
    const wallet = new ethers.Wallet(privateKey, provider);
    
    console.log('📝 Deployer address:', wallet.address);
    
    // Check balance
    const balance = await provider.getBalance(wallet.address);
    console.log('💰 Balance:', ethers.formatEther(balance), 'ETH\n');
    
    // Check network
    const network = await provider.getNetwork();
    console.log('🌐 Network:', {
        chainId: network.chainId.toString(),
        name: network.name || 'Pasifika PoA'
    });
    
    // Create contract factory
    const factory = new ethers.ContractFactory(contractABI, contractBytecode, wallet);
    
    console.log('\n📤 Deploying contract...');
    
    // Deploy (with ZERO gas cost!)
    const contract = await factory.deploy({
        gasLimit: 3000000,
        gasPrice: 0  // ZERO GAS!
    });
    
    console.log('⏳ Waiting for deployment...');
    await contract.waitForDeployment();
    
    const address = await contract.getAddress();
    console.log('✅ Contract deployed at:', address);
    
    // Test the contract
    console.log('\n🧪 Testing contract...');
    
    // Store a value
    const tx1 = await contract.store(42, { gasPrice: 0 });
    console.log('📝 Storing value 42...');
    await tx1.wait();
    console.log('✅ Value stored');
    
    // Retrieve the value
    const value = await contract.retrieve();
    console.log('📖 Retrieved value:', value.toString());
    
    console.log('\n✅ All done!');
    console.log('Contract address:', address);
    console.log('Total gas cost: 0 ETH (ZERO GAS!)');
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
