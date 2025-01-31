FROM rust:latest
WORKDIR /bot

COPY Cargo.toml .
COPY Cargo.lock .
COPY .sqlx ./.sqlx
COPY src ./src
RUN cargo build --release

CMD ["./target/release/Trackr"]