#!/bin/bash
echo "Running ByteMolar tests..."

# Run Rust tests
cargo test --manifest-path=program/Cargo.toml -- --nocapture

# Run client tests
cd client
npm test

echo "Tests complete!"