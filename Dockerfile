FROM rust:1.90 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release


FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/almacen-backend /app/almacen-backend

CMD ["/app/almacen-backend"]