# Use an official Rust image as the base for building the application
FROM rust:1.82.0 AS builder

# Install protoc (required for compiling .proto files)
RUN apt-get update && apt-get install -y protobuf-compiler

# Set the working directory inside the container
WORKDIR /usr/src/catalog-service

# Copy the workspace-level Cargo files
COPY Cargo.lock /usr/src/Cargo.lock

COPY Cargo.toml /usr/src/Cargo.toml

# Copy the package-specific Cargo.toml
COPY catalog-service/Cargo.toml ./

# Copy the proto files to the workspace
COPY proto /usr/src/proto

# Copy the entire package source code, including the .env file
COPY catalog-service/ .

# Run unit tests before proceeding with the build
RUN cargo test --release

# Build dependencies separately to cache them if they don’t change
RUN cargo fetch

# Build the application in release mode
RUN cargo build --release --package catalog-service

# Use a smaller base image for running the compiled app
FROM debian:bullseye-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/catalog-service/target/release/catalog-service /usr/local/bin/catalog-service

# Copy the .env file to the runtime container
COPY --from=builder /usr/src/catalog-service/.env /usr/local/bin/.env

# Expose the necessary port
EXPOSE 8080
EXPOSE 443

# Run the binary by default when the container starts
ENTRYPOINT ["catalog-service"]