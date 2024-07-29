#!/bin/bash -e

# See: https://fourteenscrews.com/essays/look-ma-no-wasm-pack/
#
# Before running:
# cargo install wasm-bindgen
# cargo install wasm-opt


ASSETS="./assets/extension/Firefox/"
OUT="./dist/extension/Firefox/"

cargo build --lib --features wasm --target wasm32-unknown-unknown --release

wasm-bindgen target/wasm32-unknown-unknown/release/analyzer.wasm --out-dir $OUT --target web --no-typescript
wasm-opt $OUT/analyzer_bg.wasm -o $OUT/analyzer_bg.wasm-opt.wasm -O
mv $OUT/analyzer_bg.wasm-opt.wasm $OUT/analyzer_bg.wasm

cp -r $ASSETS* $OUT

echo "Build completed. Artifacts created in \`$OUT\`"