# w3rs - Warcraft 3 and Rust - a match made in hell?

This repo is an assorted collection of utilities for working with Warcraft 3: CE and Rust.

## WASM

Most interesting right now - libraries for using Rust in WC3. The `test` directory includes a sample project that can get you up to speed using WASM in WC3.

Quickstart guide:

1. Get [rustup](https://rustup.rs/)
2. `rustup toolchain install nightly` to install the latest `nightly` Rust toolchain
3. `rustup target add wasm32-unknown-unknown` to pull in the WASM target tools
4. `rustup override set nightly` set `nightly` as default channel for this repo
5. `cargo build --release --target wasm32-unknown-unknown --bin war3map` to build `war3map.wasm` from the `test/` project.
6. `target/wasm32-unknown-unknown/target/war3map.wasm` is now the compiled WASM binary - add it to a `.w3x` and try to run it in W3CE!