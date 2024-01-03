rm -f .local/cef-bindings.rs && \
touch .local/cef-bindings.rs && \
cargo clean && \
cargo run --bin rebuild --features rebuild && \
mkdir -p target/debug && \
cp -rf .local/cef/Debug/* target/debug && \
cp -rf .local/cef/Resources/* target/debug && \
mkdir -p target/release && \
cp -rf .local/cef/Release/* target/release && \
cp -rf .local/cef/Resources/* target/release
