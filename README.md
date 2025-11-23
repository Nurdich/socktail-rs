# SockTail-RS 🦀

Fast SOCKS5 proxy over Tailscale VPN, written in Rust.

## Features

- ✅ High-performance async SOCKS5 proxy (Tokio)
- ✅ Tailscale VPN integration
- ✅ XOR key obfuscation (compatible with Go version)
- ✅ Cross-platform (Linux/macOS/Windows)
- ✅ Small binary size (~1-3 MB)
- ✅ Low memory footprint (~5 MB)

## Quick Start

### Installation

```bash
cargo install socktail
```

### Usage

```bash
# Basic usage (uses embedded key)
socktail

# Custom hostname
socktail -H my-proxy

# Custom auth key
socktail -a tskey-auth-xxxx

# With Headscale
socktail -a tskey-auth-xxxx -c https://headscale.example.com

# Development mode (skip Tailscale)
socktail --no-vpn -v
```

## Building from Source

```bash
# Development build
cargo build

# Release build
cargo build --release

# With embedded auth key
AUTH_KEY=tskey-auth-xxxxx cargo build --release

# With custom control server
AUTH_KEY=tskey-auth-xxxxx CONTROL_URL=https://headscale.example.com cargo build --release
```

## Development

```bash
# Run tests
cargo test

# Run with hot reload
cargo install cargo-watch
cargo watch -x run

# Check code
cargo clippy

# Format code
cargo fmt

# Run benchmarks
cargo bench
```

## Performance

| Metric | Value |
|--------|-------|
| Binary Size | 1-3 MB |
| Memory Usage | ~5 MB |
| Throughput | 900+ Mbps |
| Startup Time | <500ms |

## Architecture

```
socktail-rs/
├── src/
│   ├── main.rs           # Entry point
│   ├── lib.rs            # Library exports
│   ├── socks5/           # SOCKS5 protocol
│   ├── vpn/              # Tailscale integration
│   ├── crypto/           # XOR obfuscation
│   └── utils/            # Utilities
├── tests/                # Integration tests
├── benches/              # Performance benchmarks
└── examples/             # Usage examples
```

## License

MIT

## Acknowledgments

- Original Go implementation: [SockTail](https://github.com/yourusername/SockTail)
- Built with [Tokio](https://tokio.rs/)
- VPN powered by [Tailscale](https://tailscale.com/)
