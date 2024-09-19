# Stage 1: Build the Rust application
FROM rust:1.81.0 AS builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Install protobuf compiler (protoc)
RUN apt-get update && \
    apt-get install -y protobuf-compiler && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy the Cargo.toml and Cargo.lock files to fetch dependencies
COPY Cargo.toml Cargo.lock ./

# Pre-fetch the dependencies to speed up the build process
RUN cargo fetch

# Copy the remaining source code
COPY . .

# Build the application in release mode
RUN cargo build --release

# Stage 2: Create the final image with the necessary dependencies
FROM debian:bookworm-slim

# Update package lists and install necessary dependencies for running the application
RUN apt-get update && \
    apt-get install -y \
    curl \
    libssl3 \
    libssl-dev \
    pkg-config \
    ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage to the final image
COPY --from=builder /usr/src/app/target/release/oracle_binance /usr/local/bin/oracle_binance

# Set the working directory for the application
WORKDIR /usr/src/app

# Copy the configuration file into the container
COPY resources/config.toml ./resources/

# Command to run the application
CMD ["oracle_binance"]