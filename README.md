# Rustlog

A simple spike for logging in Rust and WebAssembly.

## Creation

- Created the project with `cargo init --lib` in the root directory.
- Added the wasm-bindgen dependency with `cargo add wasm-bindgen` (and similar for `js-sys`, `log`, etc.)
- Added the CLI with `cargo install wasm-bindgen-cli`

See `./build.sh` for the Rust build commands to create the Wasm artifacts.
Start the site with `npx serve ./public` (no build step necessary - all JS currently).
