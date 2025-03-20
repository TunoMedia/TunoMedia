FROM rust:1.85.1-slim-bookworm AS builder

RUN apt-get update -y && apt-get install protobuf-compiler -y

WORKDIR /tuno-cli

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release

COPY build.rs ./
COPY ./src ./src
COPY ./tuno ./tuno

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /tuno-cli

COPY config.toml ./
COPY ./media ./media

COPY --from=builder /tuno-cli/target/release/tuno-cli .

CMD ["./tuno-cli"]
