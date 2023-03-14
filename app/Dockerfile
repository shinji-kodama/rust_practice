FROM rust:latest

RUN apt update && \
    apt install -y sqlite3 curl python3-pip python3-venv && \
    pip install cargo-lambda  && \
    cargo install maturin && \
    cargo install diesel_cli --no-default-features --features sqlite-bundled && \
    cargo install trunk && \
    cargo install cargo-make && \
    rustup target add aarch64-unknown-linux-gnu && \
    rustup target add wasm32-unknown-unknown
