#!/bin/bash

set -e

# Define paths
BIN_DIR="bin"           
BINARY_NAME="ven"       

# Create output directories if they don't exist
mkdir -p "$BIN_DIR"

# Build for Windows
echo "Building for Windows..."
cargo build --release --target x86_64-pc-windows-gnu
if [[ $? -ne 0 ]]; then
    echo "Windows build failed!"
    exit 1
fi

# Copy Windows binary with the desired name
cp target/x86_64-pc-windows-gnu/release/$BINARY_NAME.exe "$BIN_DIR/${BINARY_NAME}_win.exe"
echo "Windows binary copied to $BIN_DIR/${BINARY_NAME}_win.exe"

# Build for Linux
echo "Building for Linux..."
cargo build --release --target x86_64-unknown-linux-musl
if [[ $? -ne 0 ]]; then
    echo "Linux build failed!"
    exit 1
fi

# Copy Linux binary
cp target/x86_64-unknown-linux-musl/release/$BINARY_NAME "$BIN_DIR/${BINARY_NAME}_lin"
echo "Linux binary copied to $BIN_DIR/${BINARY_NAME}_lin"

# Prompt to build for macOS
read -p "Do you want to build for macOS? (y/n): " build_macos

if [[ $build_macos == "y" ]]; then
    echo "Building for macOS..."
    cross build --release --target x86_64-apple-darwin
    if [[ $? -ne 0 ]]; then
        echo "macOS build failed!"
        exit 1
    fi

    # Copy macOS binary
    cp target/x86_64-apple-darwin/release/$BINARY_NAME "$BIN_DIR/${BINARY_NAME}_mac"
    echo "macOS binary copied to $BIN_DIR/${BINARY_NAME}_mac"
fi

echo "Build completed successfully!"
