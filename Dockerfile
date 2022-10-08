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

# Download the static build of Litestream directly into the path & make it executable.
# This is done in the builder and copied as the chmod doubles the size.
ADD https://github.com/benbjohnson/litestream/releases/download/v0.3.9/litestream-v0.3.9-linux-amd64-static.tar.gz /tmp/litestream.tar.gz
RUN tar -C /usr/local/bin -xzf /tmp/litestream.tar.gz

# Start building the final image
FROM debian:buster-slim
WORKDIR /home/rust/
COPY --from=builder /home/rust/target/release/peek .

COPY --from=builder /usr/local/bin/litestream /usr/local/bin/litestream

COPY ./server/litestream.yaml /etc/litestream.yaml

EXPOSE 3000

ENTRYPOINT ["litestream", "replicate", "--config", "/etc/litestream.yaml", "--exec", "./peek"]

