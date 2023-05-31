FROM docker.io/rustlang/rust-slim:nightly as builder
WORKDIR /build
RUN apt-get update && apt-get install -y libssl-dev libpq-dev && rm -rf /var/lib/apt/lists/*
COPY . .
RUN cargo build --release
FROM debian:buster-slim
WORKDIR /run
RUN apt-get update && apt-get install -y libssl1.1 libpq5 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/target/release/nb_runtime /usr/local/bin/nb_runtime
CMD ["nb_runtime"]
