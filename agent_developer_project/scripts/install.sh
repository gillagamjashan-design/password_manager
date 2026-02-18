#!/usr/bin/env bash
# install.sh — installs the agent-team binary to ~/.local/bin/
# Run this from the project root: bash scripts/install.sh

set -e  # Exit immediately if any command fails

echo "[install] Building agent-team in release mode..."
CARGO_HOME=/workspace/jashan/.cargo cargo build --release

BINARY_SRC="./target/release/agent-team"
INSTALL_DIR="$HOME/.local/bin"
BINARY_DEST="$INSTALL_DIR/agent-team"

echo "[install] Creating install directory if it doesn't exist..."
mkdir -p "$INSTALL_DIR"

echo "[install] Copying binary to $BINARY_DEST ..."
cp "$BINARY_SRC" "$BINARY_DEST"
chmod +x "$BINARY_DEST"

echo "[install] Binary installed at: $BINARY_DEST"

# Check if ~/.local/bin is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "[install] $INSTALL_DIR is not in your PATH. Adding it now..."

    EXPORT_LINE='export PATH="$HOME/.local/bin:$PATH"'

    # Add to ~/.bashrc
    if [ -f "$HOME/.bashrc" ]; then
        echo "$EXPORT_LINE" >> "$HOME/.bashrc"
        echo "[install] Added to ~/.bashrc"
    fi

    # Add to ~/.zshrc if it exists
    if [ -f "$HOME/.zshrc" ]; then
        echo "$EXPORT_LINE" >> "$HOME/.zshrc"
        echo "[install] Added to ~/.zshrc"
    fi

    echo ""
    echo "[install] IMPORTANT: Run the following command to apply the PATH change:"
    echo "          source ~/.bashrc"
    echo "          (or open a new terminal)"
else
    echo "[install] $INSTALL_DIR is already in your PATH. No changes needed."
fi

echo ""
echo "[install] Done! You can now run 'agent-team' from anywhere."
