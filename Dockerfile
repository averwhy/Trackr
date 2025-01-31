FROM rust:latest AS builder 
WORKDIR /bot

COPY Cargo.toml .
COPY Cargo.lock .
COPY .sqlx ./.sqlx
COPY src ./src
RUN cargo build --release

COPY --from=builder /bot/target/release/Trackr .
CMD ["./Trackr"]