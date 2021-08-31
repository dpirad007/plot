set RUSTFLAGS=-C target-feature=+atomics,+bulk-memory
rustup run nightly-2021-02-11 wasm-pack build --target web --scope wasml -- -Z build-std=panic_abort,std

