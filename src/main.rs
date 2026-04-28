mod vm;

use hide_macro::decrypt;
use vm::VM;

use crate::vm::RAM;

fn main() {
    let mut ram = RAM::RAM::setup();
    cryptify::flow_stmt!();
    let mut virtual_machine = VM::VM::new();
    cryptify::flow_stmt!();

    let secret = decrypt!(include_bytes!("../password.bin"));

    for i in secret {
        ram.add_byte(*i);
    }

    for i in 0..secret.len() {
        println!("{}: {}", i, ram.get_value(i));
    }
    ram.debug();
    //virtual_machine.run();
}
