# Set the base image to the official Rust image
FROM rust:buster AS build

# Set the working directory
WORKDIR /usr/src/gazouilleur

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

RUN cargo fetch

# Copy the source code to the container
COPY src/ ./src/

# Build the application
RUN cargo build --release

# Create a new stage for the runtime image
FROM alpine:latest as runtime

# Copy the binary from the build stage to the runtime image
COPY --from=build /usr/src/gazouilleur/target/release/gazouilleur /usr/local/bin/gazouilleur

EXPOSE 8000

# Start the application
CMD ["/usr/local/bin/gazouilleur"]
