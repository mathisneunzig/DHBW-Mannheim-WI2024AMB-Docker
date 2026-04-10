FROM rust:slim AS builder
WORKDIR /app
COPY . .

RUN rustup target add x86_64-unknown-linux-musl && \
    apt-get update && apt-get install -y musl-tools && \
    cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/greetTommy /greetTommy
EXPOSE 8000
CMD ["/greetTommy"]
