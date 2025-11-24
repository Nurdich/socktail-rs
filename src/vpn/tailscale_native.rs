//! Native Tailscale integration using libtailscale-rs crate

use anyhow::{Context, Result};
use libtailscale::Tailscale;
use tracing::{error, info};

pub struct TailscaleNative {
    ts: Tailscale,
    connected: bool,
}

impl TailscaleNative {
    /// Create a new Tailscale instance
    pub fn new() -> Result<Self> {
        let ts = Tailscale::new();
        Ok(Self {
            ts,
            connected: false,
        })
    }

    /// Check if libtailscale is available at runtime
    pub fn is_available() -> bool {
        // libtailscale-rs handles availability internally
        true
    }

    /// Set the hostname for this Tailscale node
    pub fn set_hostname(&mut self, hostname: &str) -> Result<()> {
        self.ts
            .set_hostname(hostname)
            .map_err(|e| anyhow::anyhow!("Failed to set hostname: {}", e))
    }

    /// Set the auth key
    pub fn set_authkey(&mut self, authkey: &str) -> Result<()> {
        self.ts
            .set_authkey(authkey)
            .map_err(|e| anyhow::anyhow!("Failed to set authkey: {}", e))
    }

    /// Set the control server URL (for Headscale, etc.)
    pub fn set_control_url(&mut self, control_url: &str) -> Result<()> {
        self.ts
            .set_control_url(control_url)
            .map_err(|e| anyhow::anyhow!("Failed to set control URL: {}", e))
    }

    /// Set state directory
    pub fn set_dir(&mut self, dir: &str) -> Result<()> {
        self.ts
            .set_dir(dir)
            .map_err(|e| anyhow::anyhow!("Failed to set directory: {}", e))
    }

    /// Set ephemeral mode (node is automatically removed when disconnected)
    pub fn set_ephemeral(&mut self, ephemeral: bool) -> Result<()> {
        self.ts
            .set_ephemeral(ephemeral)
            .map_err(|e| anyhow::anyhow!("Failed to set ephemeral mode: {}", e))
    }

    /// Connect to Tailscale and wait until ready
    pub fn connect(&mut self) -> Result<()> {
        info!("Connecting to Tailscale network via native API...");

        self.ts
            .up()
            .map_err(|e| anyhow::anyhow!("Tailscale connection failed: {}", e))?;

        self.connected = true;
        info!("Successfully connected to Tailscale network");
        Ok(())
    }

    /// Get the Tailscale loopback information
    pub fn get_loopback(&mut self) -> Result<String> {
        let loopback = self
            .ts
            .loopback()
            .map_err(|e| anyhow::anyhow!("Failed to get loopback: {}", e))?;

        Ok(format!(
            "Address: {}, Credential: {}",
            loopback.address, loopback.credential
        ))
    }

    /// Disconnect from Tailscale
    pub fn disconnect(&mut self) -> Result<()> {
        if !self.connected {
            return Ok(());
        }

        info!("Disconnecting from Tailscale...");
        // The Tailscale struct handles cleanup in Drop
        self.connected = false;
        Ok(())
    }

    /// Check if currently connected
    pub fn is_connected(&self) -> bool {
        self.connected
    }
}

impl Drop for TailscaleNative {
    fn drop(&mut self) {
        if let Err(e) = self.disconnect() {
            error!("Error disconnecting Tailscale: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_instance() {
        let ts = TailscaleNative::new();
        assert!(ts.is_ok());
    }

    #[test]
    fn test_set_hostname() {
        let mut ts = TailscaleNative::new().unwrap();
        let result = ts.set_hostname("test-node");
        assert!(result.is_ok());
    }
}
