FROM rust AS chef
WORKDIR /app
RUN cargo install cargo-chef
RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get install -y musl-tools

# Analyse the dependencis and project structure
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build minimal dependency
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

# Cached compile
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin pasm_server

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/pasm_server .
EXPOSE 3000
CMD ["./pasm_server"]
