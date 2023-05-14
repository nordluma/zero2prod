# Builder Stage
# Use latest Rust stable release as base image
FROM rust:1.69.0 AS builder

# Change working directory to "app" (equivalent to `cd app`)
# The folder will be created if it doesn't exist already
WORKDIR /app

# Install required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y

# Copy all files from working environment to our Docker Image
COPY . .

# Set SQLX to offline mode
ENV SQLX_OFFLINE true

# Build binary
# We'll build with the release profile to make it "blazingly" fast
RUN cargo build --release

# Runtime Stage
FROM debian:bullseye-slim AS runtime

WORKDIR /app

# Install OpenSSL - needed for our dependencies

# Install ca-certificates - needed to verify TLS certificates
# when establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder environment
# to our runtime environment
COPY --from=builder /app/target/release/zero2prod zero2prod

# Copy the configuration file for runtime
COPY configuration configuration

# Set environment to production
ENV APP_ENVIRONMENT production

# When `docker run` is executed, launch the binary
ENTRYPOINT [ "./zero2prod" ]
