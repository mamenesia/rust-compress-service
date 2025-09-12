#!/bin/bash

echo "Building Rust Compress API..."

cargo build --release

echo "Build complete!"
echo "Binary located at target/release/rust_compress_api"
echo "To run the API:"
echo "  cargo run --release"
echo "Or:"
echo "  ./target/release/rust_compress_api"
