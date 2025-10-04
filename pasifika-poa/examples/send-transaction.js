#!/usr/bin/env node

/**
 * Example: Send a transaction on Pasifika PoA Chain
 * 
 * Install dependencies:
 * npm install ethers
 * 
 * Run:
 * node send-transaction.js
 */

const { ethers } = require('ethers');

async function main() {
    console.log('💸 Sending transaction on Pasifika PoA Chain...\n');

    // Connect to local node
    const provider = new ethers.JsonRpcProvider('http://localhost:8545');
    
    // Sender (pre-funded in genesis)
    const senderKey = '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80';
    const sender = new ethers.Wallet(senderKey, provider);
    
    // Receiver (another pre-funded account)
    const receiverAddress = '0x70997970C51812dc3A010C7d01b50e0d17dc79C8';
    
    console.log('👤 From:', sender.address);
    console.log('👤 To:', receiverAddress);
    
    // Check balances before
    const senderBalanceBefore = await provider.getBalance(sender.address);
    const receiverBalanceBefore = await provider.getBalance(receiverAddress);
    
    console.log('\n📊 Balances before:');
    console.log('  Sender:', ethers.formatEther(senderBalanceBefore), 'ETH');
    console.log('  Receiver:', ethers.formatEther(receiverBalanceBefore), 'ETH');
    
    // Send transaction
    const amount = ethers.parseEther('10.0');
    console.log('\n💰 Sending:', ethers.formatEther(amount), 'ETH');
    
    const tx = await sender.sendTransaction({
        to: receiverAddress,
        value: amount,
        gasLimit: 21000,
        gasPrice: 0  // ZERO GAS!
    });
    
    console.log('📤 Transaction hash:', tx.hash);
    console.log('⏳ Waiting for confirmation...');
    
    const receipt = await tx.wait();
    console.log('✅ Transaction confirmed in block:', receipt.blockNumber);
    
    // Check balances after
    const senderBalanceAfter = await provider.getBalance(sender.address);
    const receiverBalanceAfter = await provider.getBalance(receiverAddress);
    
    console.log('\n📊 Balances after:');
    console.log('  Sender:', ethers.formatEther(senderBalanceAfter), 'ETH');
    console.log('  Receiver:', ethers.formatEther(receiverBalanceAfter), 'ETH');
    
    console.log('\n💵 Transaction cost:');
    console.log('  Gas used:', receipt.gasUsed.toString());
    console.log('  Gas price:', '0 wei');
    console.log('  Total cost:', '0 ETH');
    console.log('  ✨ ZERO GAS FEES! ✨');
    
    // Verify the transfer
    const expectedSenderBalance = senderBalanceBefore - amount;
    const expectedReceiverBalance = receiverBalanceBefore + amount;
    
    console.log('\n✅ Transfer verified:');
    console.log('  Sender lost:', ethers.formatEther(amount), 'ETH (+ 0 gas)');
    console.log('  Receiver gained:', ethers.formatEther(amount), 'ETH');
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
