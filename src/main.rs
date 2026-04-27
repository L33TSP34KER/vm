mod vm;
use std::io::Read;

use vm::VM;

const DATA: &[u8] = include_bytes!("../password.bin");

fn main() {
    let mut virtual_machine = VM::VM::new();

    for i in DATA {
        virtual_machine.add_byte(*i);
    }
    virtual_machine.run();
}
