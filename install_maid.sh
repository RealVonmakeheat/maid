#!/bin/bash
# install_maid.sh - Easy installer for the Maid CLI tool

set -e

echo "🧹 Maid CLI Installer"
echo "====================="

# Check if Rust/Cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Installing Rust first..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo "✅ Rust installed successfully"
else
    echo "✅ Rust is already installed"
fi

# Install maid
echo "📦 Installing Maid CLI..."

# Try from crates.io first
if cargo install maid-cli; then
    echo "✅ Maid installed successfully from crates.io"
else
    # If that fails, install from GitHub
    echo "⚠️ Couldn't install from crates.io, trying from GitHub..."
    
    # Create temporary directory
    TMP_DIR=$(mktemp -d)
    cd "$TMP_DIR"
    
    # Clone the repository
    git clone https://github.com/Realvonmakeheat/maid.git
    cd maid
    
    # Build and install
    cargo install --path .
    
    # Clean up
    cd
    rm -rf "$TMP_DIR"
    
    echo "✅ Maid installed successfully from GitHub"
fi

# Check if installation was successful
if command -v maid &> /dev/null; then
    echo "🎉 Installation complete! You can now use 'maid' command."
    echo
    echo "📝 Quick usage guide:"
    echo "  maid clean            # Clean up files in current directory"
    echo "  maid clean --restructure  # Clean and organize files"
    echo "  maid keep             # Keep important files, move others to trash"
    echo
    echo "📚 For more information, run: maid --help"
else
    echo "❌ Installation failed. Please try manual installation:"
    echo "1. git clone https://github.com/Realvonmakeheat/maid.git"
    echo "2. cd maid"
    echo "3. cargo build --release"
    echo "4. sudo cp target/release/maid /usr/local/bin/"
fi
