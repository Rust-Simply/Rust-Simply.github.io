dx build --features web --release
cargo run --features ssr

static-web-server --port 8787 --root ./docs