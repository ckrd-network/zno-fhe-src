#!/usr/bin/env bash

# List of packages to install
# List of packages to install
lib_packages=(rust-audit-info) # library crates
# "deny" "duplicates" "geiger" "nextest" "outdated" "vet"
# "edit"
cargo_subcommands=("auditable" "audit" "crev") # cargo sub-commands
bin_packages=("redo") # non-sub-command cargo binaries

set -x

# Define the installation directory according to the XDG Base Directory Specification
bin_dir="${XDG_DATA_HOME:-${HOME}/.local/share}/cargo/bin"

# Add the installation directory to the PATH if it's not already there
if [[ ":${PATH}:" != *":${bin_dir}:"* ]]; then
    export PATH=${PATH}:${bin_dir}
fi

# CARGO_TARGET_DIR="$bin_dir" cargo install --force --git https://github.com/zombiezen/redo-rs.git

for package in "${lib_packages[@]}"; do
    # Check if the package is installed
    if ! cargo metadata --format-version=1 | jq -e --arg package "$package" '.packages[] | select(.name == $package)' &> /dev/null; then
        echo "$package is not installed. Installing..."
        cargo install "$package"
    else
        # Get the installed version
        installed_version=$(cargo metadata --format-version=1 | jq -r --arg package "$package" '.packages[] | select(.name == $package) | .version')
        # Get the latest version
        latest_version=$(cargo search "$package" --limit 1 | awk '{print $3}' | tr -d '"')

        # Check if the installed version is up-to-date
        if [[ "$installed_version" != "$latest_version" ]]; then
            echo "$package is not up-to-date: $installed_version != $latest_version. Updating..."
            CARGO_TARGET_DIR="$bin_dir" cargo install --force "$package"
        else
            echo "$package is up-to-date."
        fi
    fi
done

for package in "${cargo_subcommands[@]}"; do
    # Check if the package is installed
    if ! command -v "cargo-$package" &> /dev/null && ! command -v "cargo $package" &> /dev/null; then
        echo "cargo-S$package is not installed. Installing..."
        cargo install "cargo-$package"
    else
        # Get the installed version
        installed_version=$(cargo install --list | grep "^$package " | awk '{print $2}' | tr -d '()')

        # Get the latest version
        latest_version=$(cargo search "cargo-$package" --limit 1 | awk '{print $3}' | tr -d '"')

        # Check if the installed version is up-to-date
        if [[ "$installed_version" != "$latest_version" ]]; then
            echo "cargo-$package is not up-to-date: $installed_version != $latest_version. Updating..."
            cargo install --force "cargo-$package"
        else
            echo "cargo-$package is up-to-date."
        fi
    fi
done

for package in "${bin_packages[@]}"; do
    # Check if the package is installed
    if ! command -v "$package" &> /dev/null; then
        echo "$package is not installed. Installing..."
        CARGO_TARGET_DIR="$bin_dir" cargo install "$package"
    else
        # Get the installed version
        installed_version=$("$package" --version | awk '{print $2}')

        # Get the latest version
        latest_version=$(cargo search "$package" --limit 1 | awk '{print $3}' | tr -d '"')

        # Check if the installed version is up-to-date
        if [[ "$installed_version" != "$latest_version" ]]; then
            echo "$package is not up-to-date: $installed_version != $latest_version. Updating..."
            CARGO_TARGET_DIR="$bin_dir" cargo install --force "$package"
        else
            echo "$package is up-to-date."
        fi
    fi
done

for scmd in "${cargo_subcommands[@]}"; do
    if command -v "cargo-$scmd" &> /dev/null; then
        echo "cargo $scmd installed successfully and is in the PATH"
    else
        echo "cargo $scmd is not in the PATH. Please check your installation."
    fi
done

for binary in "${bin_packages[@]}"; do
    if command -v "$binary" &> /dev/null; then
        echo "$binary installed successfully and is in the PATH"
    else
        echo "$binary is not in the PATH. Please check your installation."
    fi
done