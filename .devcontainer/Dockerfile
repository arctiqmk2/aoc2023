FROM ubuntu:latest

# Install necessary packages
RUN apt-get update && apt-get install -y curl build-essential

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add cargo bin to PATH
ENV PATH="/root/.cargo/bin:${PATH}"
