FROM rust:latest

RUN apt update && \
    apt install -y sqlite3 curl python3-pip python3-venv && \
    pip install cargo-lambda  && \
    cargo install maturin
