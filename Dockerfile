FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies (SSL for most DB crates)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Binary will be mounted at runtime
CMD ["/app/almacen-backend"]