//! SOCKS5 protocol implementation
//!
//! This module provides a complete SOCKS5 proxy server implementation
//! with support for IPv4, IPv6, and domain name resolution.

pub mod protocol;
pub mod server;
pub mod relay;

pub use protocol::{AuthRequest, ConnectRequest, TargetAddr};
pub use server::Socks5Server;
