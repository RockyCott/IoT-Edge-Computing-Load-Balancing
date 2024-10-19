# Use an official Rust image as the base image
FROM rust:latest AS builder

# Create new project
RUN USER=root cargo new --bin axum_node
WORKDIR /axum_node

# Copy the Cargo.toml file for dependency resolution
COPY Cargo.toml ./
COPY src ./

# Download the dependencies without compiling the application
RUN cargo fetch

# Copy the source code
COPY . .

# Install cmake
RUN apt-get update && apt-get install -y cmake && rm -rf /var/lib/apt/lists/*

# Compile the application in release mode
RUN cargo build --release

# Create a lighter image using ubuntu
FROM ubuntu:latest

# Insatll openssl and build-essential
RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y openssl && apt-get install -y build-essential && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /axum_node

# Copy the binary from the builder image
COPY --from=builder /axum_node/target/release/axum_node ./axum_node

# Expose the port that Axum uses
EXPOSE 3000

# Execute the application
CMD ["./axum_node"]