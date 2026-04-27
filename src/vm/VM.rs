use std::{
    fmt::Debug,
    process::exit,
    time::{self, Instant, SystemTime, UNIX_EPOCH},
};

use crate::vm::OP::{
    OpCode, impl_Meow, impl_Nyaa, impl_add, impl_call, impl_dup, impl_eq, impl_input, impl_jmp,
    impl_jz, impl_load, impl_nay, impl_pop, impl_print, impl_push, impl_ret, impl_store, impl_sub,
};

#[derive(Debug, Clone)]
pub struct VM {
    ram: Vec<u8>,
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
            ram: Vec::new(),
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
        println!("==========DEBUG DUMP===========");
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
        println!("==========STACK DUMP===========");
        for i in 0..self.stack.len() {
            print!("{} ", self.stack.get(i).unwrap());
        }
        println!();
        println!("===============================");
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
        match a {
            Ok(d) => {
                if d.as_millis() > 500 {
                    let chars = vec![
                        0x4e, 0x79, 0x61, 0x20, 0x4d, 0x65, 0x6f, 0x6f, 0x77, 0x77, 0x77,
                    ];
                    for i in chars {
                        print!("{i}");
                    }
                    println!();

                    for i in 0..self.ram.len() {
                        self.ram[i] = (0xEE + (i as u8 % (255 - 0xEE))) ^ self.key;
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
        let mut skip:bool = false;
        match op {
            OpCode::Nyaa => {
                impl_Nyaa(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
            }
            OpCode::Meow => {
                impl_Meow(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
            }
            OpCode::Nay => {
                impl_nay(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
            }
            OpCode::Push => {
                if let Some(last) = self.last_op_time
                    && last.elapsed() > std::time::Duration::from_secs(1)
                {
                    for i in 0..self.ram.len() {
                        self.ram[i] = (0xEE + (i as u8 % (255 - 0xEE))) ^ self.key;
                    }

                }
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
                impl_load(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
            }
            OpCode::Store => {
                impl_store(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
            }
            OpCode::Print => {
                impl_print(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
            }
            OpCode::Input => {
                impl_input(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
                skip = true;
            }
            OpCode::Eq => {
                impl_eq(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
            }
            OpCode::Check => {
                impl_dup(&mut self.pc, &mut self.ram, &mut self.stack, self.key);
            }
            OpCode::Debug => {
                self.pc += 1;
            }
        }
            if let Some(last) = self.last_op_time
                    && last.elapsed() > std::time::Duration::from_secs(1) && !skip
                {
                    for i in 0..self.ram.len() {
                        self.ram[i] = (0xEE + (i as u8 % (255 - 0xEE))) ^ self.key;
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
                let chars: Vec<u8> = vec![
                    0x4e, 0x79, 0x61, 0x20, 0x4d, 0x65, 0x6f, 0x6f, 0x77, 0x77, 0x77,
                ];
                for i in chars {
                    print!("{}", i as char);
                }
                print!(" ");

                for i in 0..self.ram.len() {
                    self.ram[i] = (0xEE + (i as u8 % (255 - 0xEE))) ^ self.key;
                }
                self.pc += 2;
            }
        }
    }
}
