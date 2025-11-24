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
    use std::path::Path;

    let project_root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir = Path::new(&project_root).join("lib");
    let lib_file = lib_dir.join("libtailscale.a");

    // Check if pre-compiled library exists in lib/ directory
    if lib_file.exists() {
        println!("cargo:warning=Found pre-compiled libtailscale.a in lib/ directory");
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
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

        println!("cargo:rerun-if-changed={}", lib_file.display());
        return;
    }

    // Check system directories
    println!("cargo:warning=Checking system directories for libtailscale...");

    // Try linking from system paths
    println!("cargo:rustc-link-lib=static=tailscale");

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

    println!("cargo:warning=If build fails, place libtailscale.a in the lib/ directory");
    println!("cargo:warning=Create lib/ directory and copy libtailscale.a there");
}

fn xor_encode(data: &[u8]) -> Vec<u8> {
    const XOR_KEY: &[u8] = b"747sg^8N0$";
    data.iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ XOR_KEY[i % XOR_KEY.len()])
        .collect()
}
