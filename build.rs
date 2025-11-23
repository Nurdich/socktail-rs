use std::env;

fn main() {
    // Read embedded auth key from environment variable at build time
    if let Ok(auth_key) = env::var("AUTH_KEY") {
        let obfuscated = xor_encode(auth_key.as_bytes());
        let hex_encoded = hex::encode(&obfuscated);
        println!("cargo:rustc-env=EMBEDDED_AUTH_KEY={}", hex_encoded);
    }

    // Read control URL from environment variable
    if let Ok(control_url) = env::var("CONTROL_URL") {
        println!("cargo:rustc-env=EMBEDDED_CONTROL_URL={}", control_url);
    }

    println!("cargo:rerun-if-env-changed=AUTH_KEY");
    println!("cargo:rerun-if-env-changed=CONTROL_URL");
}

fn xor_encode(data: &[u8]) -> Vec<u8> {
    const XOR_KEY: &[u8] = b"747sg^8N0$";
    data.iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ XOR_KEY[i % XOR_KEY.len()])
        .collect()
}
