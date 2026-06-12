# ---- Builder ----
FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev openssl-dev pkgconfig

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src src
RUN cargo build --release --bin pasm_server && \
    cp target/release/pasm_server /pasm_server

# ---- Runtime ----
FROM alpine:latest
RUN apk add --no-cache ca-certificates curl
COPY --from=builder /pasm_server /usr/local/bin/pasm_server

EXPOSE 3000
CMD ["pasm_server"]
