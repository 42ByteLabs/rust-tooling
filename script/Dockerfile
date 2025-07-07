# Rust Tooling image 
# FROM gcr.io/42bytelabs/rust-tooling:v0.1.1 as tooling
FROM rust-tooling:latest as tooling

FROM docker.io/library/rust:1.85-alpine as builder

ENV TARGET=x86_64-unknown-linux-musl

WORKDIR /app

COPY . .

RUN rustup target add ${TARGET} && \
    cargo install rust-script

# Final image
FROM docker.io/library/alpine:3.21

WORKDIR /app

COPY --from=tooling ["/usr/local/bin/rust-tooling", "/usr/local/bin/rust-tooling"]
COPY --from=tooling ["/etc/ssl/certs/ca-certificates.crt", "/etc/ssl/certs/"]

COPY --from=builder ["/usr/local/cargo/bin/rust-script", "/usr/local/bin/rust-script"]

# Setup user
RUN addgroup -g 1000 rust && \
    adduser -D -u 1000 -G rust rust
USER rust

ENTRYPOINT [ "rust-tooling" ]
