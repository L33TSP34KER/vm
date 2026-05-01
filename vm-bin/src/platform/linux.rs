use cryptify::flow_stmt;
use libc::{close, getpid, kill, signal};
use std::arch::{asm, global_asm};
use std::fs::File;
use std::io::{Read, Seek};
use std::ops::Div;
use std::os::fd::AsRawFd;

static mut INITIAL_RSP: usize = 0;

#[unsafe(link_section = ".init_array")]
#[used]
static CAPTURE_RSP: fn() = capture_initial_rsp;

unsafe extern "C" {
    static _start: u8;
}

fn capture_initial_rsp() {
    unsafe {
        asm!("mov {}, rsp", out(reg) INITIAL_RSP);
    }
}

const SIGEMT: libc::c_int = 3;

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
    options(att_syntax)
);

unsafe extern "C" {
    fn o();
}

extern "C" fn entry9(_sig: i32) {
    unsafe { o() }
}

pub fn main() {
    let mut file = File::open("/dev/random").unwrap();

    unsafe {
        use libc::{SIGABRT, SIGQUIT};
        flow_stmt!();
        signal(SIGEMT, entry9 as *const () as usize);
        flow_stmt!();
    }

    let mut a: u8 = 0;
    flow_stmt!();
    let j: usize = 3;
    flow_stmt!();
    if a.div(j as u8) % 9 == 0 {
        flow_stmt!();
    }
    flow_stmt!();
    flow_stmt!();
    flow_stmt!();
    loop {
        match a {
            127 => unsafe {
                flow_stmt!();
                unsafe {
                    kill(getpid(), 3);
                }
            },
            0x28 => {
                flow_stmt!();
            }
            0x27 => {
                unsafe {}
                flow_stmt!();
            }
            0x26 => {
                unsafe {}
                flow_stmt!();
            }
            0x25 => {
                flow_stmt!();
            }
            0x24 => {
                flow_stmt!();
            }
            0x23 => {
                flow_stmt!();
            }
            0x22 => {
                flow_stmt!();
            }
            0x21 => {
                flow_stmt!();
                unsafe {}
            }
            _ => {}
        }
        let mut buffer = [0; 1];
        let _ = file.read(&mut buffer);
        file.seek_relative(2);
        a = buffer[0];
    }
}
