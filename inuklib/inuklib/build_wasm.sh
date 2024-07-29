#!/bin/bash -e

# See: https://fourteenscrews.com/essays/look-ma-no-wasm-pack/
#
# Before running:
# cargo install wasm-bindgen
# cargo install wasm-opt


cargo build --lib --features wasm --target wasm32-unknown-unknown --release

# Generate JS bindings
wasm-bindgen target/wasm32-unknown-unknown/release/analyzer.wasm --out-dir ./dist --target web --no-typescript

# Optimize WASM
wasm-opt ./dist/analyzer_bg.wasm -o ./dist/analyzer_bg.wasm-opt.wasm -O
mv ./dist/analyzer_bg.wasm-opt.wasm ./dist/analyzer_bg.wasm

# Copy web assets.
cp -r ./web/* ./dist/

echo "Build completed. Artifacts created in \`./dist/\`"