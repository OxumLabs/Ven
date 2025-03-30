#!/bin/bash

set -e

# Define paths
BIN_DIR="bin"
BINARY_NAME="ven"

# Create output directories if they don't exist
mkdir -p "$BIN_DIR"

# Build for macOS natively
echo "Building for macOS..."

# Build the project using cargo for the default macOS target
cargo build --release --target x86_64-apple-darwin
if [[ $? -ne 0 ]]; then
    echo "macOS build failed!"
    exit 1
fi

# Copy macOS binary to the bin directory
cp target/x86_64-apple-darwin/release/$BINARY_NAME "$BIN_DIR/${BINARY_NAME}_mac"
echo "macOS binary copied to $BIN_DIR/${BINARY_NAME}_mac"

echo "Build completed successfully!"