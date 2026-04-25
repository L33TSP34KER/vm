use std::process::exit;

use crate::vm::OP::{OpCode, impl_push};

#[derive(Debug, Clone)]
pub struct VM {
    ram: Vec<u8>,
    stack: Vec<u8>,
    pc: usize
}

impl VM {
    pub fn new() -> VM {
        VM {
            ram: Vec::new(),
            stack: Vec::new(),
            pc: 0
        }
    }

    fn debug(&self) {
        println!("==========DEBUG DUMP===========");
        for i in 0..self.ram.len() {
            if i % 16 == 0 {
                println!();
            }
            if i == self.pc {
                print!("-> {} <- ", self.ram.get(i).unwrap());
            } else {
                print!("{} ", self.ram.get(i).unwrap());
            }
        }
        println!();
        println!("===============================");
    }

    fn get_op(&self) -> Option<OpCode> {
        let byte = self.ram.get(self.pc).unwrap();
        let high = byte >> 4;

        //check if its an op firstly to win compute
        if high != 0xf {
            return None;
        }

        if *byte == 0xff {
            self.debug();
        }

        OpCode::iterator().find(|&i| i as u8 == *byte)
    }

    fn exec_op(&mut self, op: OpCode) {
        match op {
            OpCode::Push => {
                if impl_push(&mut self.pc, &mut self.ram, &mut self.stack) {
                    return;
                }
            }
            _ => {
                exit(999);
            }
        }
    }

    pub fn add_byte(&mut self, byte: u8) {
        self.ram.push(byte);
    }

    pub fn run(&mut self) {
        self.ram.reserve(4096);
        for pc in 0..self.ram.len() {
            self.pc = pc;
            if let Some(op) = self.get_op() {
                self.exec_op(op);
            }
        }
    }
}
