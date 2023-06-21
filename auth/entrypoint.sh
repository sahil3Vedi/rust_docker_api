if [ "$ENV" = "dev" ]; then
    cargo run
else
    cargo build --release && ./target/release/auth
fi