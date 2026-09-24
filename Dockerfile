# Build stage
FROM rust:1.88-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock* ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release

COPY src ./src
COPY migrations ./migrations

RUN touch src/main.rs && \
    cargo build --release


# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/todo-api /app/todo-api
COPY --from=builder /app/migrations /app/migrations

EXPOSE 3000

CMD ["./todo-api"]