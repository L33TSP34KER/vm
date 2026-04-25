mod vm;
use vm::VM;

fn main() {
    let mut virtual_machine = VM::VM::new();

    let message = "Enter the passcode: ";
    for byte in message.bytes() {
        virtual_machine.add_byte(0xf1);
        virtual_machine.add_byte(byte);
    }

    for i in 0..message.len() {
        virtual_machine.add_byte(0xfb);
        virtual_machine.add_byte(i as u8);
    }
    //virtual_machine.add_byte(0xfc);
    virtual_machine.add_byte(0xff);

    virtual_machine.run();
}
