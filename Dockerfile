FROM docker.io/rustlang/rust:nightly-slim as builder
WORKDIR /build
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY . .
RUN cargo build
FROM debian:11-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libssl1.1 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/target/release/nb_runtime /app/nekobot
CMD ["./nekobot"]
