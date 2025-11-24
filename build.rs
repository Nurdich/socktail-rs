use std::env;
use std::path::PathBuf;
use std::process::Command;

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

    // Build libtailscale if native-tailscale feature is enabled
    #[cfg(feature = "native-tailscale")]
    build_libtailscale();
}

#[cfg(feature = "native-tailscale")]
fn build_libtailscale() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let libtailscale_dir = out_dir.join("libtailscale");

    // Check if libtailscale directory exists
    if !libtailscale_dir.exists() {
        println!("cargo:warning=Cloning libtailscale repository...");

        // Clone libtailscale repository
        let status = Command::new("git")
            .args(&[
                "clone",
                "https://github.com/tailscale/libtailscale.git",
                libtailscale_dir.to_str().unwrap(),
            ])
            .status();

        if status.is_err() || !status.unwrap().success() {
            println!("cargo:warning=Failed to clone libtailscale. Install manually or use CLI mode.");
            println!("cargo:warning=See: https://github.com/tailscale/libtailscale");
            return;
        }
    }

    // Build libtailscale static library
    println!("cargo:warning=Building libtailscale (this may take a while on first build)...");

    let status = Command::new("make")
        .arg("archive")
        .current_dir(&libtailscale_dir)
        .status();

    if status.is_err() || !status.unwrap().success() {
        println!("cargo:warning=Failed to build libtailscale.");
        println!("cargo:warning=Make sure Go is installed and in PATH.");
        println!("cargo:warning=Falling back to CLI mode.");
        return;
    }

    // Link the library
    println!("cargo:rustc-link-search=native={}", libtailscale_dir.display());
    println!("cargo:rustc-link-lib=static=tailscale");

    // Link required system libraries for Go runtime
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=pthread");
        println!("cargo:rustc-link-lib=dl");
    }

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=Security");
    }

    println!("cargo:warning=libtailscale built successfully!");
    println!("cargo:rerun-if-changed={}", libtailscale_dir.display());
}

fn xor_encode(data: &[u8]) -> Vec<u8> {
    const XOR_KEY: &[u8] = b"747sg^8N0$";
    data.iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ XOR_KEY[i % XOR_KEY.len()])
        .collect()
}
