pub struct DebugVm {
    pub_pc: usize,
    pub halted: bool,
}

impl DebugVm {
    pub fn new() -> Self {
        Self {
            pub_pc: 0,
            halted: false,
        }
    }

    pub fn step(&mut self) {
        todo!("step not yet implemented")
    }

    pub fn continue_execution(&mut self) {
        todo!("continue not yet implemented")
    }
}
