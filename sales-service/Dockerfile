FROM rust:1.82.0 AS builder

# Install protoc (required for compiling .proto files)
RUN apt-get update && apt-get install -y protobuf-compiler

# Set the working directory inside the container
WORKDIR /usr/src/sales-service

# Copy the workspace-level Cargo files
COPY Cargo.lock /usr/src/Cargo.lock

# Copy the package-specific Cargo.toml
COPY sales-service/Cargo.toml ./

# Copy the proto files to the workspace
COPY proto /usr/src/proto

# Copy the entire package source code, including the .env file
COPY sales-service/ .

# Build dependencies separately to cache them if they don’t change
RUN cargo fetch

# Build the application in release mode
RUN cargo build --release --package sales-service

# Use a smaller base image for running the compiled app
FROM debian:bullseye-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/sales-service/target/release/sales-service /usr/local/bin/sales-service

# Copy the .env file to the runtime container
COPY --from=builder /usr/src/sales-service/.env /usr/local/bin/.env

EXPOSE 8080
EXPOSE 443

ENTRYPOINT ["sales-service"]