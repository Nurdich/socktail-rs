//! Tailscale VPN integration

use anyhow::{Context, Result};
use serde::Deserialize;
use std::process::{Command, Stdio};
use tracing::{debug, error, info};

pub struct TailscaleManager {
    hostname: String,
    authkey: String,
    control_url: Option<String>,
    connected: bool,
}

impl TailscaleManager {
    pub fn new(hostname: String, authkey: String, control_url: Option<String>) -> Self {
        Self {
            hostname,
            authkey,
            control_url,
            connected: false,
        }
    }

    /// Check if tailscale command is available
    pub fn is_available() -> bool {
        Command::new("tailscale")
            .arg("version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }

    pub fn connect(&mut self) -> Result<()> {
        info!("Connecting to Tailscale network...");
        debug!("Hostname: {}", self.hostname);

        let mut cmd = Command::new("tailscale");
        cmd.args(&["up", "--authkey", &self.authkey, "--hostname", &self.hostname]);

        if let Some(url) = &self.control_url {
            cmd.args(&["--login-server", url]);
            debug!("Using custom control server: {}", url);
        }

        let output = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .context("Failed to execute tailscale command")?;

        if output.status.success() {
            self.connected = true;
            info!("Successfully connected to Tailscale network");
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("Tailscale connection failed: {}", stderr);
            anyhow::bail!("Tailscale connection failed: {}", stderr)
        }
    }

    pub fn status(&self) -> Result<TailscaleStatus> {
        let output = Command::new("tailscale")
            .args(&["status", "--json"])
            .output()
            .context("Failed to get Tailscale status")?;

        if !output.status.success() {
            anyhow::bail!("Failed to get status");
        }

        let status: TailscaleStatus =
            serde_json::from_slice(&output.stdout).context("Failed to parse status JSON")?;

        Ok(status)
    }

    pub fn disconnect(&mut self) -> Result<()> {
        if !self.connected {
            return Ok(());
        }

        info!("Disconnecting from Tailscale...");

        Command::new("tailscale")
            .arg("down")
            .output()
            .context("Failed to disconnect from Tailscale")?;

        self.connected = false;
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }
}

#[derive(Debug, Deserialize)]
pub struct TailscaleStatus {
    #[serde(rename = "BackendState")]
    pub backend_state: String,

    #[serde(rename = "Self")]
    pub self_node: Option<TailscaleNode>,
}

#[derive(Debug, Deserialize)]
pub struct TailscaleNode {
    #[serde(rename = "HostName")]
    pub hostname: String,

    #[serde(rename = "Online")]
    pub online: bool,
}

impl Drop for TailscaleManager {
    fn drop(&mut self) {
        if let Err(e) = self.disconnect() {
            error!("Error disconnecting Tailscale: {}", e);
        }
    }
}
