//! Native Tailscale integration using libtailscale

use super::libtailscale as ffi;
use anyhow::{Context, Result};
use std::ffi::{CStr, CString};
use tracing::{debug, error, info};

pub struct TailscaleNative {
    handle: ffi::Tailscale,
    connected: bool,
}

impl TailscaleNative {
    /// Create a new Tailscale instance
    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::tailscale_new() };
        if handle < 0 {
            anyhow::bail!("Failed to create Tailscale instance");
        }

        Ok(Self {
            handle,
            connected: false,
        })
    }

    /// Check if libtailscale is available at runtime
    pub fn is_available() -> bool {
        // Try to create a Tailscale instance
        // If it fails, libtailscale is not linked
        unsafe { ffi::tailscale_new() >= 0 }
    }

    /// Set the hostname for this Tailscale node
    pub fn set_hostname(&self, hostname: &str) -> Result<()> {
        let c_hostname = CString::new(hostname)?;
        let result = unsafe { ffi::tailscale_set_hostname(self.handle, c_hostname.as_ptr()) };
        if result != 0 {
            anyhow::bail!("Failed to set hostname: {}", self.get_error_message());
        }
        Ok(())
    }

    /// Set the auth key
    pub fn set_authkey(&self, authkey: &str) -> Result<()> {
        let c_authkey = CString::new(authkey)?;
        let result = unsafe { ffi::tailscale_set_authkey(self.handle, c_authkey.as_ptr()) };
        if result != 0 {
            anyhow::bail!("Failed to set authkey: {}", self.get_error_message());
        }
        Ok(())
    }

    /// Set the control server URL (for Headscale, etc.)
    pub fn set_control_url(&self, control_url: &str) -> Result<()> {
        let c_url = CString::new(control_url)?;
        let result = unsafe { ffi::tailscale_set_control_url(self.handle, c_url.as_ptr()) };
        if result != 0 {
            anyhow::bail!("Failed to set control URL: {}", self.get_error_message());
        }
        Ok(())
    }

    /// Set state directory
    pub fn set_dir(&self, dir: &str) -> Result<()> {
        let c_dir = CString::new(dir)?;
        let result = unsafe { ffi::tailscale_set_dir(self.handle, c_dir.as_ptr()) };
        if result != 0 {
            anyhow::bail!("Failed to set directory: {}", self.get_error_message());
        }
        Ok(())
    }

    /// Set ephemeral mode (node is automatically removed when disconnected)
    pub fn set_ephemeral(&self, ephemeral: bool) -> Result<()> {
        let result = unsafe { ffi::tailscale_set_ephemeral(self.handle, ephemeral as i32) };
        if result != 0 {
            anyhow::bail!("Failed to set ephemeral mode: {}", self.get_error_message());
        }
        Ok(())
    }

    /// Connect to Tailscale and wait until ready
    pub fn connect(&mut self) -> Result<()> {
        info!("Connecting to Tailscale network via native API...");

        let result = unsafe { ffi::tailscale_up(self.handle) };
        if result != 0 {
            let error = self.get_error_message();
            error!("Tailscale connection failed: {}", error);
            anyhow::bail!("Tailscale connection failed: {}", error);
        }

        self.connected = true;
        info!("Successfully connected to Tailscale network");
        Ok(())
    }

    /// Get the Tailscale IP addresses assigned to this node
    pub fn get_ips(&self) -> Result<String> {
        let mut buf = vec![0u8; 256];
        let result = unsafe {
            ffi::tailscale_getips(self.handle, buf.as_mut_ptr() as *mut i8, buf.len())
        };

        if result != 0 {
            anyhow::bail!("Failed to get IPs: {}", self.get_error_message());
        }

        // Find the null terminator
        let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
        let ips = String::from_utf8_lossy(&buf[..len]).to_string();
        Ok(ips)
    }

    /// Get error message from libtailscale
    fn get_error_message(&self) -> String {
        let mut buf = vec![0u8; 512];
        let result = unsafe {
            ffi::tailscale_errmsg(self.handle, buf.as_mut_ptr() as *mut i8, buf.len())
        };

        if result == 0 {
            let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
            String::from_utf8_lossy(&buf[..len]).to_string()
        } else {
            "Unknown error".to_string()
        }
    }

    /// Disconnect from Tailscale
    pub fn disconnect(&mut self) -> Result<()> {
        if !self.connected {
            return Ok(());
        }

        info!("Disconnecting from Tailscale...");
        let result = unsafe { ffi::tailscale_close(self.handle) };
        if result != 0 {
            anyhow::bail!("Failed to disconnect: {}", self.get_error_message());
        }

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
    #[ignore] // Only run with libtailscale available
    fn test_create_instance() {
        let ts = TailscaleNative::new();
        assert!(ts.is_ok());
    }

    #[test]
    #[ignore]
    fn test_set_hostname() {
        let ts = TailscaleNative::new().unwrap();
        let result = ts.set_hostname("test-node");
        assert!(result.is_ok());
    }
}
