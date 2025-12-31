# Justfile
# This Justfile defines common tasks for building and managing the Rust project.

# Define the target directories for debug and release builds
target := justfile_directory() / "target"

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

# Task to lint the code using Clippy
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Task to run tests
test:
    cargo test

# Task to automatically fix linting issues
fix:
    cargo fix --allow-dirty --allow-staged
    cargo fmt --all

# Task to build the project in debug mode
build: generate
    cargo build

# Task to build the project in release mode
build-release: generate
    cargo build --release

# Task to run the project in debug mode with the appropriate PATH
run *args:
    cargo build -p daemon --bin ffit-daemon
    PATH="{{target}}/debug:$PATH" cargo run -p ctl --bin ffit -- {{args}}

# Task to run the project in release mode with the appropriate PATH
run-release *args:
    cargo build -p daemon --bin ffit-daemon --release
    PATH="{{target}}/release:$PATH" cargo run -p ctl --bin ffit --release -- {{args}}

# Task to run the daemon in debug mode
run-daemon *args:
    cargo run -p daemon --bin ffit-daemon -- {{args}}

# Task to run the daemon in release mode
run-daemon-release *args:
    cargo run -p daemon --bin ffit-daemon --release -- {{args}}

# Task to run the project in debug mode with the appropriate PATH
bin-run *args:
    PATH="{{target}}/debug:$PATH" ffit {{args}}

# Task to run the project in release mode with the appropriate PATH
bin-run-release *args:
    PATH="{{target}}/release:$PATH" ffit {{args}}

# Task to run the daemon in debug mode with the appropriate PATH
bin-run-daemon *args:
    PATH="{{target}}/debug:$PATH" ffit-daemon {{args}}

# Task to run the daemon in release mode with the appropriate PATH
bin-run-daemon-release *args:
    PATH="{{target}}/release:$PATH" ffit-daemon {{args}}
