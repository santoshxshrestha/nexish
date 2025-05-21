#!/usr/bin/env bash
set -e
echo "=== nexish Uninstaller ==="

INSTALL_PATH="/usr/local/bin/nexish"
REPO_DIR="$HOME/nexish"

if [ ! -f "$INSTALL_PATH" ]; then
    echo "Error: nexish binary is not installed at $INSTALL_PATH"
else
    sudo rm -f "$INSTALL_PATH"
    echo "Removed nexish binary from $INSTALL_PATH"
fi

if [ ! -d "$REPO_DIR" ]; then
    echo "Warning: nexish repository not found at $REPO_DIR"
else
    rm -rf "$REPO_DIR"
    echo "Removed nexish repository at $REPO_DIR"
fi

echo "nexish is completely removed."
