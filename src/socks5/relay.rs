//! Data relay between client and target

use tokio::io::{self, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::debug;

/// Relay data bidirectionally between client and target
pub async fn relay_data(client: TcpStream, target: TcpStream) -> io::Result<()> {
    let (mut client_read, mut client_write) = io::split(client);
    let (mut target_read, mut target_write) = io::split(target);

    let client_to_target = async {
        let bytes = io::copy(&mut client_read, &mut target_write).await?;
        debug!("Client -> Target: {} bytes", bytes);
        target_write.shutdown().await?;
        Ok::<_, io::Error>(bytes)
    };

    let target_to_client = async {
        let bytes = io::copy(&mut target_read, &mut client_write).await?;
        debug!("Target -> Client: {} bytes", bytes);
        client_write.shutdown().await?;
        Ok::<_, io::Error>(bytes)
    };

    // Run both directions concurrently
    match tokio::try_join!(client_to_target, target_to_client) {
        Ok((c2t, t2c)) => {
            debug!("Relay completed: {} bytes up, {} bytes down", c2t, t2c);
            Ok(())
        }
        Err(e) => {
            debug!("Relay error: {}", e);
            Err(e)
        }
    }
}
