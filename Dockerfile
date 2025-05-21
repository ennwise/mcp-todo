# Stage 1: Build the Rust application
FROM rust:1.87-bullseye AS builder

# Create a new empty shell project
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
# Build a dummy project to cache dependencies
RUN cargo build --release --locked
RUN rm -rf src target

# Copy the actual source code
COPY src ./src

# Build the application
RUN cargo build --release --locked --target-dir /usr/src/app/target --bin rustquote_service

# Stage 2: Create the runtime image
FROM debian:bullseye-slim AS runtime


# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/rustquote_service /usr/local/bin/rustquote_service

# Copy application data
COPY data ./data

# Expose the application port (assuming 3000 for now)
EXPOSE 3000

# Define the command to run the application
CMD ["/usr/local/bin/rustquote_service"]