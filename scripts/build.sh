#!/bin/bash

echo "Building ByteMolar..."

# Install dependencies
npm install

# Build Solana program
cargo build-bpf

# Run tests
cargo test

echo "Build complete!"