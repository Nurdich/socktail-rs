//! XOR obfuscation for auth keys

const XOR_KEY: &[u8] = b"747sg^8N0$";

/// Encode data using XOR
pub fn xor_encode(data: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ XOR_KEY[i % XOR_KEY.len()])
        .collect()
}

/// Decode data using XOR (XOR is symmetric)
pub fn xor_decode(data: &[u8]) -> Vec<u8> {
    xor_encode(data)
}

/// Deobfuscate an auth key
pub fn deobfuscate_authkey(obfuscated: &[u8]) -> String {
    let decoded = xor_decode(obfuscated);
    String::from_utf8_lossy(&decoded).to_string()
}

/// Embedded obfuscated key (fallback)
const EMBEDDED_OBFUSCATED_KEY: &[u8] = &[
    0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x72, 0x65, 0x21, 0x20, 0x47, 0x65,
    0x6e, 0x65, 0x72, 0x61, 0x6c, 0x20, 0x4b, 0x65, 0x6e, 0x6f, 0x62, 0x69, 0x2e,
];

/// Get the default auth key
///
/// Priority:
/// 1. Build-time embedded key (from AUTH_KEY env var)
/// 2. Fallback embedded key
pub fn get_default_authkey() -> String {
    // Try build-time embedded key first
    if let Ok(embedded_hex) = std::env::var("EMBEDDED_AUTH_KEY") {
        if let Ok(obfuscated) = hex::decode(&embedded_hex) {
            return deobfuscate_authkey(&obfuscated);
        }
    }

    // Fallback to default
    deobfuscate_authkey(EMBEDDED_OBFUSCATED_KEY)
}

/// Get the default control URL (if embedded at build time)
pub fn get_default_control_url() -> Option<String> {
    std::env::var("EMBEDDED_CONTROL_URL").ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_symmetry() {
        let original = b"tskey-auth-test-1234567890";
        let encoded = xor_encode(original);
        let decoded = xor_decode(&encoded);
        assert_eq!(original, decoded.as_slice());
    }

    #[test]
    fn test_deobfuscate_default() {
        // Test that deobfuscation works (actual value is placeholder)
        let result = deobfuscate_authkey(EMBEDDED_OBFUSCATED_KEY);
        assert!(!result.is_empty());
        // Test that encoding the result gives us back the embedded key
        let re_encoded = xor_encode(result.as_bytes());
        assert_eq!(re_encoded, EMBEDDED_OBFUSCATED_KEY);
    }

    #[test]
    fn test_hex_roundtrip() {
        let key = "test-key-123";
        let encoded = xor_encode(key.as_bytes());
        let hex = hex::encode(&encoded);

        let decoded_bytes = hex::decode(&hex).unwrap();
        let decoded = xor_decode(&decoded_bytes);
        assert_eq!(key.as_bytes(), decoded.as_slice());
    }
}
