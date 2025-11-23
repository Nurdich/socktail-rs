//! SockTail - SOCKS5 proxy over Tailscale VPN
//!
//! # Overview
//!
//! SockTail provides a fast, secure SOCKS5 proxy server that routes traffic
//! through Tailscale VPN. It's designed for simplicity and performance.
//!
//! # Examples
//!
//! ```no_run
//! use socktail::socks5::server::Socks5Server;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let server = Socks5Server::new("127.0.0.1:1080".to_string());
//!     server.run().await?;
//!     Ok(())
//! }
//! ```

/// SOCKS5 protocol implementation
pub mod socks5;

/// VPN integration (Tailscale)
pub mod vpn;

/// Cryptographic utilities (XOR obfuscation)
pub mod crypto;

/// Utility functions
pub mod utils;
