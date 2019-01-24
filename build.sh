rm -f test.ppm
export RUSTFLAGS="-C target-cpu=native"
cargo run --release > test.ppm
# ./target/release/tracer > test.ppm