.PHONY: help build run release test fmt clippy clean install audit doc all

# Default target - show help
help:
	@echo "Password Manager - Available targets:"
	@echo ""
	@echo "  make help       - Show this help message"
	@echo "  make build      - Build the project in debug mode"
	@echo "  make run        - Run the project in debug mode"
	@echo "  make release    - Build optimized release binary"
	@echo "  make test       - Run all tests"
	@echo "  make fmt        - Format code with rustfmt"
	@echo "  make clippy     - Run clippy linter"
	@echo "  make clean      - Remove build artifacts"
	@echo "  make install    - Install binary to ~/.cargo/bin"
	@echo "  make audit      - Run security audit (requires cargo-audit)"
	@echo "  make doc        - Generate and open documentation"
	@echo "  make all        - Format, clippy, test, and build"
	@echo ""

# Build in debug mode
build:
	@echo "Building in debug mode..."
	cargo build

# Run in debug mode
run:
	@echo "Running password manager..."
	cargo run

# Build optimized release binary
release:
	@echo "Building release binary..."
	cargo build --release
	@echo "Binary location: target/release/password_manager"

# Run all tests
test:
	@echo "Running tests..."
	cargo test

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt

# Run clippy
clippy:
	@echo "Running clippy..."
	cargo clippy -- -D warnings

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

# Install to system
install:
	@echo "Installing to ~/.cargo/bin..."
	cargo install --path .

# Security audit (requires cargo-audit: cargo install cargo-audit)
audit:
	@echo "Running security audit..."
	@command -v cargo-audit >/dev/null 2>&1 || { echo "cargo-audit not found. Install with: cargo install cargo-audit"; exit 1; }
	cargo audit

# Generate and open documentation
doc:
	@echo "Generating documentation..."
	cargo doc --open

# Complete check: format, lint, test, build
all: fmt clippy test build
	@echo "All checks passed!"
