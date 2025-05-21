#!/usr/bin/env bash
set -e

echo "=== nexish Installer ==="

if ! command -v cargo >/dev/null 2>&1; then
    echo "Rust is not installed. Installing via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    export PATH="$HOME/.cargo/bin:$PATH"
else
    echo "Rust is already installed."
fi

git clone --depth 1 --branch main https://github.com/santoshxshrestha/nexish.git "$HOME/nexish"

echo "Building nexish in release mode..."
cd "$HOME/nexish"
cargo build --release

BINARY_PATH="$HOME/nexish/target/release/nexish"
INSTALL_DIR="/usr/local/bin"
if [ ! -f "$BINARY_PATH" ]; then
    echo "Error: Release binary not found at $BINARY_PATH."
    exit 1
fi

echo "Copying nexish to $INSTALL_DIR (you may need to enter your password)..."
sudo cp "$BINARY_PATH" "$INSTALL_DIR/nexish"

sudo chmod +x "$INSTALL_DIR/nexish"

echo "nexish has been installed to $INSTALL_DIR and should be available in your PATH."
echo "You can now run 'nexish' from anywhere in your terminal."
