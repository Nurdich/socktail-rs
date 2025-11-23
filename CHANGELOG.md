# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-11-23

## [0.1.0] - 2025-11-23

### Added
- Initial Rust implementation
- SOCKS5 proxy server with async/await (Tokio)
- Support for IPv4, IPv6, and domain names
- Tailscale VPN integration via CLI
- XOR key obfuscation (compatible with Go version)
- Build-time auth key embedding
- Graceful shutdown handling
- Structured logging with tracing
- Comprehensive CLI with clap
- Development mode (--no-vpn flag)
- Cross-platform support (Linux/macOS/Windows)
- Unit tests for all modules
- GitHub Actions CI/CD
- Makefile for common tasks

### Performance
- Zero-copy data relay with Tokio
- Small binary size (~1-3 MB with optimizations)
- Low memory footprint (~5 MB)
- Fast startup (<500ms)

## [0.1.0] - 2025-11-23

### Added
- Initial release
- Feature parity with Go version
- Better performance and smaller binary size
