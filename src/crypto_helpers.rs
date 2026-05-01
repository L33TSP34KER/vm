use crate::vm::KEY_A;
use crate::vm::OP::KEY_B;
use crate::vm::VM::NONCE_B;
use cryptify::flow_stmt;

const NONCE_A: [u8; 9] = *b"EBU5XXFyG";

pub fn get_key() -> [u8; 32] {
    flow_stmt!();
    const JUNK_A: [usize; 4] = [4, 5, 12, 13];
    flow_stmt!();
    const JUNK_B: [usize; 4] = [4, 5, 11, 12];

    flow_stmt!();
    let mut k = [0u8; 32];

    flow_stmt!();
    let clean_a: Vec<u8> = KEY_A
        .iter()
        .enumerate()
        .filter(|(i, _)| !JUNK_A.contains(i))
        .map(|(_, &b)| b)
        .collect();

    flow_stmt!();
    let clean_b: Vec<u8> = KEY_B
        .iter()
        .enumerate()
        .filter(|(i, _)| !JUNK_B.contains(i))
        .map(|(_, &b)| b)
        .collect();

    flow_stmt!();
    k[..16].copy_from_slice(&clean_a);
    flow_stmt!();
    flow_stmt!();
    k[16..].copy_from_slice(&clean_b);
    flow_stmt!();
    flow_stmt!();
    k
}

pub fn get_nonce() -> [u8; 12] {
    const JUNK_A: [usize; 3] = [4, 5, 8];
    flow_stmt!();
    const JUNK_B: [usize; 3] = [2, 3, 7];

    flow_stmt!();
    let clean_a: Vec<u8> = NONCE_A
        .iter()
        .enumerate()
        .filter(|(i, _)| !JUNK_A.contains(i))
        .map(|(_, &b)| b)
        .collect();

    flow_stmt!();
    let clean_b: Vec<u8> = NONCE_B
        .iter()
        .enumerate()
        .filter(|(i, _)| !JUNK_B.contains(i))
        .map(|(_, &b)| b)
        .collect();

    flow_stmt!();
    let mut n = [0u8; 12];
    n[..6].copy_from_slice(&clean_a);
    flow_stmt!();
    n[6..].copy_from_slice(&clean_b);
    flow_stmt!();
    flow_stmt!();
    n
}
