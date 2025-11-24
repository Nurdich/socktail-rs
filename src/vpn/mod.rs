//! VPN integration (Tailscale) - Pure Rust implementation

pub mod tailscale_rust;

// Re-export pure Rust implementation as the default
pub use tailscale_rust::TailscaleRust;

// Type alias for backward compatibility
pub type TailscaleNative = TailscaleRust;
