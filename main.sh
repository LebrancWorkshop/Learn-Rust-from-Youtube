cd "$1" &&
cargo rustc -- --emit asm &&
cargo run "$1" &&
cd ..