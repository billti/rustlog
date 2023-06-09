# Rustlog

Created the project with `cargo init --lib` in the directory.

Added the wasm-bindgen dependency with `cargo add wasm-bindgen`

Added the CLI with `cargo install wasm-bindgen-cli`

Built the debug version with `cargo build --target wasm32-unknown-unknown`

Built the wasm package with `wasm-bindgen --out-dir ./out --target web ./target/wasm32-unknown-unknown/debug/rustlog.wasm`

See `./build.sh` for release build.
Start the site with `npx serve ./public`
