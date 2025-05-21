#!/usr/bin/env bash
set -e
echo "=== Nexsh Uninstaller ==="
INSTALL_DIR="/usr/local/bin/nexsh"
REPO_DIR="$HOME/nexsh"
if [ ! -f "$INSTALL_DIR" ]; then
    echo "Error: nexsh is not installed at $INSTALL_DIR"
    exit 1
fi
sudo rm -rf $INSTALL_DIR
rm -rf "$REPO_DIR"
