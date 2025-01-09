#!/bin/bash
echo "Deploying ByteMolar to Solana..."

# Set the Solana cluster
CLUSTER="devnet"
solana config set --url $CLUSTER

# Build the program
./scripts/build-program.sh

# Deploy the program
solana program deploy \
    ./program/target/deploy/bytemolar_program.so

echo "Deployment complete!"