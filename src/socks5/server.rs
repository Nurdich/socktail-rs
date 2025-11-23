//! SOCKS5 server implementation

use super::protocol::*;
use super::relay::relay_data;
use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, error, info, warn};

pub struct Socks5Server {
    listen_addr: String,
}

impl Socks5Server {
    pub fn new(listen_addr: String) -> Self {
        Self { listen_addr }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(&self.listen_addr).await?;
        info!("SOCKS5 server listening on {}", self.listen_addr);

        loop {
            match listener.accept().await {
                Ok((socket, peer_addr)) => {
                    debug!("New connection from {}", peer_addr);

                    tokio::spawn(async move {
                        if let Err(e) = handle_client(socket).await {
                            error!("Error handling client {}: {}", peer_addr, e);
                        }
                    });
                }
                Err(e) => {
                    error!("Failed to accept connection: {}", e);
                }
            }
        }
    }
}

async fn handle_client(mut client: TcpStream) -> anyhow::Result<()> {
    // 1. Authentication phase
    let mut buf = BytesMut::with_capacity(512);

    if client.read_buf(&mut buf).await? == 0 {
        return Err(anyhow::anyhow!("Connection closed"));
    }

    let auth_req = AuthRequest::parse(&mut buf)?;

    if auth_req.version != SOCKS5_VERSION {
        return Err(Socks5Error::UnsupportedVersion(auth_req.version).into());
    }

    if !auth_req.supports_method(AUTH_NO_AUTH) {
        client
            .write_all(&auth_response(AUTH_NO_ACCEPTABLE))
            .await?;
        return Err(Socks5Error::AuthFailed.into());
    }

    client.write_all(&auth_response(AUTH_NO_AUTH)).await?;
    debug!("Authentication successful");

    // 2. Request phase
    buf.clear();

    if client.read_buf(&mut buf).await? == 0 {
        return Err(anyhow::anyhow!("Connection closed after auth"));
    }

    let connect_req = ConnectRequest::parse(&mut buf)?;

    if connect_req.version != SOCKS5_VERSION {
        return Err(Socks5Error::UnsupportedVersion(connect_req.version).into());
    }

    if connect_req.command != CMD_CONNECT {
        client
            .write_all(&connect_response(REP_COMMAND_NOT_SUPPORTED))
            .await?;
        return Err(Socks5Error::UnsupportedCommand(connect_req.command).into());
    }

    let target_addr = connect_req.target.to_string();
    debug!("Connecting to target: {}", target_addr);

    // 3. Connect to target
    match TcpStream::connect(&target_addr).await {
        Ok(target) => {
            debug!("Connected to {}", target_addr);
            client.write_all(&connect_response(REP_SUCCESS)).await?;

            // 4. Relay data
            if let Err(e) = relay_data(client, target).await {
                warn!("Relay error: {}", e);
            }
        }
        Err(e) => {
            error!("Failed to connect to {}: {}", target_addr, e);
            client
                .write_all(&connect_response(REP_CONNECTION_REFUSED))
                .await?;
        }
    }

    Ok(())
}
