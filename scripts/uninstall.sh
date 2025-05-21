#!/usr/bin/env bash
set -e
echo "=== Nexsh Uninstaller ==="

INSTALL_PATH="/usr/local/bin/nexsh"
REPO_DIR="$HOME/nexsh"

if [ ! -f "$INSTALL_PATH" ]; then
    echo "Error: nexsh binary is not installed at $INSTALL_PATH"
else
    sudo rm -f "$INSTALL_PATH"
    echo "Removed nexsh binary from $INSTALL_PATH"
fi

# Remove the cloned repo, if it exists
if [ ! -d "$REPO_DIR" ]; then
    echo "Warning: nexsh repository not found at $REPO_DIR"
else
    rm -rf "$REPO_DIR"
    echo "Removed nexsh repository at $REPO_DIR"
fi

echo "nexsh is completely removed."
