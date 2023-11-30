FROM rust:1.74.0

WORKDIR /app

RUN [ "cargo", "install", "trunk" ]
RUN [ "rustup", "target", "add", "wasm32-unknown-unknown"]
COPY assests /app/assests
COPY index.html /app/index.html
COPY src /app/src
COPY Cargo.toml /app/Cargo.toml
COPY Cargo.lock /app/Cargo.lock
RUN [ "trunk", "build", "--release" ]


CMD [ "trunk", "serve", "--release", "--address", "0.0.0.0" ]