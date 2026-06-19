#!/bin/bash
echo "Setting up development environment..."
rustup component add clippy rustfmt
cargo fetch
echo "Setup complete!"
