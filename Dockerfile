FROM docker.pkg.github.com/chipp/base-image.rust.pi/base:latest

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
  cargo clean --release -p reader && \
  cargo clean --release -p bluetooth && \
  cargo clean --release -p rumble

RUN cargo build --target=x86_64-unknown-linux-gnu && \
  cargo clean --target=x86_64-unknown-linux-gnu -p reader && \
  cargo clean --target=x86_64-unknown-linux-gnu -p bluetooth && \
  cargo clean --target=x86_64-unknown-linux-gnu -p rumble && \
  rm ./reader/src/*.rs && \
  rm ./bluetooth/src/*.rs && \
  rm ./rumble/src/*.rs

COPY ./reader/src ./reader/src
COPY ./bluetooth/src ./bluetooth/src
COPY ./rumble/src ./rumble/src

RUN cargo test --target=x86_64-unknown-linux-gnu && \
  rm -rf target/debug/

RUN cargo build --release
