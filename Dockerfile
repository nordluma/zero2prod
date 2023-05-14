# Cargo Chef
FROM lukemathwalker/cargo-chef:latest-rust-1.69.0 as chef

WORKDIR /app

RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .

# Compute a lock-like fil for our project
RUN cargo chef prepare --recipe-path recipe.json

# Builder Stage
FROM chef as builder

COPY --from=planner /app/recipe.json recipe.json

# Build project dependencies, not the application 
RUN cargo chef cook --release --recipe-path recipe.json

# Until this point the dependency tree stays the same,
# all layers should be cached
COPY . .

# Set SQLX to offline mode
ENV SQLX_OFFLINE true

# Build binary
# We'll build with the release profile to make it "blazingly" fast
RUN cargo build --release --bin zero2prod

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
