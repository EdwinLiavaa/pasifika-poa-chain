# Pasifika PoA Chain - Docker Image

FROM rust:1.88-slim as builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    git \
    clang \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Copy the entire reth source
COPY ../ ./

# Build pasifika-poa
WORKDIR /build/pasifika-poa
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /build/pasifika-poa/target/release/pasifika-poa /usr/local/bin/

# Create data directory
RUN mkdir -p /data

WORKDIR /data

# Expose ports
EXPOSE 8545 30303

# Default command
ENTRYPOINT ["pasifika-poa"]
CMD ["--node-id", "node1", "--http-port", "8545", "--p2p-port", "30303", "--dev"]
