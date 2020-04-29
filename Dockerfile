FROM docker.pkg.github.com/chipp/base-image.rust.pi/base:latest

WORKDIR /home/rust/src
RUN USER=rust cargo init --lib /home/rust/src

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release && \
  cargo clean --release -p temp_reader && \
  rm src/*.rs

COPY ./src ./src

RUN cargo test --target=x86_64-unknown-linux-gnu && \
  rm -rf target/debug/

RUN cargo build --release
