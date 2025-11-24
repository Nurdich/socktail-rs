//! VPN integration (Tailscale)

pub mod tailscale;

#[cfg(feature = "native-tailscale")]
pub mod libtailscale;
#[cfg(feature = "native-tailscale")]
pub mod tailscale_native;

pub use tailscale::TailscaleManager;

#[cfg(feature = "native-tailscale")]
pub use tailscale_native::TailscaleNative;

