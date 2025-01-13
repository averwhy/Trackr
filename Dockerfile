# dockerfile for building the image for the rust discord bot that uses serenity
FROM rust:latest
WORKDIR /bot

COPY Cargo.toml .
RUN cargo build

COPY src ./src

CMD ["python", "bot.py"]