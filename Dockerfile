FROM docker.io/rustlang/rust:nightly-slim as builder
WORKDIR /build
COPY . .
RUN cargo build --release
FROM debian:buster-slim
WORKDIR /run
COPY --from=builder /build/target/release/nb_runtime /usr/local/bin/nb_runtime
CMD ["nb_runtime"]
