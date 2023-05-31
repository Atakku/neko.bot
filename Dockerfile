FROM docker.io/rustlang/rust:nightly as builder
WORKDIR /build
COPY . .
RUN cargo build --release
FROM debian:buster-slim
WORKDIR /run
RUN apt-get update & apt-get install -y libssl & rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/target/release/nb_runtime /usr/local/bin/nb_runtime
CMD ["nb_runtime"]
