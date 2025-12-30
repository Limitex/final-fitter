# Justfile
# This Justfile defines common tasks for building and managing the Rust project.

# Default task to list all available tasks
default:
    @just --list

# Task to generate code from Protocol Buffers definitions
generate:
    cd crates/daemon && buf generate

# Task to clean build artifacts and generated files
clean:
    cargo clean
    rm -rf crates/daemon/src/generated

# Task to build the project in debug mode
build: generate
    cargo build

# Task to build the project in release mode
release: generate
    cargo build --release

