FROM rust:1.85.1-alpine3.21 as builder

RUN apk update && apk upgrade

RUN apk add --no-cache musl-dev protobuf

WORKDIR /tuno-cli

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release

COPY build.rs ./
COPY ./src ./src
COPY ./tuno ./tuno

RUN cargo build --release

FROM alpine:3.21

WORKDIR /tuno-cli

COPY config.toml ./
COPY ./media ./media

COPY --from=builder /tuno-cli/target/release/tuno-cli .

CMD ["./tuno-cli"]
