
RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals -Z macro-backtrace' \
        rustup run nightly \
        wasm-pack build --target web --scope ml.wasm --dev . \
        -- -Z build-std=panic_abort,std

sed -i '/"files":/a \    "snippets",' pkg/package.json
