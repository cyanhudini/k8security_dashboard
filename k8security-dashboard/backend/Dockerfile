
FROM rust:1.88-slim-bookworm AS builder

WORKDIR /app


RUN apt-get update && apt-get install -y \
    pkg-config \
    libpq-dev \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./


RUN mkdir src/ \
    && echo "fn main() {println!(\"dummy\");}" > src/main.rs \
    && cargo build --release \
    && rm -rf src/

RUN cargo install diesel_cli --root /usr/local --no-default-features --features "postgres"

COPY . .

RUN cargo build --release --locked --target-dir ./target


FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update && apt-get install -y \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*


COPY --from=builder /app/target/release/backend ./backend


COPY --from=builder /app/migrations ./migrations

COPY --from=builder /usr/local/bin/diesel /usr/local/bin/diesel

COPY --from=builder /app/.env ./.env

COPY --from=builder /app/report.json ./report.json

EXPOSE 8080


CMD ["sh", "-c", "/usr/local/bin/diesel migration run && ./backend"]