use std::{
    fmt::Debug,
    process::exit,
    time::{self, Instant, SystemTime, UNIX_EPOCH},
};

use cryptify;

use crate::vm::{OP::{
    OpCode, impl_Meow, impl_Nyaa, impl_add, impl_call, impl_dup, impl_eq, impl_input, impl_jmp,
    impl_jz, impl_load, impl_nay, impl_pop, impl_print, impl_push, impl_ret, impl_store, impl_sub,
}, RAM};

#[derive(Debug, Clone)]
pub struct VM {
    ram: RAM::RAM, 
    stack: Vec<u8>,
    pc: usize,
    key: u8,
    last_op_time: Option<Instant>,
    crash_in_10: bool,
    crash_counter: u8,
    time: SystemTime,
}

impl VM {
    pub fn new() -> VM {
        VM {
            ram: RAM::RAM::setup(),
            stack: Vec::new(),
            pc: 0,
            key: (std::time::SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u8),
            last_op_time: None,
            crash_in_10: false,
            crash_counter: 0,
            time: time::SystemTime::now(),
        }
    }

    fn debug(&self) {
        let var = cryptify::encrypt_string!("==========DEBUG DUMP===========");
        let stack_msg = cryptify::encrypt_string!("==========STACK DUMP===========");
        println!("{}", var);
        for i in 0..self.ram.len() {
            if i % 8 == 0 {
                println!();
            }
            if i == self.pc {
                print!("{i}:[{}] ", self.ram.get(i).unwrap() ^ self.key);
            } else {
                print!("{i}:{} ", self.ram.get(i).unwrap() ^ self.key);
            }
        }
        println!();
        println!("{}", stack_msg);
        for i in 0..self.stack.len() {
            print!("{} ", self.stack.get(i).unwrap());
        }
        println!();
    }

    fn get_op(&mut self) -> Option<OpCode> {
        let byte = self.ram.get(self.pc).unwrap() ^ self.key;

        let high = byte >> 4;

        //check if its an op firstly to win compute
        if high != 0xf && high != 0xE {
            return None;
        }

        if byte == 0xff {
            self.debug();
        }

        self.last_op_time = Some(Instant::now());

        OpCode::iterator().find(|&i| i as u8 == byte)
    }

    fn check(&mut self) {
        let a = time::SystemTime::now().duration_since(self.time);
        cryptify::flow_stmt!();
        match a {
            Ok(d) => {
                cryptify::flow_stmt!();
                if d.as_millis() > 500 {
                    let chars = vec![
                        0x4e, 0x79, 0x61, 0x20, 0x4d, 0x65, 0x6f, 0x6f, 0x77, 0x77, 0x77,
                    ];
                    for i in chars {
                        print!("{i}");
                    }
                    println!();

                    for i in 0..self.ram.len() {
                        self.ram.set(i, (0xEE + (i as u8 % (255 - 0xEE))) ^ self.key);
                    }
                }
            }
            Err(_) => {
                let chars = vec![
                    0x4e, 0x79, 0x61, 0x20, 0x4d, 0x65, 0x6f, 0x6f, 0x77, 0x77, 0x77,
                ];
                for i in chars {
                    print!("{i}");
                }
                println!();
            }
        }
    }

    fn exec_op(&mut self, op: OpCode) {
        self.time = time::SystemTime::now();
        let mut skip: bool = false;
        cryptify::flow_stmt!();
        cryptify::flow_stmt!();
        match op {
            OpCode::Nyaa => {
                cryptify::flow_stmt!();
                impl_Nyaa(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
                cryptify::flow_stmt!();
            }
            OpCode::Meow => {
                cryptify::flow_stmt!();
                impl_Meow(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
                cryptify::flow_stmt!();
            }
            OpCode::Nay => {
                cryptify::flow_stmt!();
                impl_nay(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
                cryptify::flow_stmt!();
            }
            OpCode::Push => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                if let Some(last) = self.last_op_time
                    && last.elapsed() > std::time::Duration::from_secs(1)
                {
                    for i in 0..self.ram.len() {
                        self.ram.set(i, (0xEE + (i as u8 % (255 - 0xEE))) ^ self.key);
                    }
                }
                if impl_push(&mut self.pc, &mut self.ram, &mut self.stack, self.key) {
                    cryptify::flow_stmt!();
                    cryptify::flow_stmt!();
                    return;
                }
                cryptify::flow_stmt!();
            }
            OpCode::Pop => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                impl_pop(&mut self.pc, &mut self.ram, &mut self.stack);
                cryptify::flow_stmt!();
            }
            OpCode::Add => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                impl_add(&mut self.pc, &mut self.ram, &mut self.stack);
                cryptify::flow_stmt!();
            }
            OpCode::Sub => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                impl_sub(&mut self.pc, &mut self.ram, &mut self.stack);
                cryptify::flow_stmt!();
            }
            OpCode::Jmp => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                impl_jmp(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
                cryptify::flow_stmt!();
            }
            OpCode::Jz => {
                cryptify::flow_stmt!();
                impl_jz(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
                cryptify::flow_stmt!();
            }
            OpCode::Call => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                impl_call(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
                cryptify::flow_stmt!();
            }
            OpCode::Ret => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                impl_ret(&mut self.pc, &mut self.ram, &mut self.stack);
            }
            OpCode::Load => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                impl_load(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
                cryptify::flow_stmt!();
            }
            OpCode::Store => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                impl_store(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
                cryptify::flow_stmt!();
            }
            OpCode::Print => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                impl_print(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
                cryptify::flow_stmt!();
            }
            OpCode::Input => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                impl_input(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
                skip = true;
            }
            OpCode::Eq => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                impl_eq(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
            }
            OpCode::Check => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                impl_dup(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
            }
            OpCode::Debug => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                self.pc += 1;
            }
        }
        cryptify::flow_stmt!();
        cryptify::flow_stmt!();
        if let Some(last) = self.last_op_time
            && last.elapsed() > std::time::Duration::from_secs(1)
            && !skip
        {
                    for i in 0..self.ram.len() {
                        self.ram.set(i, (0xEE + (i as u8 % (255 - 0xEE))) ^ self.key);
                    }
        }
    }

    pub fn add_byte(&mut self, byte: u8) {
        self.ram.push(byte ^ self.key);
    }

    pub fn run(&mut self) {
        self.check();
        cryptify::flow_stmt!();
        while self.pc < self.ram.len() {
            cryptify::flow_stmt!();
            if let Some(op) = self.get_op() {
                cryptify::flow_stmt!();
                self.exec_op(op);
            } else {
                cryptify::flow_stmt!();
                let chars: Vec<u8> = vec![
                    0x4e, 0x79, 0x61, 0x20, 0x4d, 0x65, 0x6f, 0x6f, 0x77, 0x77, 0x77,
                ];
                cryptify::flow_stmt!();
                for i in chars {
                    print!("{}", i as char);
                }
                print!(" ");

                for i in 0..self.ram.len() {
                    self.ram.set(i, (0xEE + (i as u8 % (255 - 0xEE))) ^ self.key);
                }
                self.pc += 2;
            }
        }
    }
}
