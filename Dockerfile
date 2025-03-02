FROM docker.io/library/rust:1.85-alpine as builder

ENV TARGET=x86_64-unknown-linux-musl

WORKDIR /app

COPY . .

RUN apk update && \
    apk add --no-cache pkgconf alpine-sdk openssl-dev perl musl-dev && \
    rustup target add ${TARGET} && \
    cargo build --release --target ${TARGET} && \
    mv target/${TARGET}/release/rust-tooling target/

FROM docker.io/library/alpine:3.21

WORKDIR /app

COPY --from=builder ["/app/target/rust-tooling", "/usr/local/bin/rust-tooling"]
COPY --from=builder ["/etc/ssl/certs/ca-certificates.crt", "/etc/ssl/certs/"]

# Setup user
RUN addgroup -g 1000 rust && \
    adduser -D -u 1000 -G rust rust

USER rust

ENTRYPOINT [ "rust-tooling" ]

