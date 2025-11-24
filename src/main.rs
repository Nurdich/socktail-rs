//! SockTail - SOCKS5 proxy over Tailscale VPN

use anyhow::Result;
use clap::Parser;
use socktail::socks5::server::Socks5Server;
use socktail::{crypto, utils};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[cfg(feature = "native-tailscale")]
use socktail::vpn::tailscale_native::TailscaleNative;
#[cfg(not(feature = "native-tailscale"))]
use socktail::vpn::tailscale::TailscaleManager;

#[derive(Parser, Debug)]
#[command(name = "socktail")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "SOCKS5 proxy over Tailscale VPN", long_about = None)]
struct Args {
    /// SOCKS5 server listen address
    #[arg(short, long, default_value = "127.0.0.1:1080")]
    listen: String,

    /// Tailscale hostname (auto-generated if not specified)
    #[arg(short = 'H', long)]
    hostname: Option<String>,

    /// Tailscale auth key (uses embedded key if not specified)
    #[arg(short, long, env = "TAILSCALE_AUTH_KEY")]
    authkey: Option<String>,

    /// Control server URL (for Headscale, etc.)
    #[arg(short, long)]
    control_url: Option<String>,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Skip Tailscale connection (development mode)
    #[arg(long)]
    no_vpn: bool,
}

fn init_logging(verbose: bool) {
    let filter = if verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("info")
    };

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    init_logging(args.verbose);

    info!("🦀 Starting SockTail v{}", env!("CARGO_PKG_VERSION"));

    // Get configuration
    let hostname = args
        .hostname
        .unwrap_or_else(|| utils::hostname::get_or_generate());

    let authkey = args
        .authkey
        .unwrap_or_else(|| crypto::xor::get_default_authkey());

    let control_url = args
        .control_url
        .or_else(|| crypto::xor::get_default_control_url());

    info!("Hostname: {}", hostname);
    if let Some(url) = &control_url {
        info!("Control server: {}", url);
    } else {
        info!("Control server: default Tailscale");
    }

    // Connect to Tailscale (unless in dev mode)
    if !args.no_vpn {
        #[cfg(feature = "native-tailscale")]
        {
            // Use native libtailscale API
            info!("Using native Tailscale implementation (libtailscale)");

            let mut ts = TailscaleNative::new()?;

            // Configure Tailscale
            ts.set_hostname(&hostname)?;
            ts.set_authkey(&authkey)?;

            if let Some(ref url) = control_url {
                ts.set_control_url(url)?;
            }

            // Set ephemeral mode for cleaner behavior
            ts.set_ephemeral(true)?;

            // Connect
            ts.connect()?;

            // Get and display IP addresses
            match ts.get_ips() {
                Ok(ips) => {
                    info!("✅ Tailscale IPs: {}", ips);
                }
                Err(e) => {
                    error!("Failed to get Tailscale IPs: {}", e);
                }
            }

            // Set up graceful shutdown
            let ts = Arc::new(Mutex::new(ts));
            let ts_clone = ts.clone();

            tokio::spawn(async move {
                tokio::signal::ctrl_c().await.ok();
                info!("🛑 Shutting down...");
                let mut mgr = ts_clone.lock().await;
                let _ = mgr.disconnect();
                std::process::exit(0);
            });
        }

        #[cfg(not(feature = "native-tailscale"))]
        {
            // Fallback to CLI-based implementation
            info!("Using CLI-based Tailscale implementation");
            info!("⚠️  For better performance, rebuild with native-tailscale feature");

            use socktail::vpn::tailscale::TailscaleManager;

            if !TailscaleManager::is_available() {
                error!("Tailscale CLI not found!");
                error!("Install tailscale or rebuild with native support:");
                error!("  cargo build --release --features native-tailscale");
                anyhow::bail!("Tailscale not available");
            }

            let mut ts_manager = TailscaleManager::new(hostname, authkey, control_url);
            ts_manager.connect()?;

            match ts_manager.status() {
                Ok(status) => {
                    info!("Tailscale status: {}", status.backend_state);
                    if let Some(node) = status.self_node {
                        info!("Node: {} (online: {})", node.hostname, node.online);
                    }
                }
                Err(e) => {
                    error!("Failed to get Tailscale status: {}", e);
                }
            }

            let ts_manager = Arc::new(Mutex::new(ts_manager));
            let ts_manager_clone = ts_manager.clone();

            tokio::spawn(async move {
                tokio::signal::ctrl_c().await.ok();
                info!("🛑 Shutting down...");
                let mut mgr = ts_manager_clone.lock().await;
                let _ = mgr.disconnect();
                std::process::exit(0);
            });
        }
    } else {
        info!("⚠️  Running in dev mode (no VPN)");
    }

    // Start SOCKS5 server
    info!("🚀 Starting SOCKS5 server on {}", args.listen);
    let server = Socks5Server::new(args.listen);
    server.run().await?;

    Ok(())
}
