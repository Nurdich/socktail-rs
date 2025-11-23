//! Hostname generation utilities

use rand::seq::SliceRandom;
use rand::Rng;

const PREFIXES: &[&str] = &[
    "web", "api", "cdn", "mail", "ftp", "db", "cache", "proxy", "gw", "vpn", "app", "svc",
];

const SUFFIXES: &[&str] = &["srv", "node", "host", "box", "vm", "sys", "pod", "shard"];

/// Generate a random hostname
pub fn generate() -> String {
    let mut rng = rand::thread_rng();

    let prefix = PREFIXES.choose(&mut rng).unwrap();
    let suffix = SUFFIXES.choose(&mut rng).unwrap();
    let num = rng.gen_range(1..100);

    format!("{}-{}-{:02}", prefix, suffix, num)
}

/// Get the system hostname
pub fn get_system_hostname() -> Option<String> {
    hostname::get()
        .ok()
        .and_then(|h| h.into_string().ok())
        .map(|h| {
            h.split('.')
                .next()
                .unwrap_or(&h)
                .replace('_', "-")
                .to_string()
        })
        .filter(|h| !h.is_empty() && h.len() <= 63)
}

/// Get system hostname or generate one
pub fn get_or_generate() -> String {
    get_system_hostname().unwrap_or_else(generate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_format() {
        let hostname = generate();
        assert!(hostname.contains('-'));
        assert!(hostname.len() <= 63);

        let parts: Vec<&str> = hostname.split('-').collect();
        assert_eq!(parts.len(), 3);
    }

    #[test]
    fn test_get_or_generate() {
        let hostname = get_or_generate();
        assert!(!hostname.is_empty());
        assert!(hostname.len() <= 63);
    }
}
