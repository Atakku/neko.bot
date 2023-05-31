FROM docker.io/rustlang/rust:nightly-slim as builder
WORKDIR /build
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY . .
RUN cargo build --release
FROM debian:buster-slim
WORKDIR /run
RUN apt-get update && apt-get install -y libc6 libssl1.1 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/target/release/nb_runtime /usr/local/bin/nb_runtime
CMD ["nb_runtime"]
