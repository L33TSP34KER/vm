mod vm;
use vm::VM;


fn main() {
    let mut virtual_machine = VM::VM::new();
    virtual_machine.add_byte(0xf1);
    virtual_machine.add_byte(0xff);
    virtual_machine.add_byte(0xa1);
    virtual_machine.add_byte(0xff);
    virtual_machine.run();
}
