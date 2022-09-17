FROM rustlang/rust:nightly as builder

WORKDIR /home/rust/

USER root

RUN rustc --version; cargo --version; rustup --version

# Avoid having to install/build all dependencies by copying
# the Cargo files and making a dummy src/main.rs
COPY Cargo.toml .
COPY Cargo.lock .
COPY server/Cargo.toml ./server/
RUN mkdir -p ./server/src/ && echo "fn main() {}" > ./server/src/main.rs
RUN cargo build --release --locked --bin peek

# We need to touch our real main.rs file or else docker will use
# the cached one.
COPY . .
RUN touch server/src/main.rs

RUN cargo build --release --locked --bin peek

# Start building the final image
FROM debian:buster-slim
WORKDIR /home/rust/
COPY --from=builder /home/rust/target/release/peek .

EXPOSE 3000

ENTRYPOINT ["./peek"]

