# Use Rust official image for building
FROM rust:1.70 as builder

WORKDIR /usr/src/rust_compress_api

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create a dummy src/lib.rs file to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs

# Build dependencies only (for caching)
RUN cargo build --release
RUN rm -f target/release/rust_compress_api*

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Use Debian slim image for runtime
FROM debian:bullseye-slim

# Install ca-certificates for HTTPS requests
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -ms /bin/bash app

# Set working directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /usr/src/rust_compress_api/target/release/rust_compress_api ./rust_compress_api

# Copy .env file
COPY .env .env

# Change ownership of files to app user
RUN chown -R app:app /app

# Switch to app user
USER app

# Expose port
EXPOSE 3000

# Run the application
CMD ["./rust_compress_api"]
