#! /bin/sh

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./public/wasm --target web ./target/wasm32-unknown-unknown/release/rustlog.wasm
# ~/binaryen/bin/wasm-opt -Os outrel/rustlog_bg.wasm -o ./outrel/opt.wasm
