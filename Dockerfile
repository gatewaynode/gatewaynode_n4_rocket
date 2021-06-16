FROM rust:1.52 AS build

WORKDIR /usr/src

RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new gatewaynode_n4_rocket

WORKDIR /usr/src/gatewaynode_n4_rocket

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release

COPY src ./src

RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch

COPY --from=build /usr/local/cargo/bin/gatewaynode_n4_rocket .

USER 1000

CMD ["./gatewaynode_n4_rocket"]