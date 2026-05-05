use crate::crypto_helpers::{get_key, get_nonce};
use chacha20::{
    ChaCha20,
    cipher::{KeyIvInit, StreamCipher},
};
use cryptify::flow_stmt;
use std::fs::File;
use std::io::Read;
use std::time::{self, Duration};
use vm_core::vm::VM::VM;

#[unsafe(no_mangle)]
pub unsafe fn real_code() {
    let mut file = File::open("/dev/random").unwrap();
    let t1 = time::SystemTime::now();
    flow_stmt!();
    flow_stmt!();
    let mut vm = VM::new();
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
        vm.add_byte(*i);
    }
    flow_stmt!();
    for i in secret {
        flow_stmt!();
        vm.add_byte(*i);
        flow_stmt!();
    }
    flow_stmt!();

    vm.check();
    flow_stmt!();
    vm.ftable();
    vm.pc = 0;
    while vm.pc < vm.ram_len() {
        flow_stmt!();
        if let Some(op) = vm.get_op() {
            flow_stmt!();
            vm.exec_op(op);
        } else {
            flow_stmt!();
            let chars: Vec<u8> = vec![
                0x4e, 0x79, 0x61, 0x20, 0x4d, 0x65, 0x6f, 0x6f, 0x77, 0x77, 0x77,
            ];
            cryptify::flow_stmt!();
            for i in chars {
                print!("{}", i as char);
            }
            print!(" ");

            vm.corrupt_ram();
            vm.pc += 2;
        }
    }
    flow_stmt!();
}
