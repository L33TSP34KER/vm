use vm_core::vm::OP;

use crate::debugger::Debugger;

impl Debugger {
    fn cmd_next(&mut self, cmds: &Vec<String>) -> Result<(), ()> {
        if let Some(op) = self.vm.get_op() {
            self.vm.exec_op(op);
        }
        Ok(())
    }
    fn cmd_break(&mut self, cmds: &Vec<String>) -> Result<(), ()> {
        if cmds.len() < 2 {
            //TODO: do the pc but im sleepy rn
            println!("usage: break <fn_id>\nor: break *<pc>");
            return Ok(());
        }
        if let Ok(id) = cmds.get(1).unwrap().parse() {
            for i in self.vm.get_ftable().keys() {
                if i == &id {
                    let ppc = self.vm.get_ftable().get(i).unwrap();
                    println!("added breakpoint on {} pc {}", i, ppc);
                    self.breakpoints.push(id);
                }
            }
        }
        Ok(())
    }

    fn cmd_continue(&mut self, cmds: &Vec<String>) -> Result<(), ()> {
        loop {
            if let Some(op) = self.vm.get_op() {
                if op as u8 == OP::OpCode::Call as u8 {
                    let fn_id = self.vm.get_raw(self.vm.pc + 1).unwrap();
                    let mut a: bool = false;
                    for bp in &self.breakpoints {
                        if *bp == fn_id {
                            println!("hit breakpoint {}", fn_id);
                            a = true;
                            break;
                        }
                    }
                    if a {
                        break;
                    }
                }
                self.vm.exec_op(op);
            }
        }
        Ok(())
    }

    fn cmd_exit(&mut self, cmds: &Vec<String>) -> Result<(), ()> {
        Err(())
    }

    fn cmd_disp(&mut self, cmds: &Vec<String>) -> Result<(), ()> {
        Ok(())
    }

    pub fn exec_cmd(&mut self, cmds: &Vec<String>) -> Result<(), ()> {
        match cmds.get(0).unwrap().to_lowercase().as_str() {
            "continue" | "c" => {
                return self.cmd_continue(&cmds);
            }
            "next" | "n" => {
                return self.cmd_next(&cmds);
            }
            "display" | "disp" | "d" => {
                return self.cmd_break(&cmds);
            }
            "break" | "breakpoint" | "b" => {
                return self.cmd_break(&cmds);
            }
            "exit" => {
                return self.cmd_exit(&cmds);
            }
            _ => {}
        }
        Ok(())
    }
}
