build-map:
    cargo build --release --target wasm32-unknown-unknown --bin war3map
    cp target/wasm32-unknown-unknown/release/war3map.wasm ../testmap/war3map.wasm
    mpqtool new ../testmap/ ../testmap.w3x

regenerate-bindings:
    cargo run --bin jassparse resource/common.j generate-rs-sys > ./sys/src/gen.rs