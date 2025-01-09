#!/bin/bash
echo "Building ByteMolar Solana program..."

# Build the program
cd program
cargo build-bpf

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "Program build successful!"
else
    echo "Program build failed!"
    exit 1
fi