#!/bin/sh

# Build the Rust project in release mode
cargo build --release

# Move the built binary to /usr/local/bin
sudo mv target/release/todo /usr/local/bin/todo

