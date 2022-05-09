FROM ghcr.io/chipp/build.rust.armv7_musl:1.50.0_1 AS builder

WORKDIR /home/rust/src
RUN USER=rust cargo init --lib /home/rust/src
RUN USER=rust cargo new --bin /home/rust/src/reader
RUN USER=rust cargo new --bin /home/rust/src/bluetooth
RUN USER=rust cargo new --bin /home/rust/src/rumble

COPY ./reader/Cargo.toml ./reader/Cargo.toml
COPY ./bluetooth/Cargo.toml ./bluetooth/Cargo.toml
COPY ./rumble/Cargo.toml ./rumble/Cargo.toml

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release && \
  cargo clean --release -p reader -p bluetooth -p rumble

RUN cargo build --target=x86_64-unknown-linux-gnu && \
  cargo clean --target=x86_64-unknown-linux-gnu -p reader -p bluetooth -p rumble && \
  rm ./reader/src/*.rs && \
  rm ./bluetooth/src/*.rs && \
  rm ./rumble/src/*.rs

COPY ./reader/src ./reader/src
COPY ./bluetooth/src ./bluetooth/src
COPY ./rumble/src ./rumble/src

RUN cargo test --target=x86_64-unknown-linux-gnu && \
  rm -rf target/debug/

RUN cargo build --release
