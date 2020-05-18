FROM docker.pkg.github.com/chipp/base-image.rust.pi/base:latest

WORKDIR /home/rust/src
RUN USER=rust cargo init --lib /home/rust/src
RUN USER=rust cargo new --bin /home/rust/src/reader

COPY ./reader/Cargo.toml ./reader/Cargo.toml

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release && \
  cargo clean --release -p reader

RUN cargo build --target=x86_64-unknown-linux-gnu && \
  cargo clean --target=x86_64-unknown-linux-gnu -p reader && \
  rm ./reader/src/*.rs

COPY ./reader/src ./reader/src

RUN cargo test --target=x86_64-unknown-linux-gnu && \
  rm -rf target/debug/

RUN cargo build --release
