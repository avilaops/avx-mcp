FROM rust:1.75 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml ./
COPY avx-cli/Cargo.toml ./avx-cli/
COPY avx-mcp/Cargo.toml ./avx-mcp/
COPY avx-config/Cargo.toml ./avx-config/

# Copy source
COPY avx-cli/src ./avx-cli/src
COPY avx-mcp/src ./avx-mcp/src
COPY avx-config/src ./avx-config/src

# Build release
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

# Install dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy binaries
COPY --from=builder /app/target/release/avx-cli /usr/local/bin/
COPY --from=builder /app/target/release/avx-mcp /usr/local/bin/

# Set environment variables
ENV AVX__STACK=production
ENV AVX__LAYER=core
ENV AVX__ENV=prod
ENV AVX__CLUSTER=default
ENV AVX__MESH=istio
ENV RUST_LOG=avx_mcp=info

# Expose port (if needed in the future)
# EXPOSE 8080

# Default command
CMD ["avx-cli", "mcp", "serve"]
