use std::usize;

use self::OpCode::*;
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    Push = 0xf1,
    Pop = 0xf2,
    Add = 0xf3,
    Sub = 0xf4,
    Jmp = 0xf5,
    Jz = 0xf6,
    Call = 0xf7,
    Ret = 0xf8,
    Load = 0xf9,
    Store = 0xfa,
    Debug = 0xff,
}

impl OpCode {
    pub fn iterator() -> impl Iterator<Item = OpCode> {
        [Push, Pop, Add, Sub, Jmp, Jz, Call, Ret, Load, Store, Debug]
            .iter()
            .copied()
    }
}

pub fn impl_push(pc: &mut usize, ram: &mut Vec<u8>, stack: &mut Vec<u8>) -> bool {
    let dest = ram.get(*pc + 1).copied();
    if let Some(dest) = dest {
        stack.insert(0, dest);
    }
    *pc+=3;
    false
}

pub fn impl_pop(pc: &mut usize, ram: &mut Vec<u8>, stack: &mut Vec<u8>) -> bool {
    false
}

pub fn impl_add(pc: &mut usize, ram: &mut Vec<u8>, stack: &mut Vec<u8>) -> bool {
    false
}

pub fn impl_sub(pc: &mut usize, ram: &mut Vec<u8>, stack: &mut Vec<u8>) -> bool {
    false
}

pub fn impl_jmp(pc: &mut usize, ram: &mut Vec<u8>, stack: &mut Vec<u8>) -> bool {
    false
}

pub fn impl_jz(pc: &mut usize, ram: &mut Vec<u8>, stack: &mut Vec<u8>) -> bool {
    false
}

pub fn impl_call(pc: &mut usize, ram: &mut Vec<u8>, stack: &mut Vec<u8>) -> bool {
    false
}

pub fn impl_ret(pc: &mut usize, ram: &mut Vec<u8>, stack: &mut Vec<u8>) -> bool {
    false
}

pub fn impl_load(pc: &mut usize, ram: &mut Vec<u8>, stack: &mut Vec<u8>) -> bool {
    false
}

pub fn impl_store(pc: &mut usize, ram: &mut Vec<u8>, stack: &mut Vec<u8>) -> bool {
    let dest = ram.get(*pc + 1).copied();
    let value = stack.first().copied();

    if let (Some(dest), Some(value)) = (dest, value) {
        stack.remove(0);
        ram[dest as usize] = value;
        return true;
    }
    *pc+=3;
    false
}
