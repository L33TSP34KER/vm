use std::{fmt::Debug, process::exit, time::UNIX_EPOCH};

use crate::vm::OP::{
    impl_add, impl_call, impl_input, impl_jmp, impl_jz, impl_load, impl_pop, impl_print, impl_push,
    impl_ret, impl_store, impl_sub, OpCode,
};

#[derive(Debug, Clone)]
pub struct VM {
    ram: Vec<u8>,
    stack: Vec<u8>,
    pc: usize,
    key: u8,
}

impl VM {
    pub fn new() -> VM {
        VM {
            ram: Vec::new(),
            stack: Vec::new(),
            pc: 0,
            key: (std::time::SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u8),
        }
    }

    fn debug(&self) {
        println!("==========DEBUG DUMP===========");
        for i in 0..self.ram.len() {
            if i % 8 == 0 {
                println!();
            }
            if i == self.pc {
                print!("{i}:[{}] ", self.ram.get(i).unwrap());
            } else {
                print!("{i}:{} ", self.ram.get(i).unwrap());
            }
        }
        println!();
        println!("==========STACK DUMP===========");
        for i in 0..self.stack.len() {
            print!("{} ", self.stack.get(i).unwrap());
        }
        println!();
        println!("===============================");
    }

    fn get_op(&self) -> Option<OpCode> {
        let byte = self.ram.get(self.pc).unwrap() ^ self.key;
        let high = byte >> 4;

        //check if its an op firstly to win compute
        if high != 0xf {
            return None;
        }

        if byte == 0xff {
            self.debug();
        }

        OpCode::iterator().find(|&i| i as u8 == byte)
    }

    fn exec_op(&mut self, op: OpCode) {
        match op {
            OpCode::Push => {
                if impl_push(&mut self.pc, &mut self.ram, &mut self.stack, self.key) {
                    return;
                }
            }
            OpCode::Pop => {
                impl_pop(&mut self.pc, &mut self.ram, &mut self.stack);
            }
            OpCode::Add => {
                impl_add(&mut self.pc, &mut self.ram, &mut self.stack);
            }
            OpCode::Sub => {
                impl_sub(&mut self.pc, &mut self.ram, &mut self.stack);
            }
            OpCode::Jmp => {
                impl_jmp(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
            }
            OpCode::Jz => {
                impl_jz(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
            }
            OpCode::Call => {
                impl_call(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
            }
            OpCode::Ret => {
                impl_ret(&mut self.pc, &mut self.ram, &mut self.stack);
            }
            OpCode::Load => {
                impl_load(&mut self.pc, &mut self.ram, &mut self.stack);
            }
            OpCode::Store => {
                impl_store(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
            }
            OpCode::Print => {
                impl_print(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
            }
            OpCode::Input => {
                impl_input(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
            }
            OpCode::Debug => {
                self.pc += 1;
            }
        }
    }

    pub fn add_byte(&mut self, byte: u8) {
        self.ram.push(byte ^ self.key);
    }

    pub fn run(&mut self) {
        self.ram.reserve(4096);
        while self.pc < self.ram.len() {
            if let Some(op) = self.get_op() {
                self.exec_op(op);
            } else {
                self.pc += 1;
            }
        }
    }
}
