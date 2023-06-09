# Rustlog

A simple spike for logging in Rust and WebAssembly.

## Creation

- Created the project with `cargo init --lib` in the root directory.
- Added the wasm-bindgen dependency with `cargo add wasm-bindgen` (and similar for `js-sys`, `log`, etc.)
- Added the CLI with `cargo install wasm-bindgen-cli`

See `./build.sh` for the Rust build commands to create the Wasm artifacts.
Start the site with `npx serve ./public` (no build step necessary - all JS currently).

## TODO

How about telemetry? How should this work?

It should probably be an entirely separate hook. An Atomic bool should indicate if it is set,
and if so unwrap and call a static Option holding a JsFunction again. Being that the compiler is
running in a WebWorker, this Function would likely just shuttle the playload over to the
main thread for logging.

Similar to Log, Telemetry should be a crate/module with a trait and a thread_local static 'current'.

There should be the native telemetry code, which just has a callback (or nothing), and the wasm
code sets this to a lambda that calls the JsFunction. (Should probably be a trait).
