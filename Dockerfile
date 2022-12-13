FROM rust:1.60.0 AS build
WORKDIR /usr/src

RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new zkctl
WORKDIR /usr/src/zkctl
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM alpine/k8s:1.20.7
COPY --from=build /usr/local/cargo/bin/zkctl .
USER 0
CMD ["./zkctl"]
