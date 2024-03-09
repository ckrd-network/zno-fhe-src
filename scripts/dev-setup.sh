#!/usr/bin/env bash

# Define the installation directory according to the XDG Base Directory Specification
install_dir="${XDG_DATA_HOME:-$HOME/.local/share}/cargo/bin"

# Install redo using cargo
CARGO_TARGET_DIR="$install_dir" cargo install --git https://github.com/zombiezen/redo-rs.git

# Add the installation directory to the PATH if it's not already there
if [[ ":$PATH:" != *":$install_dir:"* ]]; then
    echo 'export PATH="$PATH:'$install_dir'"' >> ~/.bashrc
    source ~/.bashrc
fi

# Test if redo is in the PATH
if command -v redo &> /dev/null; then
    echo "redo installed successfully and is in the PATH"
else
    echo "redo is not in the PATH. Please check your installation."
fi