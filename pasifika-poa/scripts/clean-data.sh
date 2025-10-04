#!/bin/bash

# Clean all blockchain data

echo "🧹 Cleaning blockchain data..."
echo ""

read -p "Are you sure you want to delete all blockchain data? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]
then
    echo "Cancelled."
    exit 1
fi

echo "Stopping any running nodes..."
pkill -f pasifika-poa

echo "Removing data directories..."
rm -rf data/
rm -rf node*/

echo "✅ Cleanup complete!"
echo ""
echo "You can now start fresh by running:"
echo "  ./scripts/start-node1.sh"
