pong *FLAGS:
    cargo run --bin pong {{ FLAGS }}

train *FLAGS:
    cargo run --bin trainer {{ FLAGS }}

bench *FLAGS:
    cargo run --bin bench --release {{ FLAGS }}
