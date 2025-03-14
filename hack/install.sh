#!/bin/bash

# Exit on error
set -e

echo "Building mealplan in release mode..."
cargo build --release

echo "Copying executable to /usr/local/bin..."
sudo cp target/release/mealplan /usr/local/bin/

echo "Installation complete! You can now run 'mealplan' from anywhere."
