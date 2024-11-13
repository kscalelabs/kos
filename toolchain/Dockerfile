FROM rust:bookworm

ENV CROSS_CONTAINER_IN_CONTAINER=true

RUN apt-get update && apt-get install -y protobuf-compiler gcc-aarch64-linux-gnu g++-aarch64-linux-gnu build-essential \
    libudev-dev
RUN rustup target add aarch64-unknown-linux-gnu && \
    rustup target add x86_64-unknown-linux-gnu && \
    rustup target add aarch64-apple-darwin && \
    rustup target add x86_64-apple-darwin && \
    rustup component add rustfmt clippy

RUN cargo install cross --git https://github.com/cross-rs/cross
RUN curl -fsSL https://get.docker.com | sh
