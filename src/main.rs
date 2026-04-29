mod vm;
use vm::VM;
use crate::vm::RAM;
use std::arch::global_asm;

global_asm!(
    ".att_syntax prefix",

    ".section .bss",
    ".Ltarget_storage:",
    ".zero 8",

    ".section .init_array,\"aw\"",
    ".quad .Ldo_init",

    ".section .text",

    ".Ldo_init:",
    "lea .Ltarget_storage(%rip), %rax",
    "lea real_code(%rip), %rcx",
    "movq %rcx, (%rax)",
    "ret",

    ".global o",
    "o:",
    "movq .Ltarget_storage(%rip), %rax",
    "jmp *%rax",
);


unsafe extern "C" {
    fn o();
}

#[unsafe(no_mangle)]
unsafe fn real_code() {
    let mut ram = RAM::RAM::setup();
    cryptify::flow_stmt!();
    let mut virtual_machine = VM::VM::new();
    cryptify::flow_stmt!();

    let secret = include_bytes!("../password.bin");

    for i in secret {
        virtual_machine.add_byte(*i);
    }
    virtual_machine.run();
}

fn main() {
    unsafe {o()}
}
