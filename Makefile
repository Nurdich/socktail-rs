.PHONY: build build-with-key build-all clean test lint run help

# Default target
all: build

# Build in development mode
build:
	cargo build

# Build in release mode
release:
	cargo build --release

# Build with embedded auth key
build-with-key:
	@if [ -z "$(AUTH_KEY)" ]; then \
		echo "Error: AUTH_KEY not specified"; \
		echo "Usage: make build-with-key AUTH_KEY=tskey-auth-xxxxx"; \
		exit 1; \
	fi
	AUTH_KEY=$(AUTH_KEY) cargo build --release

# Build with custom config (auth key + control URL)
build-with-config:
	@if [ -z "$(AUTH_KEY)" ]; then \
		echo "Error: AUTH_KEY not specified"; \
		echo "Usage: make build-with-config AUTH_KEY=tskey-auth-xxxxx CONTROL_URL=https://headscale.example.com"; \
		exit 1; \
	fi
	AUTH_KEY=$(AUTH_KEY) CONTROL_URL=$(CONTROL_URL) cargo build --release

# Build for all platforms (requires cross)
build-all-with-key:
	@if [ -z "$(AUTH_KEY)" ]; then \
		echo "Error: AUTH_KEY not specified"; \
		exit 1; \
	fi
	@echo "Building for all platforms..."
	@mkdir -p dist
	# Linux x86_64
	AUTH_KEY=$(AUTH_KEY) cargo build --release --target x86_64-unknown-linux-musl
	# macOS x86_64
	AUTH_KEY=$(AUTH_KEY) cargo build --release --target x86_64-apple-darwin
	# Windows x86_64
	AUTH_KEY=$(AUTH_KEY) cargo build --release --target x86_64-pc-windows-gnu
	@echo "✅ All builds completed!"

# Clean build artifacts
clean:
	cargo clean
	rm -rf dist/

# Run tests
test:
	cargo test --all-features

# Run benchmarks
bench:
	cargo bench

# Lint code
lint:
	cargo clippy --all-features -- -D warnings
	cargo fmt --check

# Format code
fmt:
	cargo fmt

# Run in development mode (no VPN)
run:
	cargo run -- --no-vpn --verbose

# Run with custom listen address
run-custom:
	cargo run -- --no-vpn --verbose --listen $(LISTEN)

# Install locally
install:
	cargo install --path .

# Build optimized for size
build-small:
	cargo build --profile release-small

# Show binary size
size:
	@ls -lh target/release/socktail 2>/dev/null || echo "Build first: make release"

# Generate documentation
doc:
	cargo doc --no-deps --open

# Security audit
audit:
	cargo audit

# Help
help:
	@echo "SockTail Rust - Makefile targets:"
	@echo ""
	@echo "  make build                - Build in development mode"
	@echo "  make release              - Build in release mode"
	@echo "  make build-with-key       - Build with embedded auth key"
	@echo "                              Usage: make build-with-key AUTH_KEY=xxx"
	@echo "  make build-with-config    - Build with auth key and control URL"
	@echo "  make test                 - Run tests"
	@echo "  make lint                 - Run linters (clippy + rustfmt)"
	@echo "  make run                  - Run in dev mode (no VPN)"
	@echo "  make clean                - Clean build artifacts"
	@echo "  make install              - Install locally"
	@echo "  make doc                  - Generate and open documentation"
	@echo ""
