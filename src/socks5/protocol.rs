//! SOCKS5 protocol definitions and parsing

use bytes::{Buf, BytesMut};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use thiserror::Error;

// Protocol constants
pub const SOCKS5_VERSION: u8 = 0x05;

// Authentication methods
pub const AUTH_NO_AUTH: u8 = 0x00;
pub const AUTH_GSSAPI: u8 = 0x01;
pub const AUTH_USERNAME_PASSWORD: u8 = 0x02;
pub const AUTH_NO_ACCEPTABLE: u8 = 0xFF;

// Commands
pub const CMD_CONNECT: u8 = 0x01;
pub const CMD_BIND: u8 = 0x02;
pub const CMD_UDP_ASSOCIATE: u8 = 0x03;

// Address types
pub const ATYP_IPV4: u8 = 0x01;
pub const ATYP_DOMAIN: u8 = 0x03;
pub const ATYP_IPV6: u8 = 0x04;

// Reply codes
pub const REP_SUCCESS: u8 = 0x00;
pub const REP_GENERAL_FAILURE: u8 = 0x01;
pub const REP_CONNECTION_NOT_ALLOWED: u8 = 0x02;
pub const REP_NETWORK_UNREACHABLE: u8 = 0x03;
pub const REP_HOST_UNREACHABLE: u8 = 0x04;
pub const REP_CONNECTION_REFUSED: u8 = 0x05;
pub const REP_TTL_EXPIRED: u8 = 0x06;
pub const REP_COMMAND_NOT_SUPPORTED: u8 = 0x07;
pub const REP_ADDRESS_TYPE_NOT_SUPPORTED: u8 = 0x08;

#[derive(Debug, Error)]
pub enum Socks5Error {
    #[error("Unsupported SOCKS version: {0}")]
    UnsupportedVersion(u8),

    #[error("Unsupported command: {0}")]
    UnsupportedCommand(u8),

    #[error("Unsupported address type: {0}")]
    UnsupportedAddressType(u8),

    #[error("Authentication failed")]
    AuthFailed,

    #[error("Connection refused")]
    ConnectionRefused,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid protocol data")]
    InvalidData,
}

pub type Result<T> = std::result::Result<T, Socks5Error>;

/// Target address (IP or domain)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetAddr {
    Ip(SocketAddr),
    Domain(String, u16),
}

impl TargetAddr {
    pub fn to_string(&self) -> String {
        match self {
            TargetAddr::Ip(addr) => addr.to_string(),
            TargetAddr::Domain(domain, port) => format!("{}:{}", domain, port),
        }
    }
}

impl std::fmt::Display for TargetAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// Authentication request from client
pub struct AuthRequest {
    pub version: u8,
    pub methods: Vec<u8>,
}

impl AuthRequest {
    pub fn parse(buf: &mut BytesMut) -> Result<Self> {
        if buf.len() < 2 {
            return Err(Socks5Error::InvalidData);
        }

        let version = buf.get_u8();
        let n_methods = buf.get_u8() as usize;

        if buf.len() < n_methods {
            return Err(Socks5Error::InvalidData);
        }

        let methods = buf.split_to(n_methods).to_vec();

        Ok(AuthRequest { version, methods })
    }

    pub fn supports_method(&self, method: u8) -> bool {
        self.methods.contains(&method)
    }
}

/// CONNECT request from client
pub struct ConnectRequest {
    pub version: u8,
    pub command: u8,
    pub target: TargetAddr,
}

impl ConnectRequest {
    pub fn parse(buf: &mut BytesMut) -> Result<Self> {
        if buf.len() < 4 {
            return Err(Socks5Error::InvalidData);
        }

        let version = buf.get_u8();
        let command = buf.get_u8();
        let _reserved = buf.get_u8();
        let atyp = buf.get_u8();

        let target = match atyp {
            ATYP_IPV4 => {
                if buf.len() < 6 {
                    return Err(Socks5Error::InvalidData);
                }
                let ip = Ipv4Addr::new(buf.get_u8(), buf.get_u8(), buf.get_u8(), buf.get_u8());
                let port = buf.get_u16();
                TargetAddr::Ip(SocketAddr::new(IpAddr::V4(ip), port))
            }
            ATYP_DOMAIN => {
                if buf.is_empty() {
                    return Err(Socks5Error::InvalidData);
                }
                let len = buf.get_u8() as usize;
                if buf.len() < len + 2 {
                    return Err(Socks5Error::InvalidData);
                }
                let domain = String::from_utf8_lossy(&buf.split_to(len)).to_string();
                let port = buf.get_u16();
                TargetAddr::Domain(domain, port)
            }
            ATYP_IPV6 => {
                if buf.len() < 18 {
                    return Err(Socks5Error::InvalidData);
                }
                let mut octets = [0u8; 16];
                buf.copy_to_slice(&mut octets);
                let ip = Ipv6Addr::from(octets);
                let port = buf.get_u16();
                TargetAddr::Ip(SocketAddr::new(IpAddr::V6(ip), port))
            }
            _ => return Err(Socks5Error::UnsupportedAddressType(atyp)),
        };

        Ok(ConnectRequest {
            version,
            command,
            target,
        })
    }
}

/// Create authentication response
pub fn auth_response(method: u8) -> [u8; 2] {
    [SOCKS5_VERSION, method]
}

/// Create connect response
pub fn connect_response(status: u8) -> Vec<u8> {
    vec![
        SOCKS5_VERSION,
        status,
        0x00,       // Reserved
        ATYP_IPV4,  // Address type
        0, 0, 0, 0, // Bind address (0.0.0.0)
        0, 0,       // Bind port (0)
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_request_parsing() {
        let mut buf = BytesMut::from(&[0x05, 0x02, 0x00, 0x02][..]);
        let auth = AuthRequest::parse(&mut buf).unwrap();

        assert_eq!(auth.version, 0x05);
        assert_eq!(auth.methods, vec![0x00, 0x02]);
        assert!(auth.supports_method(AUTH_NO_AUTH));
    }

    #[test]
    fn test_connect_ipv4() {
        let mut buf = BytesMut::from(
            &[
                0x05, 0x01, 0x00, 0x01, // Version, CMD, RSV, ATYP
                0x7F, 0x00, 0x00, 0x01, // 127.0.0.1
                0x00, 0x50, // Port 80
            ][..],
        );

        let req = ConnectRequest::parse(&mut buf).unwrap();
        assert_eq!(req.command, CMD_CONNECT);

        if let TargetAddr::Ip(addr) = req.target {
            assert_eq!(addr.port(), 80);
            assert!(addr.ip().to_string().contains("127.0.0.1"));
        } else {
            panic!("Expected IP address");
        }
    }

    #[test]
    fn test_connect_domain() {
        let mut buf = BytesMut::from(
            &[
                0x05, 0x01, 0x00, 0x03, // Version, CMD, RSV, ATYP
                0x0B, // Domain length (11)
                b'e', b'x', b'a', b'm', b'p', b'l', b'e', b'.', b'c', b'o', b'm',
                0x01, 0xBB, // Port 443
            ][..],
        );

        let req = ConnectRequest::parse(&mut buf).unwrap();

        if let TargetAddr::Domain(domain, port) = req.target {
            assert_eq!(domain, "example.com");
            assert_eq!(port, 443);
        } else {
            panic!("Expected domain");
        }
    }
}
