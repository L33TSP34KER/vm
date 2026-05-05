use std::{env::args, fs};

use crate::debugger::Debugger;
mod debugger;
mod debugger_cmds;

fn main() {
    if args().len() < 2 {
        println!("usage:\n{} <file_path>", args().nth(0).unwrap());
        return;
    }
    let file_path = fs::read(args().nth(1).unwrap());
    match file_path {
        Ok(e) => {
            Debugger::new(e).run();
        }
        Err(e) => {
            println!("err: {}", e);
        }
    }
}
