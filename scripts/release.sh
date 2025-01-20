#!/bin/bash

# Release the project

# Build the client and server in release mode
cargo build --release --bin client
cargo build --release --bin server

# Create a release directory
mkdir -p release

# Copy the built binaries to the release directory
cp target/release/client release/
cp target/release/server release/

# Create a tarball of the release directory
tar -czvf release.tar.gz release
