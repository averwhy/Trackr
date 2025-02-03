FROM rust:latest AS builder 
WORKDIR /home/bot

COPY . .
RUN cargo build --release

FROM debian:bookworm
WORKDIR /bot
COPY --from=builder /home/bot/target/release/Trackr .
CMD ["sudo ./Trackr"]