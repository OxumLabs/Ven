    #!/bin/bash

    set -e

    # Define paths
    BIN_DIR="bin"
    BINARY_NAME="ven"

    # Create output directories if they don't exist
    mkdir -p "$BIN_DIR"

    # Function to prompt the user for confirmation
    prompt_for_build() {
        local platform=$1
        read -p "Do you want to build for $platform? (y/n): " response

        # Convert response to lowercase
        response=${response,,}

        # Check for valid yes responses
        if [[ "$response" =~ ^(y|yes|yy)$ ]]; then
            return 0 # Yes
        elif [[ "$response" =~ ^(n|no)$ ]]; then
            return 1 # No
        else
            echo "Invalid response! Please type 'y' or 'n'."
            prompt_for_build "$platform" # Prompt again
        fi
    }

    # Build for Windows
    if prompt_for_build "Windows"; then
        echo "Building for Windows..."
        cargo build --release --target x86_64-pc-windows-gnu
        if [[ $? -ne 0 ]]; then
            echo "Windows build failed!"
            exit 1
        fi

        # Copy Windows binary with the desired name
        cp target/x86_64-pc-windows-gnu/release/$BINARY_NAME.exe "$BIN_DIR/${BINARY_NAME}_win.exe"
        echo "Windows binary copied to $BIN_DIR/${BINARY_NAME}_win.exe"
    fi

    # Build for Linux
    if prompt_for_build "Linux"; then
        echo "Building for Linux..."
        cargo build --release --target x86_64-unknown-linux-musl
        if [[ $? -ne 0 ]]; then
            echo "Linux build failed!"
            exit 1
        fi

        # Copy Linux binary
        cp target/x86_64-unknown-linux-musl/release/$BINARY_NAME "$BIN_DIR/${BINARY_NAME}_lin"
        echo "Linux binary copied to $BIN_DIR/${BINARY_NAME}_lin"
    fi

    # Build for macOS
    if prompt_for_build "macOS"; then
        echo "Building for macOS..."

        # Use osxcross to build for macOS using o64-clang
        CC=o64-clang cargo build --release --target x86_64-apple-darwin
        if [[ $? -ne 0 ]]; then
            echo "macOS build failed!"
            exit 1
        fi

        # Copy macOS binary
        cp target/x86_64-apple-darwin/release/$BINARY_NAME "$BIN_DIR/${BINARY_NAME}_mac"
        echo "macOS binary copied to $BIN_DIR/${BINARY_NAME}_mac"
    fi

    echo "Build completed successfully!"
