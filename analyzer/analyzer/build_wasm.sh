#!/bin/bash -e

# See: https://fourteenscrews.com/essays/look-ma-no-wasm-pack/

# cargo install wasm-bindgen
# cargo install wasm-opt

# Build project for WASM
cargo build --lib --features wasm --target wasm32-unknown-unknown --release

# Generate JS bindings
wasm-bindgen target/wasm32-unknown-unknown/release/analyzer.wasm --out-dir ./dist --target web --no-typescript

# Optimize WASM
wasm-opt ./dist/analyzer_bg.wasm -o ./dist/analyzer_bg.wasm-opt.wasm -O
mv ./dist/analyzer_bg.wasm-opt.wasm ./dist/analyzer_bg.wasm

# Copy the HTML and assets
cp -r ./web/* ./dist/

echo "Build completed. Open ./dist/index.html to view your project."
