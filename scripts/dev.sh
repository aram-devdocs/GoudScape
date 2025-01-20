#!/bin/bash

# Run the development environment for the client and server

# Start the server
cargo run --bin server &

# Start the client
cargo run --bin client
