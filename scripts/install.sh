#!/bin/bash

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Install Bevy dependencies
sudo apt-get update
sudo apt-get install -y libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev

# Install pre-commit
pip install pre-commit

# Install pre-commit hooks
pre-commit install
