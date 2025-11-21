set shell := ["powershell", "-Command"]

# Build all crates
build:
    cargo build --workspace --release

# Run tests
test:
    cargo test --workspace

# Format code
fmt:
    cargo fmt --all

# Lint code
lint:
    cargo clippy --workspace -- -D warnings

# Clean build artifacts
clean:
    cargo clean

# Build FFI library
build-ffi:
    cargo build --package librepods-ffi --release --lib

# Build CLI
build-cli:
    cargo build --package librepods-cli --release --bin librepods

# Run CLI
run-cli:
    cargo run --package librepods-cli --release

# Full check
check: fmt lint test
    @echo "All checks passed!"
