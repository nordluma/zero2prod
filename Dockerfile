# Use latest Rust stable release as base image
FROM rust:1.69.0

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

# Set environment to production
ENV APP_ENVIRONMENT production

# When `docker run` is executed, launch the binary
ENTRYPOINT [ "./target/release/zero2prod" ]
