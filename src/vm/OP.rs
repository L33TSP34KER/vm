use std::{io::Read, usize};

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
    Print = 0xfb,
    Input = 0xfc,
    Debug = 0xff,
}

impl OpCode {
    pub fn iterator() -> impl Iterator<Item = OpCode> {
        [
            Push, Pop, Add, Sub, Jmp, Jz, Call, Ret, Load, Store, Debug, Print, Input,
        ]
        .iter()
        .copied()
    }
}

pub fn impl_print(pc: &mut usize, ram: &mut Vec<u8>, stack: &mut Vec<u8>, key: u8) -> bool {
    let dest = ram.get(*pc + 1).copied().map(|b| b ^ key);
    let char = stack.get(dest.unwrap() as usize).copied();
    print!("{}", char.unwrap() as char);
    *pc += 2;
    false
}

pub fn impl_push(pc: &mut usize, ram: &mut Vec<u8>, stack: &mut Vec<u8>, key: u8) -> bool {
    let dest = ram.get(*pc + 1).copied().map(|b| b ^ key);
    if let Some(dest) = dest {
        stack.push(dest);
    }
    *pc += 1;
    false
}

pub fn impl_pop(pc: &mut usize, ram: &mut Vec<u8>, stack: &mut Vec<u8>) -> bool {
    if stack.pop().is_some() {
        *pc += 1;
        return true;
    }
    false
}

pub fn impl_add(pc: &mut usize, ram: &mut Vec<u8>, stack: &mut Vec<u8>) -> bool {
    if stack.len() >= 2 {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(b.wrapping_add(a));
        *pc += 1;
        return true;
    }
    false
}

pub fn impl_sub(pc: &mut usize, ram: &mut Vec<u8>, stack: &mut Vec<u8>) -> bool {
    if stack.len() >= 2 {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(b.wrapping_sub(a));
        *pc += 1;
        return true;
    }
    false
}

pub fn impl_jmp(pc: &mut usize, ram: &mut Vec<u8>, stack: &mut Vec<u8>, key: u8) -> bool {
    let dest = ram.get(*pc + 1).copied().map(|b| b ^ key);
    if let Some(dest) = dest {
        *pc = dest as usize;
        return true;
    }
    false
}

pub fn impl_jz(pc: &mut usize, ram: &mut Vec<u8>, stack: &mut Vec<u8>, key: u8) -> bool {
    let val = stack.pop().unwrap();
    let dest = ram.get(*pc + 1).copied().map(|b| b ^ key);
    if val == 0 {
        if let Some(dest) = dest {
            *pc = dest as usize;
            return true;
        }
    } else {
        *pc += 2;
    }
    false
}

pub fn impl_call(pc: &mut usize, ram: &mut Vec<u8>, stack: &mut Vec<u8>, key: u8) -> bool {
    let dest = ram.get(*pc + 1).copied().map(|b| b ^ key);
    if let Some(dest) = dest {
        stack.push((*pc + 2) as u8);
        *pc = dest as usize;
        return true;
    }
    false
}

pub fn impl_ret(pc: &mut usize, ram: &mut Vec<u8>, stack: &mut Vec<u8>) -> bool {
    if let Some(addr) = stack.pop() {
        *pc = addr as usize;
        return true;
    }
    false
}

pub fn impl_load(pc: &mut usize, ram: &mut Vec<u8>, stack: &mut Vec<u8>) -> bool {
    let addr = stack.pop().unwrap();
    if let Some(val) = ram.get(addr as usize).copied() {
        stack.push(val);
        *pc += 1;
        return true;
    }
    false
}

pub fn impl_store(pc: &mut usize, ram: &mut Vec<u8>, stack: &mut Vec<u8>, key: u8) -> bool {
    let dest = ram.get(*pc + 1).copied().map(|b| b ^ key);
    let value = stack.first().copied();

    if let (Some(dest), Some(value)) = (dest, value) {
        stack.remove(0);
        ram[dest as usize] = value;
        return true;
    }
    *pc += 3;
    false
}

pub fn impl_input(pc: &mut usize, _ram: &mut Vec<u8>, stack: &mut Vec<u8>, _key: u8) -> bool {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    for byte in input.bytes() {
        stack.push(byte);
    }
    *pc += 1;
    true
}
