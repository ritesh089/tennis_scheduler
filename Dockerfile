# Dockerfile
FROM rust:1.83.0 as builder
WORKDIR /app
# Copy your source code into the container
COPY . .
# Build the release version of your application
RUN cargo build --release

# Use Ubuntu 22.04 as the runtime image so that glibc is new enough.
FROM ubuntu:22.04
WORKDIR /app
# Install PostgreSQL client libraries (libpq)
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*
# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/tennis_scheduler .
# Expose the port your app listens on
EXPOSE 8080
# Run the application
CMD ["./tennis_scheduler"]
