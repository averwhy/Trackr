FROM rust:latest AS builder 
WORKDIR /bot

COPY Cargo.toml .
COPY Cargo.lock .
COPY .sqlx ./.sqlx
COPY src ./src
RUN cargo build --release

FROM debian:bookworm
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /bot
COPY config.json .
COPY --from=builder /bot/target/release/Trackr .
CMD ["./Trackr"]