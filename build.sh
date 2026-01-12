#!/bin/bash

# Build script for Japanese Emoji Learning App
# Checks for dependencies and builds the project

set -e

echo "=========================================="
echo "Japanese Emoji Learning App - Build Script"
echo "=========================================="
echo ""

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if Rust is installed
echo "Checking for Rust installation..."
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    echo -e "${GREEN}✓ Rust is installed: $RUST_VERSION${NC}"
else
    echo -e "${RED}✗ Rust is not installed${NC}"
    echo ""
    echo "To install Rust, run:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo ""
    exit 1
fi

# Check if Cargo is installed
echo "Checking for Cargo installation..."
if command -v cargo &> /dev/null; then
    CARGO_VERSION=$(cargo --version)
    echo -e "${GREEN}✓ Cargo is installed: $CARGO_VERSION${NC}"
else
    echo -e "${RED}✗ Cargo is not installed${NC}"
    echo "Cargo should come with Rust. Please reinstall Rust."
    exit 1
fi

echo ""
echo "All dependencies are satisfied!"
echo ""

# Build the project
echo "Building the project..."
echo ""

if cargo build --release; then
    echo ""
    echo -e "${GREEN}=========================================="
    echo "✓ Build successful!"
    echo "==========================================${NC}"
    echo ""
    echo "To run the app:"
    echo "  cargo run --release"
    echo ""
    echo "Or run the binary directly:"
    echo "  ./target/release/emoji_japanese_learner"
    echo ""
else
    echo ""
    echo -e "${RED}=========================================="
    echo "✗ Build failed!"
    echo "==========================================${NC}"
    echo ""
    exit 1
fi
