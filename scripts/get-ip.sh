#!/bin/bash
# Automatically detect the machine's LAN IP address

# Get the primary non-localhost IP address
IP=$(ip addr show | grep "inet " | grep -v "127.0.0.1" | awk '{print $2}' | cut -d/ -f1 | head -1)

if [ -z "$IP" ]; then
    echo "0.0.0.0"
else
    echo "$IP"
fi
