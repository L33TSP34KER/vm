use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20::ChaCha20;
use std::env;
use std::fs;

const KEY: &[u8; 32] = b"Q8Fee&W9XdXXW6LBAS6lVAd1og6q#fm1";
const NONCE: &[u8; 12] = b"EBU5Fy0aBv4Z";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=password.bin");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = format!("{}/password.bin.enc", out_dir);

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let bin_path = format!("{}/password.bin", manifest_dir);

    let mut data = fs::read(bin_path).expect("password.bin not found - run compile.sh first");
    let mut cipher = ChaCha20::new(KEY.into(), NONCE.into());
    cipher.apply_keystream(&mut data);
    fs::write(&out_path, &data).expect("failed to write password.bin.enc");
}
