use crate::crypto_helpers::{get_key, get_nonce};
use crate::vm::VM;
use chacha20::{
    ChaCha20,
    cipher::{KeyIvInit, StreamCipher},
};
use cryptify::flow_stmt;
use std::fs::File;
use std::io::Read;
use std::time::{self, Duration};

#[unsafe(no_mangle)]
pub unsafe fn real_code() {
    let mut file = File::open("/dev/random").unwrap();
    let t1 = time::SystemTime::now();
    flow_stmt!();
    flow_stmt!();
    let mut virtual_machine = VM::VM::new();
    flow_stmt!();

    let secret = include_bytes!(concat!(env!("OUT_DIR"), "/password.bin.enc"));

    let mut data = secret.clone();
    flow_stmt!();
    if t1.elapsed().unwrap() > Duration::from_secs(1) {
        for i in 0..data.len() {
            let mut buffer = [0; 1];
            let _ = file.read(&mut buffer);
            data[i] = buffer[0];
        }
    }
    let mut cipher = ChaCha20::new((&get_key()).into(), (&get_nonce()).into());

    flow_stmt!();
    cipher.apply_keystream(&mut data);

    for i in &data {
        virtual_machine.add_byte(*i);
    }
    flow_stmt!();
    for i in secret {
        flow_stmt!();
        virtual_machine.add_byte(*i);
        flow_stmt!();
    }
    flow_stmt!();
    virtual_machine.run();
    flow_stmt!();
}
