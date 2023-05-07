# Use latest Rust stable release as base image
FROM rust:1.69.0

# Change working directory to "app" (equivalent to `cd app`)
# The folder will be created if it doesn't exist already
WORKDIR /app

# Install required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y

# Copy all files from working enviroment to our Docker Image
COPY . .

# Build binary
# We'll build with the release profile to make it "blazingly" fast
RUN cargo build --release

# When `docker run` is executed, launch the binary
ENTRYPOINT [ "./target/release/zero2prod" ]
