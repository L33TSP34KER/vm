use crate::debugger_cmds;
use std::io::{self, Write};

use vm_core::vm::VM::VM;

pub struct Debugger {
    pub breakpoints: Vec<u8>,
    pub vm: vm_core::vm::VM::VM,
    bytes: Vec<u8>,
    last_commands: Option<Vec<String>>,
}

impl Debugger {
    pub fn new(bytes: Vec<u8>) -> Self {
        Debugger {
            breakpoints: Vec::new(),
            vm: VM::new(),
            bytes,
            last_commands: None,
        }
    }

    fn print_interface(&self) {
        println!("\n");
    }

    fn init(&mut self) {
        for i in &self.bytes {
            self.vm.add_byte(*i);
        }
        self.vm.ftable();
    }

    fn loop_input(&mut self) {
        loop {
            print!("(vmdb) >");
            io::stdout().flush();
            let mut input = Default::default();
            let mut inputs = Default::default();
            let _ = io::stdin().read_line(&mut input);
            inputs = input.trim();
            let cmds_str: Vec<&str> = inputs.split(" ").collect();
            let cmds: Vec<String> = cmds_str.iter().map(|&s| s.to_string()).collect();

            println!("{}", cmds.len());
            if cmds.len() == 1 && cmds.get(0).unwrap().is_empty() {
                match self.last_commands.clone() {
                    Some(a) => {
                        let _ = self.exec_cmd(&a);
                    }
                    None => {
                        continue;
                    }
                }
                continue;
            }
            if self.exec_cmd(&cmds).is_err() {
                break;
            }
            self.last_commands = Some(cmds);
        }
    }

    pub fn run(&mut self) {
        println!("init");
        self.init();
        println!("started");
        self.loop_input();
    }
}
