FROM rustlang/rust:nightly as builder

WORKDIR /home/rust/

USER root

RUN rustc --version; cargo --version; rustup --version

RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 && \
    chmod +x tailwindcss-linux-x64 && \
    mv tailwindcss-linux-x64 tailwindcss

# Avoid having to install/build all dependencies by copying
# the Cargo files and making a dummy src/main.rs
COPY Cargo.toml .
COPY Cargo.lock .

COPY server/Cargo.toml ./server/
RUN mkdir -p ./server/src/ && echo "fn main() {}" > ./server/src/main.rs

COPY frontend/Cargo.toml ./frontend/
RUN mkdir -p ./frontend/src/ && echo "fn main() {}" > ./frontend/src/lib.rs

RUN cargo build --release --locked --bin peek

# We need to touch our real main.rs file or else docker will use
# the cached one.
COPY . .
RUN touch server/src/main.rs

RUN cd frontend && \
    wasm-pack build --target web

RUN ./tailwindcss -i frontend/src/tailwind.css -o frontend/pkg/tailwind.css

RUN cargo build --release --locked --bin peek

# Download the static build of Litestream directly into the path & make it executable.
# This is done in the builder and copied as the chmod doubles the size.
ADD https://github.com/benbjohnson/litestream/releases/download/v0.3.9/litestream-v0.3.9-linux-amd64-static.tar.gz /tmp/litestream.tar.gz
RUN tar -C /usr/local/bin -xzf /tmp/litestream.tar.gz

# Start building the final image
FROM debian:buster-slim as final
WORKDIR /home/rust/

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && update-ca-certificates

COPY --from=builder /home/rust/target/release/peek .

COPY --from=builder /usr/local/bin/litestream /usr/local/bin/litestream

COPY ./server/litestream.yaml /etc/litestream.yaml

EXPOSE 3000

ENTRYPOINT ["litestream", "replicate", "--config", "/etc/litestream.yaml", "--exec", "./peek"]
