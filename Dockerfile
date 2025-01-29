# dockerfile for building the image for the rust discord bot that uses serenity
FROM rust:latest
WORKDIR /bot

COPY Cargo.toml .
COPY .sqlx ./.sqlx
COPY src ./src
RUN cargo build --release

CMD ["cargo", "run"]