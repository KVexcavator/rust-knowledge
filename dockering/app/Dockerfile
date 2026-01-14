FROM rust:1.91 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

COPY src ./src
RUN cargo build --release


FROM debian:stable-slim

WORKDIR /app

COPY --from=builder /app/target/release/app /app/app

CMD ["./app"]

# docker build -t app .
# docker run -it app