//! Cryptographic utilities

pub mod xor;

pub use xor::{deobfuscate_authkey, get_default_authkey, xor_decode, xor_encode};
