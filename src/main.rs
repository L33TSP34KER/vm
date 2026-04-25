mod vm;
use vm::VM;

fn main() {
    let mut virtual_machine = VM::VM::new();

    virtual_machine.add_byte(0xfc);
    virtual_machine.add_byte(0xff);

    for _ in 0..250 {
        virtual_machine.add_byte(0xf1);
        virtual_machine.add_byte(0);
    }

    virtual_machine.add_byte(0xff);
    virtual_machine.add_byte(0xff);

    virtual_machine.run();
}
